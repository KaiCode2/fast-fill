// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Ownable} from "solady/auth/Ownable.sol";
import {ReentrancyGuard} from "solady/utils/ReentrancyGuard.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {IFastFill, FillStatus, OrderRecord} from "./interfaces/IFastFill.sol";
import {Order, OrderLib} from "./libraries/OrderLib.sol";
import {PricingLib} from "./libraries/PricingLib.sol";
import {AddressCast} from "./libraries/AddressCast.sol";

/// @title  FastFillBase
/// @notice Shared lifecycle for an optimistic cross-chain fill protocol that wraps message-based
///         bridges. The bridge-specific *initiate* (source) and *settle* (destination) glue lives
///         in adapters (CctpAdapter, OftAdapter) that inherit this base.
///
///         The whole design hinges on one value: `orderId = keccak256(abi.encode(order))`. It is
///         computed identically at source-encode, at fill, and at settle. Because the order data
///         settles through the bridge's *authenticated* channel, a relayer that fills against a
///         fabricated order computes an orderId no settling message will reproduce, and is simply
///         never reimbursed. Fills are therefore trustless: a bad filler can only lose its own
///         funds. The destination contract's token balance is the reimbursement pool — there is no
///         separate escrow, and every order settles exactly once.
abstract contract FastFillBase is IFastFill, Ownable, ReentrancyGuard {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    // ---------------------------------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------------------------------
    error Paused();
    error WrongBridgeType(uint8 expected, uint8 got);
    error WrongDestinationChain(uint32 expected);
    error OrderAlreadyActive(bytes32 orderId);
    error AlreadySettled(bytes32 orderId);
    error FillerNotAllowed(address filler);
    error NothingToClaim();
    error InvalidMaxFeeRate(uint256 maxFeeRate);
    error InvalidWindow(uint64 startTime, uint64 expectedDeliveryTime);
    error InvalidOutputAmount(uint256 outputAmount, uint256 inputAmount);
    error UnknownRemoteAdapter(uint32 chainId);
    error NotSourceChain(uint32 srcChainId);
    error ZeroRecipient();

    // ---------------------------------------------------------------------------------------------
    // Storage
    // ---------------------------------------------------------------------------------------------

    /// @notice Destination-chain record for each order, keyed by orderId.
    mapping(bytes32 orderId => OrderRecord) internal _orders;

    /// @notice Funds that failed to push (e.g. reverting/blacklisted recipient), claimable later.
    mapping(address account => mapping(address token => uint256)) internal _claimable;

    /// @notice The fast-fill adapter address on each remote chain (bytes32 to allow non-EVM later).
    ///         Used as the CCTP mintRecipient/destinationCaller, the OFT `to`, and the OFT peer.
    mapping(uint32 chainId => bytes32 adapter) public remoteAdapter;

    /// @notice Allowlisted fillers (only consulted when `fillAllowlistEnabled`).
    mapping(address filler => bool) public allowedFiller;

    /// @notice Per-adapter governance cap on the fee rate, WAD (<= 1e18).
    uint256 public maxFeeRate;

    /// @notice Monotonic counter assigning each source-side order a unique nonce.
    uint64 internal _nonceCounter;

    /// @notice When true, initiate/fill/settle are blocked (claim is never blocked).
    bool public paused;

    /// @notice When true, only allowlisted addresses may call `fill` (default false).
    bool public fillAllowlistEnabled;

    // ---------------------------------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------------------------------
    constructor(address owner_, uint256 maxFeeRate_) {
        if (maxFeeRate_ > PricingLib.WAD) revert InvalidMaxFeeRate(maxFeeRate_);
        _initializeOwner(owner_);
        maxFeeRate = maxFeeRate_;
    }

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    // ---------------------------------------------------------------------------------------------
    // Hooks implemented by each adapter
    // ---------------------------------------------------------------------------------------------

    /// @dev The bridge family this adapter handles (OrderLib.BRIDGE_CCTP / BRIDGE_OFT).
    function _bridgeType() internal pure virtual returns (uint8);

    /// @dev Validate that `order.outputToken` is the token this adapter settles in, and return
    ///      its ERC20 address. Reverts on mismatch. Used by both `fill` and `_settle`.
    function _resolveOutputToken(Order memory order) internal view virtual returns (address token);

    // ---------------------------------------------------------------------------------------------
    // Optimistic fill (destination, before the bridge message arrives)
    // ---------------------------------------------------------------------------------------------

    /// @inheritdoc IFastFill
    function fill(Order calldata order) external nonReentrant whenNotPaused returns (bytes32 orderId) {
        if (order.bridgeType != _bridgeType()) revert WrongBridgeType(_bridgeType(), order.bridgeType);
        if (block.chainid != order.dstChainId) revert WrongDestinationChain(order.dstChainId);
        if (fillAllowlistEnabled && !allowedFiller[msg.sender]) revert FillerNotAllowed(msg.sender);

        orderId = order.hash();
        OrderRecord storage rec = _orders[orderId];
        if (rec.status != FillStatus.None) revert OrderAlreadyActive(orderId);

        address token = _resolveOutputToken(order);
        address recipient = order.recipient.toAddress();

        uint256 feeToFiller = PricingLib.fee(
            order.outputAmount,
            order.startTime,
            order.expectedDeliveryTime,
            block.timestamp,
            order.discountRate,
            maxFeeRate
        );
        uint256 payoutToRecipient = order.outputAmount - feeToFiller;

        // Effects before interactions.
        rec.filler = msg.sender;
        rec.status = FillStatus.Filled;
        rec.fillTime = uint40(block.timestamp);

        // Pull the payout from the relayer into the contract, then forward to the recipient with a
        // pull-payment fallback so a temporarily-reverting recipient can't brick the fill.
        SafeTransferLib.safeTransferFrom(token, msg.sender, address(this), payoutToRecipient);
        _payout(orderId, token, recipient, payoutToRecipient);

        emit OrderFilled(orderId, msg.sender, payoutToRecipient, feeToFiller, uint40(block.timestamp));
    }

    // ---------------------------------------------------------------------------------------------
    // Settlement (destination, when the bridge message arrives) — called by adapters
    // ---------------------------------------------------------------------------------------------

    /// @dev Disburse an arrived bridge transfer. The adapter MUST have authenticated `order` and
    ///      ensured `arrived` tokens are now held by this contract before calling.
    function _settle(Order memory order, bytes32 orderId, uint256 arrived) internal {
        OrderRecord storage rec = _orders[orderId];
        if (rec.status == FillStatus.Settled) revert AlreadySettled(orderId);

        address token = _resolveOutputToken(order);
        address recipient = order.recipient.toAddress();
        address filler = rec.filler;

        uint256 owed = arrived < order.outputAmount ? arrived : order.outputAmount;
        uint256 surplus = arrived - owed;

        // Effects before interactions.
        rec.status = FillStatus.Settled;

        if (filler != address(0)) {
            // Order was optimistically filled: reimburse the filler exactly `owed`; surplus -> recipient.
            _payout(orderId, token, filler, owed);
            if (surplus != 0) _payout(orderId, token, recipient, surplus);
        } else {
            // Never filled: the recipient receives everything that arrived.
            _payout(orderId, token, recipient, arrived);
        }

        emit OrderSettled(orderId, filler, arrived, surplus);
    }

    // ---------------------------------------------------------------------------------------------
    // Payout & claim (pull-payment fallback)
    // ---------------------------------------------------------------------------------------------

    /// @dev Try to push `amount` of `token` to `to`; on failure credit the claim ledger instead.
    function _payout(bytes32 orderId, address token, address to, uint256 amount) internal {
        if (amount == 0) return;
        if (_tryTransfer(token, to, amount)) return;
        _claimable[to][token] += amount;
        emit PayoutDeferred(orderId, to, token, amount);
    }

    /// @dev A return-value-checked ERC20 transfer that never reverts (returns success instead).
    function _tryTransfer(address token, address to, uint256 amount) private returns (bool) {
        (bool ok, bytes memory ret) = token.call(abi.encodeWithSelector(0xa9059cbb, to, amount)); // transfer(address,uint256)
        return ok && (ret.length == 0 || abi.decode(ret, (bool)));
    }

    /// @inheritdoc IFastFill
    function claim(address token) external nonReentrant returns (uint256 amount) {
        amount = _claimable[msg.sender][token];
        if (amount == 0) revert NothingToClaim();
        _claimable[msg.sender][token] = 0;
        SafeTransferLib.safeTransfer(token, msg.sender, amount);
        emit Claimed(msg.sender, token, amount);
    }

    // ---------------------------------------------------------------------------------------------
    // Source-side helpers for adapters
    // ---------------------------------------------------------------------------------------------

    /// @dev Allocate the next unique nonce for an outbound order.
    function _nextNonce() internal returns (uint64 nonce) {
        nonce = _nonceCounter;
        _nonceCounter = nonce + 1;
    }

    /// @dev Common create-time validation. Adapters add bridge-specific checks (token, domain/eid).
    function _assertCreatable(Order memory order) internal view {
        if (order.srcChainId != block.chainid) revert NotSourceChain(order.srcChainId);
        if (remoteAdapter[order.dstChainId] == bytes32(0)) revert UnknownRemoteAdapter(order.dstChainId);
        if (order.expectedDeliveryTime <= order.startTime) {
            revert InvalidWindow(order.startTime, order.expectedDeliveryTime);
        }
        if (order.outputAmount == 0 || order.outputAmount > order.inputAmount) {
            revert InvalidOutputAmount(order.outputAmount, order.inputAmount);
        }
        if (order.recipient == bytes32(0)) revert ZeroRecipient();
    }

    // ---------------------------------------------------------------------------------------------
    // Views
    // ---------------------------------------------------------------------------------------------

    /// @inheritdoc IFastFill
    function quoteFill(Order calldata order, uint256 fillTime)
        external
        view
        returns (uint256 payoutToRecipient, uint256 feeToFiller)
    {
        feeToFiller = PricingLib.fee(
            order.outputAmount, order.startTime, order.expectedDeliveryTime, fillTime, order.discountRate, maxFeeRate
        );
        payoutToRecipient = order.outputAmount - feeToFiller;
    }

    /// @inheritdoc IFastFill
    function getOrder(bytes32 orderId) external view returns (OrderRecord memory) {
        return _orders[orderId];
    }

    /// @notice Funds owed to `account` in `token` from a deferred payout.
    function claimable(address account, address token) external view returns (uint256) {
        return _claimable[account][token];
    }

    // ---------------------------------------------------------------------------------------------
    // Admin
    // ---------------------------------------------------------------------------------------------

    function setRemoteAdapter(uint32 chainId, bytes32 adapter) external onlyOwner {
        remoteAdapter[chainId] = adapter;
    }

    function setMaxFeeRate(uint256 newMaxFeeRate) external onlyOwner {
        if (newMaxFeeRate > PricingLib.WAD) revert InvalidMaxFeeRate(newMaxFeeRate);
        maxFeeRate = newMaxFeeRate;
    }

    function setPaused(bool newPaused) external onlyOwner {
        paused = newPaused;
    }

    function setFillAllowlistEnabled(bool enabled) external onlyOwner {
        fillAllowlistEnabled = enabled;
    }

    function setAllowedFiller(address filler, bool allowed) external onlyOwner {
        allowedFiller[filler] = allowed;
    }
}
