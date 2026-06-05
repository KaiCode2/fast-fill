// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Ownable} from "solady/auth/Ownable.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {CallbackExecutor} from "./CallbackExecutor.sol";
import {BurnMessageV2Lib} from "./libraries/BurnMessageV2Lib.sol";
import {AddressCast} from "./libraries/AddressCast.sol";
import {ExecHook, ExecHookLib} from "./libraries/ExecHookLib.sol";
import {ICctpExecReceiver} from "./interfaces/ICctpExecReceiver.sol";
import {ITokenMessengerV2} from "./interfaces/cctp/ITokenMessengerV2.sol";
import {IMessageTransmitterV2} from "./interfaces/cctp/IMessageTransmitterV2.sol";
import {IFastFillConfig, ChainConfig} from "./interfaces/IFastFillConfig.sol";

/// @title  CctpExecutor
/// @notice Permissionless, incentivized CCTP v2 mint-relay + hook-executor — a public-good replacement
///         for a centralized forwarding service. A canonical singleton (CREATE2-identical on every
///         chain): anyone may relay a destination mint and earn the burner-specified `mintFee` in USDC,
///         after which the executor forwards the remaining USDC either to a plain recipient
///         (forward-only) or to a receiver contract whose `onCctpExecute` it calls (hook mode).
///
///         Any integrator can use it. A transfer opts in by naming this contract as BOTH the CCTP
///         `mintRecipient` AND `destinationCaller` (so `execute` is the only way the message can be
///         consumed — the mint, the relayer payment, and the hook are atomic, and a griefer cannot
///         consume the message any other way) and putting an `ExecHook` envelope in the `hookData`.
///
///         The executor is deliberately generic: it never inspects the `payload` and trusts no field
///         in it. It forwards the AUTHENTICATED CCTP `sourceDomain`/`sender` to the receiver, which is
///         responsible for its own authentication (see `ICctpExecReceiver`).
contract CctpExecutor is CallbackExecutor, Ownable {
    using AddressCast for bytes32;
    using AddressCast for address;

    /// @notice The immutable chain registry. Same address on every chain (CREATE2).
    IFastFillConfig public immutable config;

    /// @notice When true, `execute` is blocked (claim is never blocked). In-flight messages can always
    ///         be self-redeemed once unpaused; the CCTP message stays valid until consumed.
    bool public paused;

    error Paused();
    error ReceiveMessageFailed();
    error MintRecipientMismatch(bytes32 mintRecipient);
    error MintFeeExceedsMinted(uint256 mintFee, uint256 minted);
    error InvalidRefundTo(address refundTo);
    error HookGasLimitTooHigh(uint64 gasLimit, uint64 maxGasLimit);
    error LengthMismatch();

    /// @notice A CCTP message was relayed and its envelope executed.
    /// @param id        keccak256(message) — a unique id for the relayed message.
    /// @param feeEarner The address paid `mintFee` (the caller-chosen fee recipient; default the caller).
    /// @param target    The envelope target (forward recipient or receiver contract).
    /// @param mintFee   USDC paid to `feeEarner`.
    /// @param forwarded USDC forwarded to `target` (minted minus mintFee).
    event Executed(
        bytes32 indexed id, address indexed feeEarner, address indexed target, uint256 mintFee, uint256 forwarded
    );

    /// @notice A batch item was skipped because it reverted (already relayed, stale attestation, an
    ///         under-funded hook, ...). Its CCTP nonce stays unconsumed, so the item is independently
    ///         retryable; the rest of the batch still executed.
    /// @param index The item's position in the batch.
    /// @param id    keccak256(message) for the skipped item.
    event BatchItemSkipped(uint256 indexed index, bytes32 indexed id);

    constructor(address config_, address owner_) {
        _initializeOwner(owner_);
        config = IFastFillConfig(config_);
    }

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    /// @notice Relay a CCTP mint and execute its envelope. Anyone may call; `msg.sender` earns the
    ///         envelope's `mintFee`. `receiveMessage` enforces that this contract is the
    ///         `destinationCaller`, so the mint and the execution are atomic and this is the message's
    ///         only consumption path. If no relayer finds the `mintFee` worthwhile, the burner (or
    ///         anyone) can self-redeem by calling this and paying the fee to themselves (net zero).
    function execute(bytes calldata message, bytes calldata attestation) external nonReentrant whenNotPaused {
        (address usdc, address transmitter) = _localUsdcAndTransmitter();
        _execute(usdc, transmitter, message, attestation, msg.sender);
    }

    /// @notice Like `execute`, but the caller directs the `mintFee` to `feeRecipient` (e.g. a hot wallet
    ///         relays while the fee accrues to a treasury). `address(0)` => `msg.sender`. This only moves
    ///         the relayer's OWN earned fee; the user-signed delivery `target`/`refundTo` come from the
    ///         attested envelope and are never caller-controlled.
    function executeTo(bytes calldata message, bytes calldata attestation, address feeRecipient)
        external
        nonReentrant
        whenNotPaused
    {
        (address usdc, address transmitter) = _localUsdcAndTransmitter();
        _execute(usdc, transmitter, message, attestation, feeRecipient == address(0) ? msg.sender : feeRecipient);
    }

    /// @notice Relay many CCTP mints in one transaction, earning the sum of their `mintFee`s at
    ///         `feeRecipient` (`address(0)` => `msg.sender`). Each item is attempted independently: one
    ///         that reverts (already relayed, stale attestation, an under-funded hook, ...) is SKIPPED —
    ///         its CCTP nonce stays unconsumed and it is retryable — while the rest still execute.
    ///         `filled[i]` reports per-item success. Reverts only on a length mismatch (a caller error).
    function executeBatch(bytes[] calldata messages, bytes[] calldata attestations, address feeRecipient)
        external
        nonReentrant
        whenNotPaused
        returns (bool[] memory filled)
    {
        if (messages.length != attestations.length) revert LengthMismatch();
        (address usdc, address transmitter) = _localUsdcAndTransmitter();
        address feeEarner = feeRecipient == address(0) ? msg.sender : feeRecipient;
        filled = new bool[](messages.length);
        for (uint256 i; i < messages.length; ++i) {
            // try/catch needs an external call; the OnlySelf trampoline runs under the batch's single
            // nonReentrant guard. A reverting item rolls back fully (its nonce stays unconsumed) and is
            // skipped — successful siblings are unaffected.
            try this._executeOne(usdc, transmitter, messages[i], attestations[i], feeEarner) {
                filled[i] = true;
            } catch {
                emit BatchItemSkipped(i, keccak256(messages[i]));
            }
        }
    }

    /// @dev `OnlySelf` trampoline enabling `executeBatch`'s per-item `try`/`catch`. Not `nonReentrant`, so
    ///      the self-call passes the caller's guard — mirrors `CallbackExecutor._executeDelivery`.
    function _executeOne(
        address usdc,
        address transmitter,
        bytes calldata message,
        bytes calldata attestation,
        address feeRecipient
    ) external {
        if (msg.sender != address(this)) revert OnlySelf();
        _execute(usdc, transmitter, message, attestation, feeRecipient);
    }

    function setPaused(bool newPaused) external onlyOwner {
        paused = newPaused;
    }

    /// @dev Resolve this chain's USDC + CCTP MessageTransmitter from the immutable registry (read once per
    ///      call, and once per batch rather than per item).
    function _localUsdcAndTransmitter() private view returns (address usdc, address transmitter) {
        ChainConfig memory lc = config.chainConfig(block.chainid);
        usdc = lc.usdc;
        transmitter = ITokenMessengerV2(lc.cctpTokenMessenger).localMessageTransmitter();
    }

    /// @dev Core relay: mint + authenticate via CCTP, pay `feeRecipient` the envelope's `mintFee`, then
    ///      forward the rest to the attested `target` (optionally running its `onCctpExecute` hook).
    ///      `target`/`refundTo` come from the source-attested envelope and are never caller-controlled;
    ///      only `feeRecipient` (where the relayer's own fee lands) is a caller choice.
    function _execute(
        address usdc,
        address transmitter,
        bytes calldata message,
        bytes calldata attestation,
        address feeRecipient
    ) internal {
        // Mint + authenticate. Reverts (and rolls back) on a bad attestation or a used nonce.
        if (!IMessageTransmitterV2(transmitter).receiveMessage(message, attestation)) revert ReceiveMessageFailed();

        (
            uint32 sourceDomain,
            bytes32 messageSender,
            bytes32 mintRecipient,
            uint256 amount,
            uint256 feeExecuted,
            bytes calldata hookData
        ) = BurnMessageV2Lib.parse(message);

        if (mintRecipient != address(this).toBytes32()) revert MintRecipientMismatch(mintRecipient);

        // The amount actually minted to us (CCTP deducts feeExecuted at mint). Use only parsed fields.
        uint256 minted = amount - feeExecuted;
        ExecHook memory h = ExecHookLib.decode(hookData);
        if (h.mintFee > minted) revert MintFeeExceedsMinted(h.mintFee, minted);
        if (h.gasLimit > MAX_CALLBACK_GAS_LIMIT) revert HookGasLimitTooHigh(h.gasLimit, MAX_CALLBACK_GAS_LIMIT);

        bytes32 id = keccak256(message);

        // Pay the fee recipient first. If the push fails (e.g. blacklisted), revert the whole relay: the
        // CCTP nonce stays unconsumed and another relayer can retry.
        if (h.mintFee != 0) SafeTransferLib.safeTransfer(usdc, feeRecipient, h.mintFee);

        uint256 forwarded = minted - h.mintFee;
        address target = h.target.toAddress();
        address refundTo = h.refundTo.toAddress();
        if (refundTo == address(0)) refundTo = target;
        if (refundTo == address(this)) revert InvalidRefundTo(refundTo);

        // gasLimit == 0 => forward-only (deliver USDC to `target`, no call). Otherwise call the
        // receiver's onCctpExecute in the same atomic frame, passing ONLY authenticated provenance.
        bytes memory callbackData = h.gasLimit == 0
            ? bytes("")
            : abi.encodeCall(ICctpExecReceiver.onCctpExecute, (sourceDomain, messageSender, usdc, forwarded, h.payload));

        _deliverWithHook(id, usdc, target, forwarded, callbackData, h.gasLimit, refundTo);

        emit Executed(id, feeRecipient, target, h.mintFee, forwarded);
    }
}
