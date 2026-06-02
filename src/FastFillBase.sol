// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Ownable} from "solady/auth/Ownable.sol";
import {ReentrancyGuard} from "solady/utils/ReentrancyGuard.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";

import {IFastFill, FillStatus, OrderRecord} from "./interfaces/IFastFill.sol";
import {IERC20Permit} from "./interfaces/IERC20Permit.sol";
import {ISignatureTransfer} from "./interfaces/permit2/ISignatureTransfer.sol";
import {Order, OrderLib} from "./libraries/OrderLib.sol";
import {PricingLib} from "./libraries/PricingLib.sol";
import {PermitLib} from "./libraries/PermitLib.sol";
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
abstract contract FastFillBase is IFastFill, Ownable, ReentrancyGuard, Multicallable {
    using OrderLib for Order;
    using AddressCast for bytes32;
    using AddressCast for address;

    /// @notice Uniswap Permit2 — same address on every chain — used for signature-based pulls where
    ///         the funds come from a signer that is not `msg.sender` (sponsored / intent flows).
    address public constant PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3;

    // ---------------------------------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------------------------------
    error Paused();
    error WrongBridgeType(uint8 expected, uint8 got);
    error WrongDestinationChain(uint32 expected);
    error OrderAlreadyActive(bytes32 orderId);
    error AlreadySettled(bytes32 orderId);
    error NothingToClaim();
    error InvalidMaxFeeRate(uint256 maxFeeRate);
    error InvalidWindow(uint64 startTime, uint64 expectedDeliveryTime);
    error InvalidOutputAmount(uint256 outputAmount, uint256 inputAmount);
    error InvalidBaseFee(uint256 baseFee, uint256 outputAmount);
    error UnsupportedChain(uint32 chainId);
    error NotSourceChain(uint32 srcChainId);
    error ZeroRecipient();

    // ---------------------------------------------------------------------------------------------
    // Storage
    // ---------------------------------------------------------------------------------------------

    /// @notice Destination-chain record for each order, keyed by orderId.
    mapping(bytes32 orderId => OrderRecord) internal _orders;

    /// @notice Funds that failed to push (e.g. reverting/blacklisted recipient), claimable later.
    mapping(address account => mapping(address token => uint256)) internal _claimable;

    /// @notice Per-adapter governance cap on the fee rate, WAD (<= 1e18).
    uint256 public maxFeeRate;

    /// @notice Monotonic counter assigning each source-side order a unique nonce.
    uint64 internal _nonceCounter;

    /// @notice When true, initiate/fill/settle are blocked (claim is never blocked).
    bool public paused;

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

    /// @dev Revert unless `chainId` is a supported counterpart for this bridge (per the config
    ///      registry). This is the "does the remote chain exist" check that replaces the old
    ///      remote-adapter registry now that the counterpart is always `address(this)`.
    function _requireSupportedRemote(uint32 chainId) internal view virtual;

    // ---------------------------------------------------------------------------------------------
    // Optimistic fill (destination, before the bridge message arrives)
    // ---------------------------------------------------------------------------------------------

    /// @inheritdoc IFastFill
    /// @dev `msg.sender` is the filler and the payer; it must have approved this adapter for the
    ///      payout (or batch a `selfPermit` via `multicall` for a single transaction).
    function fill(Order calldata order) external nonReentrant whenNotPaused returns (bytes32 orderId) {
        address token;
        address recipient;
        uint256 payout;
        uint256 fee;
        (orderId, token, recipient, payout, fee) = _prepareFill(order, msg.sender);
        SafeTransferLib.safeTransferFrom(token, msg.sender, address(this), payout);
        _payout(orderId, token, recipient, payout);
        emit OrderFilled(orderId, msg.sender, payout, fee, uint40(block.timestamp));
    }

    /// @notice Fill `order` on behalf of `filler`, pulling the payout from `filler` via a Permit2
    ///         signature. Lets a third party submit the fill while the funds and the recorded filler
    ///         are `filler` — the filler signs once, off-chain, and need not send the transaction.
    /// @dev The Permit2 signature commits to `fillWitness(orderId)`, so a submitter can only fill the
    ///      exact order the filler authorized; the signed `permitted.amount` must be `order.outputAmount`
    ///      (the worst-case payout), of which only the actual `payout` is pulled.
    function fillFor(Order calldata order, address filler, PermitLib.Permit2Data calldata permit)
        external
        nonReentrant
        whenNotPaused
        returns (bytes32 orderId)
    {
        address token;
        address recipient;
        uint256 payout;
        uint256 fee;
        (orderId, token, recipient, payout, fee) = _prepareFill(order, filler);
        // The filler signed `permitted.amount == order.outputAmount` (worst-case) for this orderId;
        // pull only the actual payout.
        _pullFillViaPermit2(orderId, token, filler, order.outputAmount, payout, permit);
        _payout(orderId, token, recipient, payout);
        emit OrderFilled(orderId, filler, payout, fee, uint40(block.timestamp));
    }

    /// @dev Shared fill bookkeeping: validate, price, and record `filler` as the filler. Pulling the
    ///      payout and forwarding it to the recipient is left to the caller (it differs by funding
    ///      source). Effects are written before the caller's interactions (checks-effects-interactions).
    function _prepareFill(Order calldata order, address filler)
        internal
        returns (bytes32 orderId, address token, address recipient, uint256 payout, uint256 fee)
    {
        if (order.bridgeType != _bridgeType()) revert WrongBridgeType(_bridgeType(), order.bridgeType);
        if (block.chainid != order.dstChainId) revert WrongDestinationChain(order.dstChainId);

        orderId = order.hash();
        OrderRecord storage rec = _orders[orderId];
        if (rec.status != FillStatus.None) revert OrderAlreadyActive(orderId);

        token = _resolveOutputToken(order);
        recipient = order.recipient.toAddress();
        fee = PricingLib.fee(
            order.outputAmount,
            order.startTime,
            order.expectedDeliveryTime,
            block.timestamp,
            order.discountRate,
            maxFeeRate,
            order.baseFee
        );
        payout = order.outputAmount - fee;

        rec.filler = filler;
        rec.status = FillStatus.Filled;
        rec.fillTime = uint40(block.timestamp);
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
    // Gasless approvals (ERC-2612 self-permit + Permit2 signature pulls)
    // ---------------------------------------------------------------------------------------------

    /// @notice Apply an EIP-2612 permit for `token` from `msg.sender` to this adapter, so an approval
    ///         and an action can land in one transaction: `multicall([selfPermit(...), fill/initiate(...)])`
    ///         (`multicall` preserves `msg.sender`). Best-effort — a front-run that already set the
    ///         allowance does not brick the batch; an insufficient allowance still reverts the action.
    function selfPermit(address token, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external {
        try IERC20Permit(token).permit(msg.sender, address(this), value, deadline, v, r, s) {} catch {}
    }

    /// @dev Pull `order.inputAmount` of the order's input token from `from` via Permit2, binding the
    ///      signature to the order intent so a sponsor cannot alter it. Used by `initiate*For`.
    /// @param bridgeParams A per-adapter hash of the transport mode `from` opted into (so a relayer
    ///        cannot downgrade fast/slow or executor settings). The timing is bound as the relative
    ///        `deliveryWindow` the signer agreed to, recovered exactly from the just-built order
    ///        (`startTime == block.timestamp` at build), not as a submit-dependent absolute time.
    function _pullOrderViaPermit2(
        Order memory order,
        address from,
        PermitLib.Permit2Data calldata permit,
        bytes32 bridgeParams
    ) internal {
        bytes32 witness = PermitLib.orderWitness(
            order.bridgeType,
            order.dstChainId,
            order.recipient,
            order.inputAmount,
            order.outputAmount,
            order.expectedDeliveryTime - order.startTime,
            order.discountRate,
            order.baseFee,
            bridgeParams
        );
        _pullViaPermit2(
            order.inputToken.toAddress(),
            from,
            order.inputAmount,
            order.inputAmount,
            permit.nonce,
            permit.deadline,
            witness,
            PermitLib.ORDER_WITNESS_TYPE_STRING,
            permit.signature
        );
    }

    /// @dev Pull `requestedAmount` from `filler` via Permit2, binding the signature to `orderId`.
    function _pullFillViaPermit2(
        bytes32 orderId,
        address token,
        address filler,
        uint256 maxAmount,
        uint256 requestedAmount,
        PermitLib.Permit2Data calldata permit
    ) internal {
        _pullViaPermit2(
            token,
            filler,
            maxAmount,
            requestedAmount,
            permit.nonce,
            permit.deadline,
            PermitLib.fillWitness(orderId),
            PermitLib.FILL_WITNESS_TYPE_STRING,
            permit.signature
        );
    }

    /// @dev Pull `requestedAmount` of `token` from `owner` to this adapter via Permit2, against a
    ///      signature that also commits to `witness` (the order intent / fill authorization). This is
    ///      how a sponsor funds an action from a signer that is not `msg.sender`.
    function _pullViaPermit2(
        address token,
        address owner,
        uint256 maxAmount,
        uint256 requestedAmount,
        uint256 nonce,
        uint256 deadline,
        bytes32 witness,
        string memory witnessTypeString,
        bytes calldata signature
    ) internal {
        ISignatureTransfer(PERMIT2)
            .permitWitnessTransferFrom(
                ISignatureTransfer.PermitTransferFrom({
                permitted: ISignatureTransfer.TokenPermissions({token: token, amount: maxAmount}),
                nonce: nonce,
                deadline: deadline
            }),
                ISignatureTransfer.SignatureTransferDetails({to: address(this), requestedAmount: requestedAmount}),
                owner,
                witness,
                witnessTypeString,
                signature
            );
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
        _requireSupportedRemote(order.dstChainId);
        if (order.expectedDeliveryTime <= order.startTime) {
            revert InvalidWindow(order.startTime, order.expectedDeliveryTime);
        }
        if (order.outputAmount == 0 || order.outputAmount > order.inputAmount) {
            revert InvalidOutputAmount(order.outputAmount, order.inputAmount);
        }
        // The flat baseFee must leave the recipient something at fill; the combined fee is additionally
        // capped at outputAmount inside PricingLib.
        if (order.baseFee >= order.outputAmount) revert InvalidBaseFee(order.baseFee, order.outputAmount);
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
            order.outputAmount,
            order.startTime,
            order.expectedDeliveryTime,
            fillTime,
            order.discountRate,
            maxFeeRate,
            order.baseFee
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

    function setMaxFeeRate(uint256 newMaxFeeRate) external onlyOwner {
        if (newMaxFeeRate > PricingLib.WAD) revert InvalidMaxFeeRate(newMaxFeeRate);
        maxFeeRate = newMaxFeeRate;
    }

    function setPaused(bool newPaused) external onlyOwner {
        paused = newPaused;
    }
}
