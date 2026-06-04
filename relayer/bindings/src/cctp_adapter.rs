///Module containing a contract's types and functions.
/**

```solidity
library PermitLib {
    struct Permit2Data { uint256 nonce; uint256 deadline; bytes signature; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod PermitLib {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Permit2Data { uint256 nonce; uint256 deadline; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Permit2Data {
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Permit2Data> for UnderlyingRustTuple<'_> {
            fn from(value: Permit2Data) -> Self {
                (value.nonce, value.deadline, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Permit2Data {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonce: tuple.0,
                    deadline: tuple.1,
                    signature: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Permit2Data {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Permit2Data {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Permit2Data {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Permit2Data {
            const NAME: &'static str = "Permit2Data";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Permit2Data(uint256 nonce,uint256 deadline,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.deadline)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Permit2Data {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deadline,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deadline,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PermitLib`](self) contract instance.

See the [wrapper's documentation](`PermitLibInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> PermitLibInstance<P, N> {
        PermitLibInstance::<P, N>::new(address, __provider)
    }
    /**A [`PermitLib`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PermitLib`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PermitLibInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for PermitLibInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PermitLibInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PermitLibInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`PermitLib`](self) contract instance.

See the [wrapper's documentation](`PermitLibInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> PermitLibInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PermitLibInstance<P, N> {
            PermitLibInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PermitLibInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > PermitLibInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library PermitLib {
    struct Permit2Data {
        uint256 nonce;
        uint256 deadline;
        bytes signature;
    }
}

interface CctpAdapter {
    type CallbackResult is uint8;
    type FillStatus is uint8;
    struct Execution {
        uint64 gasLimit;
        bytes data;
    }
    struct Order {
        uint8 bridgeType;
        uint32 srcChainId;
        uint32 dstChainId;
        bytes32 sender;
        bytes32 recipient;
        bytes32 inputToken;
        bytes32 outputToken;
        uint256 inputAmount;
        uint256 outputAmount;
        uint64 nonce;
        uint64 startTime;
        uint64 expectedDeliveryTime;
        uint256 discountRate;
        uint256 baseFee;
        uint64 callbackGasLimit;
        bytes hookData;
    }
    struct OrderRecord {
        address filler;
        FillStatus status;
        uint40 fillTime;
    }

    error AlreadyInitialized();
    error AlreadySettled(bytes32 orderId);
    error DomainMismatch(uint32 configured, uint32 onchain);
    error InsufficientCallbackGas(uint256 available, uint256 callbackGasLimit);
    error InvalidAddress(bytes32 b);
    error InvalidBaseFee(uint256 baseFee, uint256 outputAmount);
    error InvalidCallbackGasLimit(uint64 callbackGasLimit, uint64 maxCallbackGasLimit);
    error InvalidMaxFeeRate(uint256 maxFeeRate);
    error InvalidOutputAmount(uint256 outputAmount, uint256 inputAmount);
    error InvalidWindow(uint64 startTime, uint64 expectedDeliveryTime);
    error MaxFeeTooHigh(uint256 maxFee, uint256 inputAmount);
    error MessageTooShort(uint256 length);
    error MintFeeTooHigh(uint256 mintFee, uint256 maxOutput);
    error MintRecipientMismatch(bytes32 mintRecipient);
    error NewOwnerIsZeroAddress();
    error NoHandoverRequest();
    error NotSourceChain(uint32 srcChainId);
    error NothingToClaim();
    error OnlySelf();
    error OrderAlreadyActive(bytes32 orderId);
    error Paused();
    error ReceiveMessageFailed();
    error RedirectFunds(address dest);
    error Reentrancy();
    error Unauthorized();
    error UnsupportedChain(uint32 chainId);
    error UntrustedExecutor(address caller);
    error UntrustedSender(bytes32 messageSender);
    error UntrustedSourceDomain(uint32 sourceDomain);
    error WrongBridgeType(uint8 expected, uint8 got);
    error WrongDestinationChain(uint32 expected);
    error WrongOutputToken(bytes32 outputToken);
    error ZeroCctpExecutor();
    error ZeroRecipient();

    event Claimed(address indexed account, address indexed token, uint256 amount);
    event DestinationCallback(bytes32 indexed id, address indexed fundsTo, CallbackResult result);
    event OrderCreated(bytes32 indexed orderId, uint8 bridgeType, address indexed sender, uint32 dstChainId, bytes32 outputToken, uint256 outputAmount, uint64 nonce);
    event OrderFilled(bytes32 indexed orderId, address indexed filler, uint256 payoutToRecipient, uint256 feeToFiller, uint40 fillTime);
    event OrderSettled(bytes32 indexed orderId, address indexed filler, uint256 arrivedAmount, uint256 surplusToRecipient);
    event OwnershipHandoverCanceled(address indexed pendingOwner);
    event OwnershipHandoverRequested(address indexed pendingOwner);
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
    event PayoutDeferred(bytes32 indexed id, address indexed to, address indexed token, uint256 amount);

    constructor(address config_, address owner_, uint256 maxFeeRate_, address cctpExecutor_);

    function FINALITY_FAST() external view returns (uint32);
    function FINALITY_FINALIZED() external view returns (uint32);
    function MAX_CALLBACK_GAS_LIMIT() external view returns (uint64);
    function PERMIT2() external view returns (address);
    function _executeDelivery(address token, address recipient, uint256 amount, uint256 gasLimit, bytes memory callbackData) external;
    function cancelOwnershipHandover() external payable;
    function cctpExecutor() external view returns (address);
    function claim(address token) external returns (uint256 amount);
    function claimable(address account, address token) external view returns (uint256);
    function completeOwnershipHandover(address pendingOwner) external payable;
    function config() external view returns (address);
    function fill(Order memory order) external returns (bytes32 orderId);
    function fillFor(Order memory order, address filler, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId);
    function getOrder(bytes32 orderId) external view returns (OrderRecord memory);
    function initiateCCTP(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint256 mintFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec) external returns (bytes32 orderId, uint64 nonce);
    function initiateCCTPFor(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint256 mintFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec, address from, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId, uint64 nonce);
    function maxFeeRate() external view returns (uint256);
    function multicall(bytes[] memory data) external payable returns (bytes[] memory);
    function onCctpExecute(uint32 sourceDomain, bytes32 sender, address usdc, uint256 amount, bytes memory payload) external;
    function owner() external view returns (address result);
    function ownershipHandoverExpiresAt(address pendingOwner) external view returns (uint256 result);
    function paused() external view returns (bool);
    function quoteFill(Order memory order, uint256 fillTime) external view returns (uint256 payoutToRecipient, uint256 feeToFiller);
    function renounceOwnership() external payable;
    function requestOwnershipHandover() external payable;
    function selfPermit(address token, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function setMaxFeeRate(uint256 newMaxFeeRate) external;
    function setPaused(bool newPaused) external;
    function settle(bytes memory message, bytes memory attestation) external;
    function transferOwnership(address newOwner) external payable;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "config_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "owner_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "maxFeeRate_",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "cctpExecutor_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "FINALITY_FAST",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "FINALITY_FINALIZED",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_CALLBACK_GAS_LIMIT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "PERMIT2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "_executeDelivery",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasLimit",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "callbackData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cancelOwnershipHandover",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "cctpExecutor",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claim",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimable",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "completeOwnershipHandover",
    "inputs": [
      {
        "name": "pendingOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "config",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IFastFillConfig"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "fill",
    "inputs": [
      {
        "name": "order",
        "type": "tuple",
        "internalType": "struct Order",
        "components": [
          {
            "name": "bridgeType",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "srcChainId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "dstChainId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "sender",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "recipient",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "inputToken",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "outputToken",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "inputAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "outputAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "nonce",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "startTime",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "expectedDeliveryTime",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "discountRate",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseFee",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callbackGasLimit",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "hookData",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "fillFor",
    "inputs": [
      {
        "name": "order",
        "type": "tuple",
        "internalType": "struct Order",
        "components": [
          {
            "name": "bridgeType",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "srcChainId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "dstChainId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "sender",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "recipient",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "inputToken",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "outputToken",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "inputAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "outputAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "nonce",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "startTime",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "expectedDeliveryTime",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "discountRate",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseFee",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callbackGasLimit",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "hookData",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "filler",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permit",
        "type": "tuple",
        "internalType": "struct PermitLib.Permit2Data",
        "components": [
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOrder",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OrderRecord",
        "components": [
          {
            "name": "filler",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum FillStatus"
          },
          {
            "name": "fillTime",
            "type": "uint40",
            "internalType": "uint40"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initiateCCTP",
    "inputs": [
      {
        "name": "dstChainId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "recipient",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "mintFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minFinalityThreshold",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "deliveryWindow",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "discountRate",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "exec",
        "type": "tuple",
        "internalType": "struct Execution",
        "components": [
          {
            "name": "gasLimit",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "nonce",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initiateCCTPFor",
    "inputs": [
      {
        "name": "dstChainId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "recipient",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "mintFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minFinalityThreshold",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "deliveryWindow",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "discountRate",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "exec",
        "type": "tuple",
        "internalType": "struct Execution",
        "components": [
          {
            "name": "gasLimit",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permit",
        "type": "tuple",
        "internalType": "struct PermitLib.Permit2Data",
        "components": [
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "nonce",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "maxFeeRate",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "multicall",
    "inputs": [
      {
        "name": "data",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "onCctpExecute",
    "inputs": [
      {
        "name": "sourceDomain",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "sender",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "usdc",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "payload",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "result",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ownershipHandoverExpiresAt",
    "inputs": [
      {
        "name": "pendingOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "result",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quoteFill",
    "inputs": [
      {
        "name": "order",
        "type": "tuple",
        "internalType": "struct Order",
        "components": [
          {
            "name": "bridgeType",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "srcChainId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "dstChainId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "sender",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "recipient",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "inputToken",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "outputToken",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "inputAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "outputAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "nonce",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "startTime",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "expectedDeliveryTime",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "discountRate",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseFee",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "callbackGasLimit",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "hookData",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "fillTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "payoutToRecipient",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "feeToFiller",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestOwnershipHandover",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "selfPermit",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMaxFeeRate",
    "inputs": [
      {
        "name": "newMaxFeeRate",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPaused",
    "inputs": [
      {
        "name": "newPaused",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "settle",
    "inputs": [
      {
        "name": "message",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "attestation",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "event",
    "name": "Claimed",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DestinationCallback",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "fundsTo",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "result",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum CallbackResult"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderCreated",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "bridgeType",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "dstChainId",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "outputToken",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "outputAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "nonce",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderFilled",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "filler",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "payoutToRecipient",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "feeToFiller",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fillTime",
        "type": "uint40",
        "indexed": false,
        "internalType": "uint40"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OrderSettled",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "filler",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "arrivedAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "surplusToRecipient",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipHandoverCanceled",
    "inputs": [
      {
        "name": "pendingOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipHandoverRequested",
    "inputs": [
      {
        "name": "pendingOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "oldOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PayoutDeferred",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AlreadySettled",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "DomainMismatch",
    "inputs": [
      {
        "name": "configured",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "onchain",
        "type": "uint32",
        "internalType": "uint32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InsufficientCallbackGas",
    "inputs": [
      {
        "name": "available",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "callbackGasLimit",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidAddress",
    "inputs": [
      {
        "name": "b",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidBaseFee",
    "inputs": [
      {
        "name": "baseFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "outputAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidCallbackGasLimit",
    "inputs": [
      {
        "name": "callbackGasLimit",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "maxCallbackGasLimit",
        "type": "uint64",
        "internalType": "uint64"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidMaxFeeRate",
    "inputs": [
      {
        "name": "maxFeeRate",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidOutputAmount",
    "inputs": [
      {
        "name": "outputAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidWindow",
    "inputs": [
      {
        "name": "startTime",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "expectedDeliveryTime",
        "type": "uint64",
        "internalType": "uint64"
      }
    ]
  },
  {
    "type": "error",
    "name": "MaxFeeTooHigh",
    "inputs": [
      {
        "name": "maxFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MessageTooShort",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MintFeeTooHigh",
    "inputs": [
      {
        "name": "mintFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxOutput",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MintRecipientMismatch",
    "inputs": [
      {
        "name": "mintRecipient",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "NewOwnerIsZeroAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoHandoverRequest",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSourceChain",
    "inputs": [
      {
        "name": "srcChainId",
        "type": "uint32",
        "internalType": "uint32"
      }
    ]
  },
  {
    "type": "error",
    "name": "NothingToClaim",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlySelf",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OrderAlreadyActive",
    "inputs": [
      {
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "Paused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReceiveMessageFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RedirectFunds",
    "inputs": [
      {
        "name": "dest",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "Reentrancy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Unauthorized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnsupportedChain",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint32",
        "internalType": "uint32"
      }
    ]
  },
  {
    "type": "error",
    "name": "UntrustedExecutor",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "UntrustedSender",
    "inputs": [
      {
        "name": "messageSender",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "UntrustedSourceDomain",
    "inputs": [
      {
        "name": "sourceDomain",
        "type": "uint32",
        "internalType": "uint32"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongBridgeType",
    "inputs": [
      {
        "name": "expected",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "got",
        "type": "uint8",
        "internalType": "uint8"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongDestinationChain",
    "inputs": [
      {
        "name": "expected",
        "type": "uint32",
        "internalType": "uint32"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongOutputToken",
    "inputs": [
      {
        "name": "outputToken",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroCctpExecutor",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroRecipient",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod CctpAdapter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c03461015657601f61499738819003918201601f19168301916001600160401b0383118484101761015a57808492608094604052833981010312610156576100478161016e565b6100536020830161016e565b9161006560606040830151920161016e565b92670de0b6b3a76400008211610143576001600160a01b0316638b78c6d8198190555f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a36002556001600160a01b03821615610134576001600160a01b031660805260a052604051614814908161018382396080518181816104d001528181610b7701528181610e140152818161189101528181612cd5015281816131b00152818161346b015281816136f70152613e93015260a05181818161066d01528181610b080152613a760152f35b63069504bd60e41b5f5260045ffd5b5063ad6bb6d160e01b5f5260045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101565756fe60806040526004361015610011575f80fd5b5f803560e01c806316c38b3c146120ac5780631e83409a14611f6f5780632569296214611f0857806331eee44d14611db157806339c3321514611d7657806354d1f13d14611d145780635778472a14611c375780635c975abb14611bf45780635fdc7c12146117af578063681143701461124c5780636afdd85014611204578063715018a614611165578063776ff3c714610e7357806377839a9e14610e3857806379502c5514610dc957806385c1783014610d8d5780638cda96de14610d125780638da5cb5b14610ca1578063928c60b914610a5857806397c36bae14610990578063ac4fca821461089e578063ac9650d814610744578063cc6eec7014610709578063d4570c1c14610691578063dcb41c2814610622578063eafa61a814610363578063f04e283e146102f7578063f2fde38b1461029a578063f3995c67146101b75763fee81cf414610164575f80fd5b346101b45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45761019b61212d565b9063389a75e1600c5252602080600c2054604051908152f35b80fd5b50346101b45760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457806101f061212d565b6064359060ff82168092036102965773ffffffffffffffffffffffffffffffffffffffff1690813b1561029657829160e4839260405195869384927fd505accf00000000000000000000000000000000000000000000000000000000845233600485015230602485015260243560448501526044356064850152608484015260843560a484015260a43560c48401525af16102885780f35b61029191612382565b5f8180f35b5050fd5b5060207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576102cd61212d565b6102d5612553565b8060601b156102ea576102e790613c21565b80f35b637448fbae82526004601cfd5b5060207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45761032a61212d565b610332612553565b63389a75e1600c528082526020600c20805442116103565790826102e79255613c21565b636f5e881883526004601cfd5b50346101b4576101407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45761039c6121d8565b604435906084356064356103ae6121eb565b926103b761220f565b94610124359167ffffffffffffffff831161061e57826004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc853603011261061a5760ff60035460401c166105f257828510156105c25761041c8584612519565b861015610588576104789160246104506104719361043861311a565b9b8c9161010435918c8c8b60e4359489359033613171565b9567ffffffffffffffff610463846122cf565b166101c0880152019061227e565b36916123fd565b6101e083015261048782613404565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057d57606061052e939273ffffffffffffffffffffffffffffffffffffffff926105339a9b9161054e575b5001511630903390612930565b6136d2565b6040805191825267ffffffffffffffff929092166020820152f35b610570915060a03d60a011610576575b6105688183612382565b810190612472565b5f610521565b503d61055e565b6040513d8a823e3d90fd5b604489876105968887612519565b907f1cde3111000000000000000000000000000000000000000000000000000000008352600452602452fd5b60448984877fed2bc1ea000000000000000000000000000000000000000000000000000000008352600452602452fd5b6004897f9e87fac8000000000000000000000000000000000000000000000000000000008152fd5b8880fd5b8780fd5b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101b45760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45773ffffffffffffffffffffffffffffffffffffffff60406106e061212d565b92826106ea612150565b94168152806020522091165f52602052602060405f2054604051908152f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760206040516103e88152f35b5060207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576004359067ffffffffffffffff82116101b457366023830112156101b45781600401359167ffffffffffffffff831161089a578260051b90366024838301011161089657346108965760405193849360208552818560200152604085810194806024860187378601019384926107fe575b50505050806040520360401b178060401c9067ffffffffffffffff16f35b92945090925b8092826044825188016024810135918291018537389084305af41561088d57602067ffffffffffffffe0603f85937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08b87030181528301943d81523d858583013e3d010116943d010152858482101561087e575091610804565b9450505090505f8080806107e0565b863d84823e3d90fd5b8280fd5b5080fd5b50346101b45760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576108d661212d565b6108de612150565b9060843567ffffffffffffffff811161098c576108ff9036906004016121aa565b9091303303610964579161091c916104718694866044359161258a565b916040519238918360208351930191606435f13d60243d1161095c575b80602091845280858386013e83010160405215610954575080f35b805190602001fd5b506024610939565b6004857f14d4a4e8000000000000000000000000000000000000000000000000000000008152fd5b8380fd5b50346101b45760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760043567ffffffffffffffff811161089a576102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc823603011261089a5780610a4b610a44610104604094013592610a1c61014482016122cf565b610a2961016483016122cf565b600254916101846101a4850135940135916024359188613ca2565b8092612519565b9082519182526020820152f35b50346101b45760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457610a906121d8565b906044359173ffffffffffffffffffffffffffffffffffffffff831680930361089a5760843567ffffffffffffffff811161089657610ad39036906004016121aa565b9093688000000000ab143c065c610c94576001688000000000ab143c065d73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163303610c68576040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115610c5d5773ffffffffffffffffffffffffffffffffffffffff916060918791610c3e575b500151168103610c135750610be6610be1610c049495610bf19336916123fd565b612af7565b809260243590612c7f565b60643590610bfe81612db8565b90612f83565b80688000000000ab143c065d80f35b7f2e775c7c000000000000000000000000000000000000000000000000000000008452600452602483fd5b610c57915060a03d60a011610576576105688183612382565b5f610bc0565b6040513d87823e3d90fd5b6024847f50ce3ed900000000000000000000000000000000000000000000000000000000815233600452fd5b63ab143c0684526004601cfd5b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b50346101b45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457600435610d4d612553565b670de0b6b3a76400008111610d625760025580f35b7fad6bb6d1000000000000000000000000000000000000000000000000000000008252600452602490fd5b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576020600254604051908152f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760206040516107d08152f35b50346101b45760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576004359067ffffffffffffffff82116101b457816004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc843603011261089a57610eef612150565b926044359067ffffffffffffffff821161098c57816004019460607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc843603011261116157688000000000ab143c065c611154576001688000000000ab143c065d60ff60035460401c1661112c57610f67818561272b565b9783859b839895969960405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152610fb0606082612382565b51902060405190610fc260a083612382565b606a82527f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c417560208301527f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b6560408301527f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e743260608301527f353620616d6f756e742900000000000000000000000000000000000000000000608083015261106b604484018561227e565b949093602401359035888d6101048d013590611086996143ec565b6101e484016110949161227e565b936101c4016110a2906122cf565b9336906110ae926123fd565b916110b99488612997565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a3688000000000ab143c065d604051908152602090f35b6004857f9e87fac8000000000000000000000000000000000000000000000000000000008152fd5b63ab143c0685526004601cfd5b8480fd5b50807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457611197612553565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275580f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760206040516e22d473030f116ddee9f6b43ac78ba38152f35b50346101b4576101807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576112856121d8565b6044356112906121eb565b9061129961220f565b90610124359167ffffffffffffffff83116117ab57826004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc85360301126117a757610144359273ffffffffffffffffffffffffffffffffffffffff8416840361061e5767ffffffffffffffff610164351161061e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc610164353603011261061e5760ff60035460401c1661177f5780606435101561174e5761136560643582612519565b6084351015611710579160246104506113a1936113b09561138461311a565b9a8b92610104359260e43592608435918c606435928a3591613171565b91906101e085019236916123fd565b81526113bb83613404565b6040516020810190606435825263ffffffff861660408201526084356060820152606081526113eb608082612382565b5190209060ff8451169163ffffffff60408601511691608086015160e087015161010088015167ffffffffffffffff6101608a01511667ffffffffffffffff6101408b01511690039167ffffffffffffffff83116116e357936105339b9c969367ffffffffffffffff8b947f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b98946116d69c9b9861018088015194846101c06101a08b01519851602081519101209a015116996040519b60208d019d8e5260408d015260608c015260808b015260a08a015260c08901521660e087015261010086015261012085015261014084015261016083015261018082015261018081526114f76101a082612382565b5190209161150860a0860151613f39565b9260e08601519250610160916115216040519384612382565b61012983527f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208401527f75696e743820627269646765547970652c75696e74333220647374436861696e60408401527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608401527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808401527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08401527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08401527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08401527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008401527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208401527f3620616d6f756e742900000000000000000000000000000000000000000000006101408401526116bb604461016435016101643560040161227e565b959094806024610164350135936101643560040135936143ec565b60843590606435906136d2565b60248d7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b8761172060449260643590612519565b7f1cde3111000000000000000000000000000000000000000000000000000000008252608435600452602452fd5b7fed2bc1ea000000000000000000000000000000000000000000000000000000008852606435600452602452604487fd5b6004887f9e87fac8000000000000000000000000000000000000000000000000000000008152fd5b8680fd5b8580fd5b34611ac05760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05760043567ffffffffffffffff8111611ac0576117fe9036906004016121aa565b60243567ffffffffffffffff8111611ac05761181e9036906004016121aa565b688000000000ab143c065c611be7576001688000000000ab143c065d60ff60035460401c16611bbf57604051907f0a70b05600000000000000000000000000000000000000000000000000000000825246600483015260a08260248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa918215611b515773ffffffffffffffffffffffffffffffffffffffff60806004946020935f91611ba0575b50015116604051938480927f2c1219210000000000000000000000000000000000000000000000000000000082525afa918215611b51575f92611b5c575b50916020916119ad935f73ffffffffffffffffffffffffffffffffffffffff6040518097819682957f57ecfd280000000000000000000000000000000000000000000000000000000084526040600485015261197d8d8d60448701916124db565b917ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8584030160248601526124db565b0393165af1908115611b51575f91611b17575b5015611aef576101788110611ac45780600811611ac0578060d811611ac05760209160b88101358260f811611ac05760d8820135928061011811611ac05760f8830135918161015811611ac05761013884013595508161017811611ac0577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe886101788501920190308103611a9557611a876004611a7d8989611a788a8a611a6b610be1368c8e6123fd565b968792013560e01c612c7f565b612519565b90610bfe81612db8565b5f688000000000ab143c065d005b7fc7286ea1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5f80fd5b7fa2abf1b6000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f514d840a000000000000000000000000000000000000000000000000000000005f5260045ffd5b90506020813d602011611b49575b81611b3260209383612382565b81010312611ac057611b4390612433565b836119c0565b3d9150611b25565b6040513d5f823e3d90fd5b9150916020823d602011611b98575b81611b7860209383612382565b81010312611ac0576119ad92611b8f602093612451565b9250909261191c565b3d9150611b6b565b611bb9915060a03d60a011610576576105688183612382565b896118de565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b34611ac0575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac057602060ff60035460401c166040519015158152f35b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac0575f60408051611c74816122e4565b82815282602082015201526004355f526001602052606060405f2064ffffffffff60405191611ca2836122e4565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c166020840190611cd181612173565b815282604085019260a81c16825273ffffffffffffffffffffffffffffffffffffffff604051945116845251611d0681612173565b602084015251166040820152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b34611ac0575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac0576020604051624c4b408152f35b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05760043567ffffffffffffffff8111611ac057806004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8336030112611ac057688000000000ab143c065c611be7576001688000000000ab143c065d60ff60035460401c16611bbf57611eb8611eaf917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d611ef1611ec0611ec76101c4602098611e8d338861272b565b9b849d829993949692959c611ea48730338b612930565b6101e484019061227e565b979092016122cf565b9536916123fd565b9289612997565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d604051908152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac057611fa661212d565b688000000000ab143c065c611be7576001688000000000ab143c065d335f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5260205260405f2054801561208457602091335f525f835260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5283525f604081205561202d82338361258a565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd3992683843392a35f688000000000ab143c065d604051908152f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac057600435801515809103611ac0576120f0612553565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff00000000000000006003549260401b169116176003555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff82168203611ac057565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203611ac057565b6003111561217d57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b9181601f84011215611ac05782359167ffffffffffffffff8311611ac05760208381860195010111611ac057565b6004359063ffffffff82168203611ac057565b60a4359063ffffffff82168203611ac057565b359063ffffffff82168203611ac057565b60c4359067ffffffffffffffff82168203611ac057565b359067ffffffffffffffff82168203611ac057565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215611ac0570180359067ffffffffffffffff8211611ac057602001918136038313611ac057565b3567ffffffffffffffff81168103611ac05790565b6060810190811067ffffffffffffffff82111761230057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60a0810190811067ffffffffffffffff82111761230057604052565b610200810190811067ffffffffffffffff82111761230057604052565b6040810190811067ffffffffffffffff82111761230057604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761230057604052565b67ffffffffffffffff811161230057601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b929192612409826123c3565b916124176040519384612382565b829481845281830111611ac0578281602093845f960137010152565b51908115158203611ac057565b519063ffffffff82168203611ac057565b519073ffffffffffffffffffffffffffffffffffffffff82168203611ac057565b908160a0910312611ac0576124d360806040519261248f8461232d565b61249881612433565b84526124a660208201612440565b60208501526124b760408201612440565b60408501526124c860608201612451565b606085015201612451565b608082015290565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b9190820391821161252657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392754330361257d57565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f511416156125c6575b50505f603452565b3b153d1710156125d7575f806125be565b6390b8ec185f526004601cfd5b3560ff81168103611ac05790565b3563ffffffff81168103611ac05790565b919061020083820312611ac0576040519061261d82612349565b8193803560ff81168103611ac0578352612639602082016121fe565b602084015261264a604082016121fe565b6040840152606081013560608401526080810135608084015260a081013560a084015260c081013560c084015260e081013560e084015261010081013561010084015261269a6101208201612226565b6101208401526126ad6101408201612226565b6101408401526126c06101608201612226565b6101608401526101808101356101808401526101a08101356101a08401526126eb6101c08201612226565b6101c08401526101e08101359067ffffffffffffffff8211611ac0570181601f82011215611ac0576101e091816020612726933591016123fd565b910152565b9060ff612737836125e4565b166128f5576040820163ffffffff61274e826125f2565b1646036128bb57506127686127633684612603565b612db8565b92835f52600160205260405f209283549160ff8360a01c1661278981612173565b61288f5761279f61279a3684612603565b613e4a565b946127ad6080840135613f39565b947fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff7401000000000000000000000000000000000000000061282c612825610100880135976127ff61014082016122cf565b61280c61016083016122cf565b600254916101806101a08501359401359142918d613ca2565b8097612519565b967fffffffffffffffffffffff00000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff79ffffffffff0000000000000000000000000000000000000000004260a81b1695169116171716179055565b857f343e211e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6128c963ffffffff916125f2565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b60ff612900836125e4565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f525f6004521660245260445ffd5b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f51141615612979575b50505f606052604052565b3b153d17101561298a575f8061296e565b637939f4245f526004601cfd5b916129c195939181958051155f146129c357506040516129b8602082612382565b5f815293613fb4565b565b612a5b612a2f916040519283917f3866c1dc00000000000000000000000000000000000000000000000000000000602084015288602484015273ffffffffffffffffffffffffffffffffffffffff861660448401528760648401526080608484015260a483019061223b565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282612382565b93613fb4565b9190820180921161252657565b60405190612a7b82612349565b60606101e0835f81525f60208201525f60408201525f838201525f60808201525f60a08201525f60c08201525f60e08201525f6101008201525f6101208201525f6101408201525f6101608201525f6101808201525f6101a08201525f6101c08201520152565b519067ffffffffffffffff82168203611ac057565b612aff612a6e565b5080518101906020820190602081840312611ac057602081015167ffffffffffffffff8111611ac05701916102009083900312611ac05760405191612b4383612349565b602081015160ff81168103611ac0578352612b6060408201612440565b6020840152612b7160608201612440565b60408401526080810151606084015260a0810151608084015260c081015160a084015260e081015160c084015261010081015160e0840152610120810151610100840152612bc26101408201612ae2565b610120840152612bd56101608201612ae2565b610140840152612be86101808201612ae2565b6101608401526101a08101516101808401526101c08101516101a0840152612c136101e08201612ae2565b6101c084015261020081015167ffffffffffffffff8111611ac057602091010181601f82011215611ac057805190612c4a826123c3565b92612c586040519485612382565b82845260208383010111611ac057815f9260208093018386015e830101526101e082015290565b9091602063ffffffff91015116604051907f0a70b056000000000000000000000000000000000000000000000000000000008252600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611b51575f91612d99575b50805115908115612d7e575b50612d4c5750308103612d215750565b7fc21fa2e5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b63ffffffff907f6a96659e000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b63ffffffff9150602001511663ffffffff821614155f612d11565b612db2915060a03d60a011610576576105688183612382565b5f612d05565b906101e08201805151601f81018091116125265760051c90816012018060121161252657601360405193849283520160051b0160405267ffffffffffffffff6101c060208401956020875260ff815116604086015263ffffffff602082015116606086015263ffffffff6040820151166080860152606081015160a0860152608081015160c086015260a081015160e086015260c081015161010086015260e08101516101208601526101008101516101408601528261012082015116610160860152826101408201511661018086015282610160820151166101a0860152610180810151828601526101a08101516101e086015201511661020083015261020061022083015280515161024083015251805180612ef3575b50508051928360051b902092604051828260010160051b011490151060061b52565b602082016020826102608601940101905b818110612f73575050601f16908115612ed15760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80612ed1565b8051845260209384019301612f04565b91815f52600160205260405f208054600260ff8260a01c16612fa481612173565b146130cc57917fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff6040927f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a94612ff988613e4a565b9161300760808a0151613f39565b9073ffffffffffffffffffffffffffffffffffffffff8316996101008101518088105f146130c6575086915b74020000000000000000000000000000000000000000613053848a612519565b97889616179055868b15155f1461309b575050613072908a858b6142c9565b81613089575b5050505b82519182526020820152a3565b61309392886142c9565b5f8181613078565b9150916130c194935067ffffffffffffffff6101c06101e085015194015116938a612997565b61307c565b91613033565b837fb196a44a000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9067ffffffffffffffff8091169116019067ffffffffffffffff821161252657565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff81116125265767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b9594989390929192613181612a6e565b506040517f0a70b0560000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff169a9060a0816024818f5afa8015611b51576060915f916133e5575b50015173ffffffffffffffffffffffffffffffffffffffff16916040518080927f0a70b05600000000000000000000000000000000000000000000000000000000825263ffffffff169d8e60048301525a9260249160a094fa908115611b515773ffffffffffffffffffffffffffffffffffffffff916060915f916133c6575b500151169282156133945783156133685761329b9085612519565b906132a591612519565b934267ffffffffffffffff16966132bc90886130f8565b976040519b6132ca8d612349565b5f8d528c4663ffffffff16906020015260408d015273ffffffffffffffffffffffffffffffffffffffff1660608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015261014085015267ffffffffffffffff166101608401526101808301526101a08201526101c081015f905260405161335c602082612382565b5f81526101e082015290565b8b7fb825dd76000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff461660045260245ffd5b6133df915060a03d60a011610576576105688183612382565b5f613280565b6133fe915060a03d60a011610576576105688183612382565b5f613200565b63ffffffff6020820151164681036136a7575063ffffffff6040820151166040517f0a70b05600000000000000000000000000000000000000000000000000000000815281600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611b51575f91613688575b50805115908115613665575b5061363a575067ffffffffffffffff6101608201511667ffffffffffffffff610140830151168082111561360c57505061010081018051801580156135ff575b6135cb57506101a08201519051908181101561359d57505073ffffffffffffffffffffffffffffffffffffffff6135216080830151613f39565b1615613575576101c0015167ffffffffffffffff16624c4b4081116135435750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f52600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e0830151907f8dd09d91000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5060e083015181116134e7565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff91506060015116155f6134a7565b6136a1915060a03d60a011610576576105688183612382565b5f61349b565b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b909391936136df82612db8565b9373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016916040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a081602481875afa908115611b51575f91613c02575b506080810173ffffffffffffffffffffffffffffffffffffffff8151166004602063ffffffff818601511692604051928380927f2c1219210000000000000000000000000000000000000000000000000000000082525afa908115611b51575f91613bb0575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f8d3638f4000000000000000000000000000000000000000000000000000000008352165afa8015611b51575f90613b73575b63ffffffff91501690808203613b45575050604086019460a063ffffffff8751166024604051809481937f0a70b05600000000000000000000000000000000000000000000000000000000835260048301525afa908115611b515763ffffffff916020915f91613b26575b5001511698806139465750879860a09473ffffffffffffffffffffffffffffffffffffffff947f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a9794866060816138f3979e9c9d9e5116920151169360e08c01516138eb8d614592565b953093614699565b63ffffffff6139056060880151613f39565b9151169560c08101519067ffffffffffffffff6101206101008301519201511691604051985f8a5260208a01526040890152606088015260808701521693a3565b6205573067ffffffffffffffff6101c0899795979694960151160167ffffffffffffffff81116125265767ffffffffffffffff1699624c4b408b11613af357899a6080899a9b990151926139998b614592565b604051916139a68361232d565b825260208201973089526040830193845260608301958652608083019182525173ffffffffffffffffffffffffffffffffffffffff16986060015173ffffffffffffffffffffffffffffffffffffffff169460e08d015193604051998a9460208601602090525160408601525160608501525167ffffffffffffffff1660808401525160a08301525160c0820160a0905260e08201613a449161223b565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018752613a749087612382565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1692613ab597614699565b60a073ffffffffffffffffffffffffffffffffffffffff7f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a926138f3565b8a7f25ad8594000000000000000000000000000000000000000000000000000000005f52600452624c4b4060245260445ffd5b613b3f915060a03d60a011610576576105688183612382565b5f613881565b7fc9e030e8000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b506020813d602011613ba8575b81613b8d60209383612382565b81010312611ac057613ba363ffffffff91612440565b613816565b3d9150613b80565b90506020813d602011613bfa575b81613bcb60209383612382565b81010312611ac0576020613bf373ffffffffffffffffffffffffffffffffffffffff92612451565b91506137c1565b3d9150613bbe565b613c1b915060a03d60a011610576576105688183612382565b5f61375b565b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b959290939167ffffffffffffffff8091169416908181105f14613e4357505b5f93808210613ce9575b505050613cd89250612a61565b81811115613ce4575090565b905090565b613cf592939450612519565b90801580159081613de5575b5015613dde575050815b808311613dd6575b5081830291670de0b6b3a764000084158286860414170215613d485750670de0b6b3a7640000613cd892045b905f8080613ccb565b91670de0b6b3a76400007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8486098281108301900393850983670de0b6b3a76400001115613dc957613cd893828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066902613d3f565b63ae47f7025f526004601cfd5b91505f613d13565b0291613d0b565b9050613e1657807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0482115f613d01565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b9050613cc1565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa8015611b5157606073ffffffffffffffffffffffffffffffffffffffff9160c0935f91613f1a575b50015116910151818103613eef575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b613f33915060a03d60a011610576576105688183612382565b5f613edd565b8060a01c613f5a5773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3d15613faf573d90613f96826123c3565b91613fa46040519384612382565b82523d5f602084013e565b606090565b945f9484156142bf5780511580156142b6575b6142a5575a61c35067ffffffffffffffff613ff3818616956707ffffffffffffff8160051c16906130f8565b160167ffffffffffffffff81116125265767ffffffffffffffff161161427557303b15611ac0575f61408c91604051809381927fac4fca8200000000000000000000000000000000000000000000000000000000835273ffffffffffffffffffffffffffffffffffffffff808916998a600486015216968760248501528a6044850152606484015260a0608484015260a483019061223b565b038183305af19081614260575b5061422e57506140a7613f85565b91849260248151146141bd575b5073ffffffffffffffffffffffffffffffffffffffff831696871515806141b3575b156141185750506020927f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9949261410d92876142c9565b5060405160018152a3565b602097507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a995935087949192507f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f73ffffffffffffffffffffffffffffffffffffffff88921698899384875286825260408720865f52825260405f2061419f828254612a61565b9055604051908152a45060405160028152a3565b50308814156140d6565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff00000000000000000000000000000000000000000000000000000000602460208401519301519216036140b45773ffffffffffffffffffffffffffffffffffffffff1692505f6140b4565b955050505060207f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a991604051908152a3565b61426d9196505f90612382565b5f945f614099565b505a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b505091939092506129c194506142c9565b50833b15613fc7565b5050505050505050565b9083156143af575f806040519473ffffffffffffffffffffffffffffffffffffffff60208701917fa9059cbb000000000000000000000000000000000000000000000000000000008352169586602482015287604482015260448152614330606482612382565b519082855af161433e613f85565b816143b5575b506143af57602073ffffffffffffffffffffffffffffffffffffffff7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f92855f525f835260405f208282165f52835260405f206143a2888254612a61565b90556040519687521694a4565b50505050565b80518015925082156143ca575b50505f614344565b8192509060209181010312611ac05760206143e59101612433565b5f806143c2565b9591989396929897949773ffffffffffffffffffffffffffffffffffffffff6040519761441889612366565b16875260208701526040519561442d876122e4565b865260208601968752604086019788526040519861444a8a612366565b308a5260208a01526e22d473030f116ddee9f6b43ac78ba33b15611ac057604051988998899889987f137c29fe000000000000000000000000000000000000000000000000000000008a5260048a019051906144c5916020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b516044890152516064880152805173ffffffffffffffffffffffffffffffffffffffff1660848801526020015160a487015273ffffffffffffffffffffffffffffffffffffffff1660c486015260e485015261010484016101409052610144840161452f9161223b565b908382037ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc01610124850152614564926124db565b035a925f6e22d473030f116ddee9f6b43ac78ba38195f18015611b51576145885750565b5f6129c191612382565b6146966101e091612a2f60405193849260208085015260ff815116604085015263ffffffff602082015116606085015263ffffffff6040820151166080850152606081015160a0850152608081015160c085015260a081015160e085015260c081015161010085015260e081015161012085015261010081015161014085015267ffffffffffffffff6101208201511661016085015267ffffffffffffffff6101408201511661018085015267ffffffffffffffff610160820151166101a08501526101808101516101c08501526101a08101518285015267ffffffffffffffff6101c082015116610200850152015161020061022084015261024083019061223b565b90565b969591909694939480601452816034526f095ea7b30000000000000000000000005f5260205f60446010828c5af18060015f51141615614793575b505f60345273ffffffffffffffffffffffffffffffffffffffff1693843b15611ac0575f9663ffffffff889461478293829973ffffffffffffffffffffffffffffffffffffffff6040519d8e9c8d9b8c9a7f779b432d000000000000000000000000000000000000000000000000000000008c5260048c01521660248a01528260448a0152166064880152608487015260a48601521660c484015261010060e484015261010483019061223b565b03925af18015611b51576145885750565b3d893b151710156147a5575b5f6146d4565b5f6034526f095ea7b30000000000000000000000005f525f3860446010838c5af1508160345260205f60446010828c5af18060015f511416156147e9575b5061479f565b3d893b151710156147fa575f6147e3565b633e3f8f735f526004601cfdfea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC04a\x01VW`\x1FaI\x978\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01ZW\x80\x84\x92`\x80\x94`@R\x839\x81\x01\x03\x12a\x01VWa\0G\x81a\x01nV[a\0S` \x83\x01a\x01nV[\x91a\0e```@\x83\x01Q\x92\x01a\x01nV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11a\x01CW`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3`\x02U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x014W`\x01`\x01`\xA0\x1B\x03\x16`\x80R`\xA0R`@QaH\x14\x90\x81a\x01\x83\x829`\x80Q\x81\x81\x81a\x04\xD0\x01R\x81\x81a\x0Bw\x01R\x81\x81a\x0E\x14\x01R\x81\x81a\x18\x91\x01R\x81\x81a,\xD5\x01R\x81\x81a1\xB0\x01R\x81\x81a4k\x01R\x81\x81a6\xF7\x01Ra>\x93\x01R`\xA0Q\x81\x81\x81a\x06m\x01R\x81\x81a\x0B\x08\x01Ra:v\x01R\xF3[c\x06\x95\x04\xBD`\xE4\x1B_R`\x04_\xFD[Pc\xADk\xB6\xD1`\xE0\x1B_R`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01VWV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_\x805`\xE0\x1C\x80c\x16\xC3\x8B<\x14a \xACW\x80c\x1E\x83@\x9A\x14a\x1FoW\x80c%i)b\x14a\x1F\x08W\x80c1\xEE\xE4M\x14a\x1D\xB1W\x80c9\xC32\x15\x14a\x1DvW\x80cT\xD1\xF1=\x14a\x1D\x14W\x80cWxG*\x14a\x1C7W\x80c\\\x97Z\xBB\x14a\x1B\xF4W\x80c_\xDC|\x12\x14a\x17\xAFW\x80ch\x11Cp\x14a\x12LW\x80cj\xFD\xD8P\x14a\x12\x04W\x80cqP\x18\xA6\x14a\x11eW\x80cwo\xF3\xC7\x14a\x0EsW\x80cw\x83\x9A\x9E\x14a\x0E8W\x80cyP,U\x14a\r\xC9W\x80c\x85\xC1x0\x14a\r\x8DW\x80c\x8C\xDA\x96\xDE\x14a\r\x12W\x80c\x8D\xA5\xCB[\x14a\x0C\xA1W\x80c\x92\x8C`\xB9\x14a\nXW\x80c\x97\xC3k\xAE\x14a\t\x90W\x80c\xACO\xCA\x82\x14a\x08\x9EW\x80c\xAC\x96P\xD8\x14a\x07DW\x80c\xCCn\xECp\x14a\x07\tW\x80c\xD4W\x0C\x1C\x14a\x06\x91W\x80c\xDC\xB4\x1C(\x14a\x06\"W\x80c\xEA\xFAa\xA8\x14a\x03cW\x80c\xF0N(>\x14a\x02\xF7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9AW\x80c\xF3\x99\\g\x14a\x01\xB7Wc\xFE\xE8\x1C\xF4\x14a\x01dW_\x80\xFD[4a\x01\xB4W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x01\x9Ba!-V[\x90c8\x9Au\xE1`\x0CRR` \x80`\x0C T`@Q\x90\x81R\xF3[\x80\xFD[P4a\x01\xB4W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W\x80a\x01\xF0a!-V[`d5\x90`\xFF\x82\x16\x80\x92\x03a\x02\x96Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81;\x15a\x02\x96W\x82\x91`\xE4\x83\x92`@Q\x95\x86\x93\x84\x92\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R3`\x04\x85\x01R0`$\x85\x01R`$5`D\x85\x01R`D5`d\x85\x01R`\x84\x84\x01R`\x845`\xA4\x84\x01R`\xA45`\xC4\x84\x01RZ\xF1a\x02\x88W\x80\xF3[a\x02\x91\x91a#\x82V[_\x81\x80\xF3[PP\xFD[P` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x02\xCDa!-V[a\x02\xD5a%SV[\x80``\x1B\x15a\x02\xEAWa\x02\xE7\x90a<!V[\x80\xF3[ctH\xFB\xAE\x82R`\x04`\x1C\xFD[P` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x03*a!-V[a\x032a%SV[c8\x9Au\xE1`\x0CR\x80\x82R` `\x0C \x80TB\x11a\x03VW\x90\x82a\x02\xE7\x92Ua<!V[co^\x88\x18\x83R`\x04`\x1C\xFD[P4a\x01\xB4Wa\x01@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x03\x9Ca!\xD8V[`D5\x90`\x845`d5a\x03\xAEa!\xEBV[\x92a\x03\xB7a\"\x0FV[\x94a\x01$5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x06\x1EW\x82`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x856\x03\x01\x12a\x06\x1AW`\xFF`\x03T`@\x1C\x16a\x05\xF2W\x82\x85\x10\x15a\x05\xC2Wa\x04\x1C\x85\x84a%\x19V[\x86\x10\x15a\x05\x88Wa\x04x\x91`$a\x04Pa\x04q\x93a\x048a1\x1AV[\x9B\x8C\x91a\x01\x045\x91\x8C\x8C\x8B`\xE45\x94\x895\x903a1qV[\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x04c\x84a\"\xCFV[\x16a\x01\xC0\x88\x01R\x01\x90a\"~V[6\x91a#\xFDV[a\x01\xE0\x83\x01Ra\x04\x87\x82a4\x04V[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x05}W``a\x05.\x93\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a\x053\x9A\x9B\x91a\x05NW[P\x01Q\x160\x903\x90a)0V[a6\xD2V[`@\x80Q\x91\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x82\x01R\xF3[a\x05p\x91P`\xA0=`\xA0\x11a\x05vW[a\x05h\x81\x83a#\x82V[\x81\x01\x90a$rV[_a\x05!V[P=a\x05^V[`@Q=\x8A\x82>=\x90\xFD[`D\x89\x87a\x05\x96\x88\x87a%\x19V[\x90\x7F\x1C\xDE1\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04R`$R\xFD[`D\x89\x84\x87\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04R`$R\xFD[`\x04\x89\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x88\x80\xFD[\x87\x80\xFD[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\xB4W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@a\x06\xE0a!-V[\x92\x82a\x06\xEAa!PV[\x94\x16\x81R\x80` R \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qa\x03\xE8\x81R\xF3[P` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xB4W6`#\x83\x01\x12\x15a\x01\xB4W\x81`\x04\x015\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08\x9AW\x82`\x05\x1B\x906`$\x83\x83\x01\x01\x11a\x08\x96W4a\x08\x96W`@Q\x93\x84\x93` \x85R\x81\x85` \x01R`@\x85\x81\x01\x94\x80`$\x86\x01\x877\x86\x01\x01\x93\x84\x92a\x07\xFEW[PPPP\x80`@R\x03`@\x1B\x17\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[\x92\x94P\x90\x92[\x80\x92\x82`D\x82Q\x88\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x08\x8DW` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x85\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8B\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x94=\x01\x01R\x85\x84\x82\x10\x15a\x08~WP\x91a\x08\x04V[\x94PPP\x90P_\x80\x80\x80a\x07\xE0V[\x86=\x84\x82>=\x90\xFD[\x82\x80\xFD[P\x80\xFD[P4a\x01\xB4W`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x08\xD6a!-V[a\x08\xDEa!PV[\x90`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t\x8CWa\x08\xFF\x906\x90`\x04\x01a!\xAAV[\x90\x9103\x03a\tdW\x91a\t\x1C\x91a\x04q\x86\x94\x86`D5\x91a%\x8AV[\x91`@Q\x928\x91\x83` \x83Q\x93\x01\x91`d5\xF1=`$=\x11a\t\\W[\x80` \x91\x84R\x80\x85\x83\x86\x01>\x83\x01\x01`@R\x15a\tTWP\x80\xF3[\x80Q\x90` \x01\xFD[P`$a\t9V[`\x04\x85\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x83\x80\xFD[P4a\x01\xB4W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\x9AWa\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x826\x03\x01\x12a\x08\x9AW\x80a\nKa\nDa\x01\x04`@\x94\x015\x92a\n\x1Ca\x01D\x82\x01a\"\xCFV[a\n)a\x01d\x83\x01a\"\xCFV[`\x02T\x91a\x01\x84a\x01\xA4\x85\x015\x94\x015\x91`$5\x91\x88a<\xA2V[\x80\x92a%\x19V[\x90\x82Q\x91\x82R` \x82\x01R\xF3[P4a\x01\xB4W`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\n\x90a!\xD8V[\x90`D5\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x08\x9AW`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\x96Wa\n\xD3\x906\x90`\x04\x01a!\xAAV[\x90\x93h\x80\0\0\0\0\xAB\x14<\x06\\a\x0C\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x0ChW`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x0C]Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91``\x91\x87\x91a\x0C>W[P\x01Q\x16\x81\x03a\x0C\x13WPa\x0B\xE6a\x0B\xE1a\x0C\x04\x94\x95a\x0B\xF1\x936\x91a#\xFDV[a*\xF7V[\x80\x92`$5\x90a,\x7FV[`d5\x90a\x0B\xFE\x81a-\xB8V[\x90a/\x83V[\x80h\x80\0\0\0\0\xAB\x14<\x06]\x80\xF3[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04R`$\x83\xFD[a\x0CW\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a\x0B\xC0V[`@Q=\x87\x82>=\x90\xFD[`$\x84\x7FP\xCE>\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04R\xFD[c\xAB\x14<\x06\x84R`\x04`\x1C\xFD[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[P4a\x01\xB4W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045a\rMa%SV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\rbW`\x02U\x80\xF3[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04R`$\x90\xFD[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `\x02T`@Q\x90\x81R\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qa\x07\xD0\x81R\xF3[P4a\x01\xB4W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xB4W\x81`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x08\x9AWa\x0E\xEFa!PV[\x92`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\t\x8CW\x81`\x04\x01\x94``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x11aWh\x80\0\0\0\0\xAB\x14<\x06\\a\x11TW`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x11,Wa\x0Fg\x81\x85a'+V[\x97\x83\x85\x9B\x83\x98\x95\x96\x99`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\x0F\xB0``\x82a#\x82V[Q\x90 `@Q\x90a\x0F\xC2`\xA0\x83a#\x82V[`j\x82R\x7FFillAuthorization witness)FillAu` \x83\x01R\x7Fthorization(bytes32 orderId)Toke`@\x83\x01R\x7FnPermissions(address token,uint2``\x83\x01R\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x83\x01Ra\x10k`D\x84\x01\x85a\"~V[\x94\x90\x93`$\x015\x905\x88\x8Da\x01\x04\x8D\x015\x90a\x10\x86\x99aC\xECV[a\x01\xE4\x84\x01a\x10\x94\x91a\"~V[\x93a\x01\xC4\x01a\x10\xA2\x90a\"\xCFV[\x936\x90a\x10\xAE\x92a#\xFDV[\x91a\x10\xB9\x94\x88a)\x97V[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[`\x04\x85\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[c\xAB\x14<\x06\x85R`\x04`\x1C\xFD[\x84\x80\xFD[P\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x11\x97a%SV[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\x80\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[P4a\x01\xB4Wa\x01\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x12\x85a!\xD8V[`D5a\x12\x90a!\xEBV[\x90a\x12\x99a\"\x0FV[\x90a\x01$5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x17\xABW\x82`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x856\x03\x01\x12a\x17\xA7Wa\x01D5\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x84\x03a\x06\x1EWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01d5\x11a\x06\x1EW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFCa\x01d56\x03\x01\x12a\x06\x1EW`\xFF`\x03T`@\x1C\x16a\x17\x7FW\x80`d5\x10\x15a\x17NWa\x13e`d5\x82a%\x19V[`\x845\x10\x15a\x17\x10W\x91`$a\x04Pa\x13\xA1\x93a\x13\xB0\x95a\x13\x84a1\x1AV[\x9A\x8B\x92a\x01\x045\x92`\xE45\x92`\x845\x91\x8C`d5\x92\x8A5\x91a1qV[\x91\x90a\x01\xE0\x85\x01\x926\x91a#\xFDV[\x81Ra\x13\xBB\x83a4\x04V[`@Q` \x81\x01\x90`d5\x82Rc\xFF\xFF\xFF\xFF\x86\x16`@\x82\x01R`\x845``\x82\x01R``\x81Ra\x13\xEB`\x80\x82a#\x82V[Q\x90 \x90`\xFF\x84Q\x16\x91c\xFF\xFF\xFF\xFF`@\x86\x01Q\x16\x91`\x80\x86\x01Q`\xE0\x87\x01Qa\x01\0\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x8A\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x8B\x01Q\x16\x90\x03\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x16\xE3W\x93a\x053\x9B\x9C\x96\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x94\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x98\x94a\x16\xD6\x9C\x9B\x98a\x01\x80\x88\x01Q\x94\x84a\x01\xC0a\x01\xA0\x8B\x01Q\x98Q` \x81Q\x91\x01 \x9A\x01Q\x16\x99`@Q\x9B` \x8D\x01\x9D\x8ER`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Ra\x01\x80\x82\x01Ra\x01\x80\x81Ra\x14\xF7a\x01\xA0\x82a#\x82V[Q\x90 \x91a\x15\x08`\xA0\x86\x01Qa?9V[\x92`\xE0\x86\x01Q\x92Pa\x01`\x91a\x15!`@Q\x93\x84a#\x82V[a\x01)\x83R\x7FOrderIntent witness)OrderIntent(` \x84\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x84\x01R\x7FId,bytes32 recipient,uint256 inp``\x84\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x84\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x84\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x84\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x84\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x84\x01R\x7FPermissions(address token,uint25a\x01 \x84\x01R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x84\x01Ra\x16\xBB`Da\x01d5\x01a\x01d5`\x04\x01a\"~V[\x95\x90\x94\x80`$a\x01d5\x015\x93a\x01d5`\x04\x015\x93aC\xECV[`\x845\x90`d5\x90a6\xD2V[`$\x8D\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x87a\x17 `D\x92`d5\x90a%\x19V[\x7F\x1C\xDE1\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x845`\x04R`$R\xFD[\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R`d5`\x04R`$R`D\x87\xFD[`\x04\x88\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x86\x80\xFD[\x85\x80\xFD[4a\x1A\xC0W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0Wa\x17\xFE\x906\x90`\x04\x01a!\xAAV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0Wa\x18\x1E\x906\x90`\x04\x01a!\xAAV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x1B\xE7W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x1B\xBFW`@Q\x90\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RF`\x04\x83\x01R`\xA0\x82`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x1BQWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80`\x04\x94` \x93_\x91a\x1B\xA0W[P\x01Q\x16`@Q\x93\x84\x80\x92\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x1BQW_\x92a\x1B\\W[P\x91` \x91a\x19\xAD\x93_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x80\x97\x81\x96\x82\x95\x7FW\xEC\xFD(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`@`\x04\x85\x01Ra\x19}\x8D\x8D`D\x87\x01\x91a$\xDBV[\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x85\x84\x03\x01`$\x86\x01Ra$\xDBV[\x03\x93\x16Z\xF1\x90\x81\x15a\x1BQW_\x91a\x1B\x17W[P\x15a\x1A\xEFWa\x01x\x81\x10a\x1A\xC4W\x80`\x08\x11a\x1A\xC0W\x80`\xD8\x11a\x1A\xC0W` \x91`\xB8\x81\x015\x82`\xF8\x11a\x1A\xC0W`\xD8\x82\x015\x92\x80a\x01\x18\x11a\x1A\xC0W`\xF8\x83\x015\x91\x81a\x01X\x11a\x1A\xC0Wa\x018\x84\x015\x95P\x81a\x01x\x11a\x1A\xC0W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x88a\x01x\x85\x01\x92\x01\x900\x81\x03a\x1A\x95Wa\x1A\x87`\x04a\x1A}\x89\x89a\x1Ax\x8A\x8Aa\x1Aka\x0B\xE16\x8C\x8Ea#\xFDV[\x96\x87\x92\x015`\xE0\x1Ca,\x7FV[a%\x19V[\x90a\x0B\xFE\x81a-\xB8V[_h\x80\0\0\0\0\xAB\x14<\x06]\0[\x7F\xC7(n\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[_\x80\xFD[\x7F\xA2\xAB\xF1\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FQM\x84\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P` \x81=` \x11a\x1BIW[\x81a\x1B2` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0Wa\x1BC\x90a$3V[\x83a\x19\xC0V[=\x91Pa\x1B%V[`@Q=_\x82>=\x90\xFD[\x91P\x91` \x82=` \x11a\x1B\x98W[\x81a\x1Bx` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0Wa\x19\xAD\x92a\x1B\x8F` \x93a$QV[\x92P\x90\x92a\x19\x1CV[=\x91Pa\x1BkV[a\x1B\xB9\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[\x89a\x18\xDEV[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[4a\x1A\xC0W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W_`@\x80Qa\x1Ct\x81a\"\xE4V[\x82\x81R\x82` \x82\x01R\x01R`\x045_R`\x01` R```@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x1C\xA2\x83a\"\xE4V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16` \x84\x01\x90a\x1C\xD1\x81a!sV[\x81R\x82`@\x85\x01\x92`\xA8\x1C\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x94Q\x16\x84RQa\x1D\x06\x81a!sV[` \x84\x01RQ\x16`@\x82\x01R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0Wc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[4a\x1A\xC0W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W` `@QbLK@\x81R\xF3[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0W\x80`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x1A\xC0Wh\x80\0\0\0\0\xAB\x14<\x06\\a\x1B\xE7W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x1B\xBFWa\x1E\xB8a\x1E\xAF\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x1E\xF1a\x1E\xC0a\x1E\xC7a\x01\xC4` \x98a\x1E\x8D3\x88a'+V[\x9B\x84\x9D\x82\x99\x93\x94\x96\x92\x95\x9Ca\x1E\xA4\x8703\x8Ba)0V[a\x01\xE4\x84\x01\x90a\"~V[\x97\x90\x92\x01a\"\xCFV[\x956\x91a#\xFDV[\x92\x89a)\x97V[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0Wc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0Wa\x1F\xA6a!-V[h\x80\0\0\0\0\xAB\x14<\x06\\a\x1B\xE7W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R` R`@_ T\x80\x15a \x84W` \x913_R_\x83R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x83R_`@\x81 Ua -\x823\x83a%\x8AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83\x843\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W`\x045\x80\x15\x15\x80\x91\x03a\x1A\xC0Wa \xF0a%SV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`\x03\x11\x15a!}WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x1A\xC0W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x1A\xC0W` \x83\x81\x86\x01\x95\x01\x01\x11a\x1A\xC0WV[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`\xA45\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`\xC45\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x1A\xC0W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1A\xC0W` \x01\x91\x816\x03\x83\x13a\x1A\xC0WV[5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x1A\xC0W\x90V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[a\x02\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a#\0W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a$\t\x82a#\xC3V[\x91a$\x17`@Q\x93\x84a#\x82V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x1A\xC0W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[Q\x90\x81\x15\x15\x82\x03a\x1A\xC0WV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[\x90\x81`\xA0\x91\x03\x12a\x1A\xC0Wa$\xD3`\x80`@Q\x92a$\x8F\x84a#-V[a$\x98\x81a$3V[\x84Ra$\xA6` \x82\x01a$@V[` \x85\x01Ra$\xB7`@\x82\x01a$@V[`@\x85\x01Ra$\xC8``\x82\x01a$QV[``\x85\x01R\x01a$QV[`\x80\x82\x01R\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x03\x91\x82\x11a%&WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a%}WV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a%\xC6W[PP_`4RV[;\x15=\x17\x10\x15a%\xD7W_\x80a%\xBEV[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5`\xFF\x81\x16\x81\x03a\x1A\xC0W\x90V[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x1A\xC0W\x90V[\x91\x90a\x02\0\x83\x82\x03\x12a\x1A\xC0W`@Q\x90a&\x1D\x82a#IV[\x81\x93\x805`\xFF\x81\x16\x81\x03a\x1A\xC0W\x83Ra&9` \x82\x01a!\xFEV[` \x84\x01Ra&J`@\x82\x01a!\xFEV[`@\x84\x01R``\x81\x015``\x84\x01R`\x80\x81\x015`\x80\x84\x01R`\xA0\x81\x015`\xA0\x84\x01R`\xC0\x81\x015`\xC0\x84\x01R`\xE0\x81\x015`\xE0\x84\x01Ra\x01\0\x81\x015a\x01\0\x84\x01Ra&\x9Aa\x01 \x82\x01a\"&V[a\x01 \x84\x01Ra&\xADa\x01@\x82\x01a\"&V[a\x01@\x84\x01Ra&\xC0a\x01`\x82\x01a\"&V[a\x01`\x84\x01Ra\x01\x80\x81\x015a\x01\x80\x84\x01Ra\x01\xA0\x81\x015a\x01\xA0\x84\x01Ra&\xEBa\x01\xC0\x82\x01a\"&V[a\x01\xC0\x84\x01Ra\x01\xE0\x81\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1A\xC0W\x01\x81`\x1F\x82\x01\x12\x15a\x1A\xC0Wa\x01\xE0\x91\x81` a'&\x935\x91\x01a#\xFDV[\x91\x01RV[\x90`\xFFa'7\x83a%\xE4V[\x16a(\xF5W`@\x82\x01c\xFF\xFF\xFF\xFFa'N\x82a%\xF2V[\x16F\x03a(\xBBWPa'ha'c6\x84a&\x03V[a-\xB8V[\x92\x83_R`\x01` R`@_ \x92\x83T\x91`\xFF\x83`\xA0\x1C\x16a'\x89\x81a!sV[a(\x8FWa'\x9Fa'\x9A6\x84a&\x03V[a>JV[\x94a'\xAD`\x80\x84\x015a?9V[\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a(,a(%a\x01\0\x88\x015\x97a'\xFFa\x01@\x82\x01a\"\xCFV[a(\x0Ca\x01`\x83\x01a\"\xCFV[`\x02T\x91a\x01\x80a\x01\xA0\x85\x015\x94\x015\x91B\x91\x8Da<\xA2V[\x80\x97a%\x19V[\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFy\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0B`\xA8\x1B\x16\x95\x16\x91\x16\x17\x17\x16\x17\x90UV[\x85\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a(\xC9c\xFF\xFF\xFF\xFF\x91a%\xF2V[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\xFFa)\0\x83a%\xE4V[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R\x16`$R`D_\xFD[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a)yW[PP_``R`@RV[;\x15=\x17\x10\x15a)\x8AW_\x80a)nV[cy9\xF4$_R`\x04`\x1C\xFD[\x91a)\xC1\x95\x93\x91\x81\x95\x80Q\x15_\x14a)\xC3WP`@Qa)\xB8` \x82a#\x82V[_\x81R\x93a?\xB4V[V[a*[a*/\x91`@Q\x92\x83\x91\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x88`$\x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`D\x84\x01R\x87`d\x84\x01R`\x80`\x84\x84\x01R`\xA4\x83\x01\x90a\";V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a#\x82V[\x93a?\xB4V[\x91\x90\x82\x01\x80\x92\x11a%&WV[`@Q\x90a*{\x82a#IV[``a\x01\xE0\x83_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R_`\xA0\x82\x01R_`\xC0\x82\x01R_`\xE0\x82\x01R_a\x01\0\x82\x01R_a\x01 \x82\x01R_a\x01@\x82\x01R_a\x01`\x82\x01R_a\x01\x80\x82\x01R_a\x01\xA0\x82\x01R_a\x01\xC0\x82\x01R\x01RV[Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[a*\xFFa*nV[P\x80Q\x81\x01\x90` \x82\x01\x90` \x81\x84\x03\x12a\x1A\xC0W` \x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0W\x01\x91a\x02\0\x90\x83\x90\x03\x12a\x1A\xC0W`@Q\x91a+C\x83a#IV[` \x81\x01Q`\xFF\x81\x16\x81\x03a\x1A\xC0W\x83Ra+``@\x82\x01a$@V[` \x84\x01Ra+q``\x82\x01a$@V[`@\x84\x01R`\x80\x81\x01Q``\x84\x01R`\xA0\x81\x01Q`\x80\x84\x01R`\xC0\x81\x01Q`\xA0\x84\x01R`\xE0\x81\x01Q`\xC0\x84\x01Ra\x01\0\x81\x01Q`\xE0\x84\x01Ra\x01 \x81\x01Qa\x01\0\x84\x01Ra+\xC2a\x01@\x82\x01a*\xE2V[a\x01 \x84\x01Ra+\xD5a\x01`\x82\x01a*\xE2V[a\x01@\x84\x01Ra+\xE8a\x01\x80\x82\x01a*\xE2V[a\x01`\x84\x01Ra\x01\xA0\x81\x01Qa\x01\x80\x84\x01Ra\x01\xC0\x81\x01Qa\x01\xA0\x84\x01Ra,\x13a\x01\xE0\x82\x01a*\xE2V[a\x01\xC0\x84\x01Ra\x02\0\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0W` \x91\x01\x01\x81`\x1F\x82\x01\x12\x15a\x1A\xC0W\x80Q\x90a,J\x82a#\xC3V[\x92a,X`@Q\x94\x85a#\x82V[\x82\x84R` \x83\x83\x01\x01\x11a\x1A\xC0W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01Ra\x01\xE0\x82\x01R\x90V[\x90\x91` c\xFF\xFF\xFF\xFF\x91\x01Q\x16`@Q\x90\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x1BQW_\x91a-\x99W[P\x80Q\x15\x90\x81\x15a-~W[Pa-LWP0\x81\x03a-!WPV[\x7F\xC2\x1F\xA2\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x90\x7Fj\x96e\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x91P` \x01Q\x16c\xFF\xFF\xFF\xFF\x82\x16\x14\x15_a-\x11V[a-\xB2\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a-\x05V[\x90a\x01\xE0\x82\x01\x80QQ`\x1F\x81\x01\x80\x91\x11a%&W`\x05\x1C\x90\x81`\x12\x01\x80`\x12\x11a%&W`\x13`@Q\x93\x84\x92\x83R\x01`\x05\x1B\x01`@Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0` \x84\x01\x95` \x87R`\xFF\x81Q\x16`@\x86\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16``\x86\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`\x80\x86\x01R``\x81\x01Q`\xA0\x86\x01R`\x80\x81\x01Q`\xC0\x86\x01R`\xA0\x81\x01Q`\xE0\x86\x01R`\xC0\x81\x01Qa\x01\0\x86\x01R`\xE0\x81\x01Qa\x01 \x86\x01Ra\x01\0\x81\x01Qa\x01@\x86\x01R\x82a\x01 \x82\x01Q\x16a\x01`\x86\x01R\x82a\x01@\x82\x01Q\x16a\x01\x80\x86\x01R\x82a\x01`\x82\x01Q\x16a\x01\xA0\x86\x01Ra\x01\x80\x81\x01Q\x82\x86\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x86\x01R\x01Q\x16a\x02\0\x83\x01Ra\x02\0a\x02 \x83\x01R\x80QQa\x02@\x83\x01RQ\x80Q\x80a.\xF3W[PP\x80Q\x92\x83`\x05\x1B\x90 \x92`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x86\x01\x94\x01\x01\x90[\x81\x81\x10a/sWPP`\x1F\x16\x90\x81\x15a.\xD1W`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a.\xD1V[\x80Q\x84R` \x93\x84\x01\x93\x01a/\x04V[\x91\x81_R`\x01` R`@_ \x80T`\x02`\xFF\x82`\xA0\x1C\x16a/\xA4\x81a!sV[\x14a0\xCCW\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x92\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x94a/\xF9\x88a>JV[\x91a0\x07`\x80\x8A\x01Qa?9V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x99a\x01\0\x81\x01Q\x80\x88\x10_\x14a0\xC6WP\x86\x91[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a0S\x84\x8Aa%\x19V[\x97\x88\x96\x16\x17\x90U\x86\x8B\x15\x15_\x14a0\x9BWPPa0r\x90\x8A\x85\x8BaB\xC9V[\x81a0\x89W[PPP[\x82Q\x91\x82R` \x82\x01R\xA3V[a0\x93\x92\x88aB\xC9V[_\x81\x81a0xV[\x91P\x91a0\xC1\x94\x93Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0a\x01\xE0\x85\x01Q\x94\x01Q\x16\x93\x8Aa)\x97V[a0|V[\x91a03V[\x83\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a%&WV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a%&Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[\x95\x94\x98\x93\x90\x92\x91\x92a1\x81a*nV[P`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9A\x90`\xA0\x81`$\x81\x8FZ\xFA\x80\x15a\x1BQW``\x91_\x91a3\xE5W[P\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91`@Q\x80\x80\x92\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rc\xFF\xFF\xFF\xFF\x16\x9D\x8E`\x04\x83\x01RZ\x92`$\x91`\xA0\x94\xFA\x90\x81\x15a\x1BQWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91``\x91_\x91a3\xC6W[P\x01Q\x16\x92\x82\x15a3\x94W\x83\x15a3hWa2\x9B\x90\x85a%\x19V[\x90a2\xA5\x91a%\x19V[\x93Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96a2\xBC\x90\x88a0\xF8V[\x97`@Q\x9Ba2\xCA\x8Da#IV[_\x8DR\x8CFc\xFF\xFF\xFF\xFF\x16\x90` \x01R`@\x8D\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Ra\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01Ra\x01\xC0\x81\x01_\x90R`@Qa3\\` \x82a#\x82V[_\x81Ra\x01\xE0\x82\x01R\x90V[\x8B\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFFF\x16`\x04R`$_\xFD[a3\xDF\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a2\x80V[a3\xFE\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a2\0V[c\xFF\xFF\xFF\xFF` \x82\x01Q\x16F\x81\x03a6\xA7WPc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x1BQW_\x91a6\x88W[P\x80Q\x15\x90\x81\x15a6eW[Pa6:WPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x83\x01Q\x16\x80\x82\x11\x15a6\x0CWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a5\xFFW[a5\xCBWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a5\x9DWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa5!`\x80\x83\x01Qa?9V[\x16\x15a5uWa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a5CWPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x90\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a4\xE7V[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P``\x01Q\x16\x15_a4\xA7V[a6\xA1\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a4\x9BV[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x93\x91\x93a6\xDF\x82a-\xB8V[\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x87Z\xFA\x90\x81\x15a\x1BQW_\x91a<\x02W[P`\x80\x81\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16`\x04` c\xFF\xFF\xFF\xFF\x81\x86\x01Q\x16\x92`@Q\x92\x83\x80\x92\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x1BQW_\x91a;\xB0W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\x8D68\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x80\x15a\x1BQW_\x90a;sW[c\xFF\xFF\xFF\xFF\x91P\x16\x90\x80\x82\x03a;EWPP`@\x86\x01\x94`\xA0c\xFF\xFF\xFF\xFF\x87Q\x16`$`@Q\x80\x94\x81\x93\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x1BQWc\xFF\xFF\xFF\xFF\x91` \x91_\x91a;&W[P\x01Q\x16\x98\x80a9FWP\x87\x98`\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x97\x94\x86``\x81a8\xF3\x97\x9E\x9C\x9D\x9EQ\x16\x92\x01Q\x16\x93`\xE0\x8C\x01Qa8\xEB\x8DaE\x92V[\x950\x93aF\x99V[c\xFF\xFF\xFF\xFFa9\x05``\x88\x01Qa?9V[\x91Q\x16\x95`\xC0\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01 a\x01\0\x83\x01Q\x92\x01Q\x16\x91`@Q\x98_\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R\x16\x93\xA3V[b\x05W0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x89\x97\x95\x97\x96\x94\x96\x01Q\x16\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a%&Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x99bLK@\x8B\x11a:\xF3W\x89\x9A`\x80\x89\x9A\x9B\x99\x01Q\x92a9\x99\x8BaE\x92V[`@Q\x91a9\xA6\x83a#-V[\x82R` \x82\x01\x970\x89R`@\x83\x01\x93\x84R``\x83\x01\x95\x86R`\x80\x83\x01\x91\x82RQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x98``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94`\xE0\x8D\x01Q\x93`@Q\x99\x8A\x94` \x86\x01` \x90RQ`@\x86\x01RQ``\x85\x01RQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x84\x01RQ`\xA0\x83\x01RQ`\xC0\x82\x01`\xA0\x90R`\xE0\x82\x01a:D\x91a\";V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x87Ra:t\x90\x87a#\x82V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92a:\xB5\x97aF\x99V[`\xA0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x92a8\xF3V[\x8A\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RbLK@`$R`D_\xFD[a;?\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a8\x81V[\x7F\xC9\xE00\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P` \x81=` \x11a;\xA8W[\x81a;\x8D` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0Wa;\xA3c\xFF\xFF\xFF\xFF\x91a$@V[a8\x16V[=\x91Pa;\x80V[\x90P` \x81=` \x11a;\xFAW[\x81a;\xCB` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0W` a;\xF3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a$QV[\x91Pa7\xC1V[=\x91Pa;\xBEV[a<\x1B\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a7[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x95\x92\x90\x93\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x90\x81\x81\x10_\x14a>CWP[_\x93\x80\x82\x10a<\xE9W[PPPa<\xD8\x92Pa*aV[\x81\x81\x11\x15a<\xE4WP\x90V[\x90P\x90V[a<\xF5\x92\x93\x94Pa%\x19V[\x90\x80\x15\x80\x15\x90\x81a=\xE5W[P\x15a=\xDEWPP\x81[\x80\x83\x11a=\xD6W[P\x81\x83\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x84\x15\x82\x86\x86\x04\x14\x17\x02\x15a=HWPg\r\xE0\xB6\xB3\xA7d\0\0a<\xD8\x92\x04[\x90_\x80\x80a<\xCBV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x86\t\x82\x81\x10\x83\x01\x90\x03\x93\x85\t\x83g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a=\xC9Wa<\xD8\x93\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02a=?V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x91P_a=\x13V[\x02\x91a=\x0BV[\x90Pa>\x16W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11_a=\x01V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90Pa<\xC1V[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x1BQW``s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\xC0\x93_\x91a?\x1AW[P\x01Q\x16\x91\x01Q\x81\x81\x03a>\xEFWP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a?3\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a>\xDDV[\x80`\xA0\x1Ca?ZWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[=\x15a?\xAFW=\x90a?\x96\x82a#\xC3V[\x91a?\xA4`@Q\x93\x84a#\x82V[\x82R=_` \x84\x01>V[``\x90V[\x94_\x94\x84\x15aB\xBFW\x80Q\x15\x80\x15aB\xB6W[aB\xA5WZa\xC3Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa?\xF3\x81\x86\x16\x95g\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`\x05\x1C\x16\x90a0\xF8V[\x16\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a%&Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aBuW0;\x15a\x1A\xC0W_a@\x8C\x91`@Q\x80\x93\x81\x92\x7F\xACO\xCA\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x99\x8A`\x04\x86\x01R\x16\x96\x87`$\x85\x01R\x8A`D\x85\x01R`d\x84\x01R`\xA0`\x84\x84\x01R`\xA4\x83\x01\x90a\";V[\x03\x81\x830Z\xF1\x90\x81aB`W[PaB.WPa@\xA7a?\x85V[\x91\x84\x92`$\x81Q\x14aA\xBDW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x96\x87\x15\x15\x80aA\xB3W[\x15aA\x18WPP` \x92\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x94\x92aA\r\x92\x87aB\xC9V[P`@Q`\x01\x81R\xA3V[` \x97P\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x95\x93P\x87\x94\x91\x92P\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x92\x16\x98\x89\x93\x84\x87R\x86\x82R`@\x87 \x86_R\x82R`@_ aA\x9F\x82\x82Ta*aV[\x90U`@Q\x90\x81R\xA4P`@Q`\x02\x81R\xA3V[P0\x88\x14\x15a@\xD6V[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x03a@\xB4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P_a@\xB4V[\x95PPPP` \x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x91`@Q\x90\x81R\xA3V[aBm\x91\x96P_\x90a#\x82V[_\x94_a@\x99V[PZ\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[PP\x91\x93\x90\x92Pa)\xC1\x94PaB\xC9V[P\x83;\x15a?\xC7V[PPPPPPPPV[\x90\x83\x15aC\xAFW_\x80`@Q\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x87\x01\x91\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16\x95\x86`$\x82\x01R\x87`D\x82\x01R`D\x81RaC0`d\x82a#\x82V[Q\x90\x82\x85Z\xF1aC>a?\x85V[\x81aC\xB5W[PaC\xAFW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x92\x85_R_\x83R`@_ \x82\x82\x16_R\x83R`@_ aC\xA2\x88\x82Ta*aV[\x90U`@Q\x96\x87R\x16\x94\xA4V[PPPPV[\x80Q\x80\x15\x92P\x82\x15aC\xCAW[PP_aCDV[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x1A\xC0W` aC\xE5\x91\x01a$3V[_\x80aC\xC2V[\x95\x91\x98\x93\x96\x92\x98\x97\x94\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x97aD\x18\x89a#fV[\x16\x87R` \x87\x01R`@Q\x95aD-\x87a\"\xE4V[\x86R` \x86\x01\x96\x87R`@\x86\x01\x97\x88R`@Q\x98aDJ\x8Aa#fV[0\x8AR` \x8A\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x1A\xC0W`@Q\x98\x89\x98\x89\x98\x89\x98\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8AR`\x04\x8A\x01\x90Q\x90aD\xC5\x91` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[Q`D\x89\x01RQ`d\x88\x01R\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x84\x88\x01R` \x01Q`\xA4\x87\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC4\x86\x01R`\xE4\x85\x01Ra\x01\x04\x84\x01a\x01@\x90Ra\x01D\x84\x01aE/\x91a\";V[\x90\x83\x82\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x01a\x01$\x85\x01RaEd\x92a$\xDBV[\x03Z\x92_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81\x95\xF1\x80\x15a\x1BQWaE\x88WPV[_a)\xC1\x91a#\x82V[aF\x96a\x01\xE0\x91a*/`@Q\x93\x84\x92` \x80\x85\x01R`\xFF\x81Q\x16`@\x85\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16``\x85\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`\x80\x85\x01R``\x81\x01Q`\xA0\x85\x01R`\x80\x81\x01Q`\xC0\x85\x01R`\xA0\x81\x01Q`\xE0\x85\x01R`\xC0\x81\x01Qa\x01\0\x85\x01R`\xE0\x81\x01Qa\x01 \x85\x01Ra\x01\0\x81\x01Qa\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01 \x82\x01Q\x16a\x01`\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x82\x01Q\x16a\x01\x80\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x82\x01Q\x16a\x02\0\x85\x01R\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a\";V[\x90V[\x96\x95\x91\x90\x96\x94\x93\x94\x80`\x14R\x81`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x8CZ\xF1\x80`\x01_Q\x14\x16\x15aG\x93W[P_`4Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x84;\x15a\x1A\xC0W_\x96c\xFF\xFF\xFF\xFF\x88\x94aG\x82\x93\x82\x99s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x9D\x8E\x9C\x8D\x9B\x8C\x9A\x7Fw\x9BC-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8CR`\x04\x8C\x01R\x16`$\x8A\x01R\x82`D\x8A\x01R\x16`d\x88\x01R`\x84\x87\x01R`\xA4\x86\x01R\x16`\xC4\x84\x01Ra\x01\0`\xE4\x84\x01Ra\x01\x04\x83\x01\x90a\";V[\x03\x92Z\xF1\x80\x15a\x1BQWaE\x88WPV[=\x89;\x15\x17\x10\x15aG\xA5W[_aF\xD4V[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x8CZ\xF1P\x81`4R` _`D`\x10\x82\x8CZ\xF1\x80`\x01_Q\x14\x16\x15aG\xE9W[PaG\x9FV[=\x89;\x15\x17\x10\x15aG\xFAW_aG\xE3V[c>?\x8Fs_R`\x04`\x1C\xFD\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f803560e01c806316c38b3c146120ac5780631e83409a14611f6f5780632569296214611f0857806331eee44d14611db157806339c3321514611d7657806354d1f13d14611d145780635778472a14611c375780635c975abb14611bf45780635fdc7c12146117af578063681143701461124c5780636afdd85014611204578063715018a614611165578063776ff3c714610e7357806377839a9e14610e3857806379502c5514610dc957806385c1783014610d8d5780638cda96de14610d125780638da5cb5b14610ca1578063928c60b914610a5857806397c36bae14610990578063ac4fca821461089e578063ac9650d814610744578063cc6eec7014610709578063d4570c1c14610691578063dcb41c2814610622578063eafa61a814610363578063f04e283e146102f7578063f2fde38b1461029a578063f3995c67146101b75763fee81cf414610164575f80fd5b346101b45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45761019b61212d565b9063389a75e1600c5252602080600c2054604051908152f35b80fd5b50346101b45760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457806101f061212d565b6064359060ff82168092036102965773ffffffffffffffffffffffffffffffffffffffff1690813b1561029657829160e4839260405195869384927fd505accf00000000000000000000000000000000000000000000000000000000845233600485015230602485015260243560448501526044356064850152608484015260843560a484015260a43560c48401525af16102885780f35b61029191612382565b5f8180f35b5050fd5b5060207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576102cd61212d565b6102d5612553565b8060601b156102ea576102e790613c21565b80f35b637448fbae82526004601cfd5b5060207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45761032a61212d565b610332612553565b63389a75e1600c528082526020600c20805442116103565790826102e79255613c21565b636f5e881883526004601cfd5b50346101b4576101407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45761039c6121d8565b604435906084356064356103ae6121eb565b926103b761220f565b94610124359167ffffffffffffffff831161061e57826004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc853603011261061a5760ff60035460401c166105f257828510156105c25761041c8584612519565b861015610588576104789160246104506104719361043861311a565b9b8c9161010435918c8c8b60e4359489359033613171565b9567ffffffffffffffff610463846122cf565b166101c0880152019061227e565b36916123fd565b6101e083015261048782613404565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa90811561057d57606061052e939273ffffffffffffffffffffffffffffffffffffffff926105339a9b9161054e575b5001511630903390612930565b6136d2565b6040805191825267ffffffffffffffff929092166020820152f35b610570915060a03d60a011610576575b6105688183612382565b810190612472565b5f610521565b503d61055e565b6040513d8a823e3d90fd5b604489876105968887612519565b907f1cde3111000000000000000000000000000000000000000000000000000000008352600452602452fd5b60448984877fed2bc1ea000000000000000000000000000000000000000000000000000000008352600452602452fd5b6004897f9e87fac8000000000000000000000000000000000000000000000000000000008152fd5b8880fd5b8780fd5b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101b45760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45773ffffffffffffffffffffffffffffffffffffffff60406106e061212d565b92826106ea612150565b94168152806020522091165f52602052602060405f2054604051908152f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760206040516103e88152f35b5060207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576004359067ffffffffffffffff82116101b457366023830112156101b45781600401359167ffffffffffffffff831161089a578260051b90366024838301011161089657346108965760405193849360208552818560200152604085810194806024860187378601019384926107fe575b50505050806040520360401b178060401c9067ffffffffffffffff16f35b92945090925b8092826044825188016024810135918291018537389084305af41561088d57602067ffffffffffffffe0603f85937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08b87030181528301943d81523d858583013e3d010116943d010152858482101561087e575091610804565b9450505090505f8080806107e0565b863d84823e3d90fd5b8280fd5b5080fd5b50346101b45760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576108d661212d565b6108de612150565b9060843567ffffffffffffffff811161098c576108ff9036906004016121aa565b9091303303610964579161091c916104718694866044359161258a565b916040519238918360208351930191606435f13d60243d1161095c575b80602091845280858386013e83010160405215610954575080f35b805190602001fd5b506024610939565b6004857f14d4a4e8000000000000000000000000000000000000000000000000000000008152fd5b8380fd5b50346101b45760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760043567ffffffffffffffff811161089a576102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc823603011261089a5780610a4b610a44610104604094013592610a1c61014482016122cf565b610a2961016483016122cf565b600254916101846101a4850135940135916024359188613ca2565b8092612519565b9082519182526020820152f35b50346101b45760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457610a906121d8565b906044359173ffffffffffffffffffffffffffffffffffffffff831680930361089a5760843567ffffffffffffffff811161089657610ad39036906004016121aa565b9093688000000000ab143c065c610c94576001688000000000ab143c065d73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163303610c68576040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115610c5d5773ffffffffffffffffffffffffffffffffffffffff916060918791610c3e575b500151168103610c135750610be6610be1610c049495610bf19336916123fd565b612af7565b809260243590612c7f565b60643590610bfe81612db8565b90612f83565b80688000000000ab143c065d80f35b7f2e775c7c000000000000000000000000000000000000000000000000000000008452600452602483fd5b610c57915060a03d60a011610576576105688183612382565b5f610bc0565b6040513d87823e3d90fd5b6024847f50ce3ed900000000000000000000000000000000000000000000000000000000815233600452fd5b63ab143c0684526004601cfd5b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b50346101b45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457600435610d4d612553565b670de0b6b3a76400008111610d625760025580f35b7fad6bb6d1000000000000000000000000000000000000000000000000000000008252600452602490fd5b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576020600254604051908152f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760206040516107d08152f35b50346101b45760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576004359067ffffffffffffffff82116101b457816004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc843603011261089a57610eef612150565b926044359067ffffffffffffffff821161098c57816004019460607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc843603011261116157688000000000ab143c065c611154576001688000000000ab143c065d60ff60035460401c1661112c57610f67818561272b565b9783859b839895969960405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152610fb0606082612382565b51902060405190610fc260a083612382565b606a82527f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c417560208301527f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b6560408301527f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e743260608301527f353620616d6f756e742900000000000000000000000000000000000000000000608083015261106b604484018561227e565b949093602401359035888d6101048d013590611086996143ec565b6101e484016110949161227e565b936101c4016110a2906122cf565b9336906110ae926123fd565b916110b99488612997565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a3688000000000ab143c065d604051908152602090f35b6004857f9e87fac8000000000000000000000000000000000000000000000000000000008152fd5b63ab143c0685526004601cfd5b8480fd5b50807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b457611197612553565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275580f35b50346101b457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b45760206040516e22d473030f116ddee9f6b43ac78ba38152f35b50346101b4576101807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101b4576112856121d8565b6044356112906121eb565b9061129961220f565b90610124359167ffffffffffffffff83116117ab57826004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc85360301126117a757610144359273ffffffffffffffffffffffffffffffffffffffff8416840361061e5767ffffffffffffffff610164351161061e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc610164353603011261061e5760ff60035460401c1661177f5780606435101561174e5761136560643582612519565b6084351015611710579160246104506113a1936113b09561138461311a565b9a8b92610104359260e43592608435918c606435928a3591613171565b91906101e085019236916123fd565b81526113bb83613404565b6040516020810190606435825263ffffffff861660408201526084356060820152606081526113eb608082612382565b5190209060ff8451169163ffffffff60408601511691608086015160e087015161010088015167ffffffffffffffff6101608a01511667ffffffffffffffff6101408b01511690039167ffffffffffffffff83116116e357936105339b9c969367ffffffffffffffff8b947f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b98946116d69c9b9861018088015194846101c06101a08b01519851602081519101209a015116996040519b60208d019d8e5260408d015260608c015260808b015260a08a015260c08901521660e087015261010086015261012085015261014084015261016083015261018082015261018081526114f76101a082612382565b5190209161150860a0860151613f39565b9260e08601519250610160916115216040519384612382565b61012983527f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208401527f75696e743820627269646765547970652c75696e74333220647374436861696e60408401527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608401527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808401527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08401527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08401527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08401527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008401527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208401527f3620616d6f756e742900000000000000000000000000000000000000000000006101408401526116bb604461016435016101643560040161227e565b959094806024610164350135936101643560040135936143ec565b60843590606435906136d2565b60248d7f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b8761172060449260643590612519565b7f1cde3111000000000000000000000000000000000000000000000000000000008252608435600452602452fd5b7fed2bc1ea000000000000000000000000000000000000000000000000000000008852606435600452602452604487fd5b6004887f9e87fac8000000000000000000000000000000000000000000000000000000008152fd5b8680fd5b8580fd5b34611ac05760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05760043567ffffffffffffffff8111611ac0576117fe9036906004016121aa565b60243567ffffffffffffffff8111611ac05761181e9036906004016121aa565b688000000000ab143c065c611be7576001688000000000ab143c065d60ff60035460401c16611bbf57604051907f0a70b05600000000000000000000000000000000000000000000000000000000825246600483015260a08260248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa918215611b515773ffffffffffffffffffffffffffffffffffffffff60806004946020935f91611ba0575b50015116604051938480927f2c1219210000000000000000000000000000000000000000000000000000000082525afa918215611b51575f92611b5c575b50916020916119ad935f73ffffffffffffffffffffffffffffffffffffffff6040518097819682957f57ecfd280000000000000000000000000000000000000000000000000000000084526040600485015261197d8d8d60448701916124db565b917ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8584030160248601526124db565b0393165af1908115611b51575f91611b17575b5015611aef576101788110611ac45780600811611ac0578060d811611ac05760209160b88101358260f811611ac05760d8820135928061011811611ac05760f8830135918161015811611ac05761013884013595508161017811611ac0577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe886101788501920190308103611a9557611a876004611a7d8989611a788a8a611a6b610be1368c8e6123fd565b968792013560e01c612c7f565b612519565b90610bfe81612db8565b5f688000000000ab143c065d005b7fc7286ea1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5f80fd5b7fa2abf1b6000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f514d840a000000000000000000000000000000000000000000000000000000005f5260045ffd5b90506020813d602011611b49575b81611b3260209383612382565b81010312611ac057611b4390612433565b836119c0565b3d9150611b25565b6040513d5f823e3d90fd5b9150916020823d602011611b98575b81611b7860209383612382565b81010312611ac0576119ad92611b8f602093612451565b9250909261191c565b3d9150611b6b565b611bb9915060a03d60a011610576576105688183612382565b896118de565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b34611ac0575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac057602060ff60035460401c166040519015158152f35b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac0575f60408051611c74816122e4565b82815282602082015201526004355f526001602052606060405f2064ffffffffff60405191611ca2836122e4565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c166020840190611cd181612173565b815282604085019260a81c16825273ffffffffffffffffffffffffffffffffffffffff604051945116845251611d0681612173565b602084015251166040820152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b34611ac0575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac0576020604051624c4b408152f35b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05760043567ffffffffffffffff8111611ac057806004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8336030112611ac057688000000000ab143c065c611be7576001688000000000ab143c065d60ff60035460401c16611bbf57611eb8611eaf917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d611ef1611ec0611ec76101c4602098611e8d338861272b565b9b849d829993949692959c611ea48730338b612930565b6101e484019061227e565b979092016122cf565b9536916123fd565b9289612997565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d604051908152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac05763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac057611fa661212d565b688000000000ab143c065c611be7576001688000000000ab143c065d335f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5260205260405f2054801561208457602091335f525f835260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5283525f604081205561202d82338361258a565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd3992683843392a35f688000000000ab143c065d604051908152f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b34611ac05760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112611ac057600435801515809103611ac0576120f0612553565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff00000000000000006003549260401b169116176003555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff82168203611ac057565b6024359073ffffffffffffffffffffffffffffffffffffffff82168203611ac057565b6003111561217d57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b9181601f84011215611ac05782359167ffffffffffffffff8311611ac05760208381860195010111611ac057565b6004359063ffffffff82168203611ac057565b60a4359063ffffffff82168203611ac057565b359063ffffffff82168203611ac057565b60c4359067ffffffffffffffff82168203611ac057565b359067ffffffffffffffff82168203611ac057565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215611ac0570180359067ffffffffffffffff8211611ac057602001918136038313611ac057565b3567ffffffffffffffff81168103611ac05790565b6060810190811067ffffffffffffffff82111761230057604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60a0810190811067ffffffffffffffff82111761230057604052565b610200810190811067ffffffffffffffff82111761230057604052565b6040810190811067ffffffffffffffff82111761230057604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761230057604052565b67ffffffffffffffff811161230057601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b929192612409826123c3565b916124176040519384612382565b829481845281830111611ac0578281602093845f960137010152565b51908115158203611ac057565b519063ffffffff82168203611ac057565b519073ffffffffffffffffffffffffffffffffffffffff82168203611ac057565b908160a0910312611ac0576124d360806040519261248f8461232d565b61249881612433565b84526124a660208201612440565b60208501526124b760408201612440565b60408501526124c860608201612451565b606085015201612451565b608082015290565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b9190820391821161252657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392754330361257d57565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f511416156125c6575b50505f603452565b3b153d1710156125d7575f806125be565b6390b8ec185f526004601cfd5b3560ff81168103611ac05790565b3563ffffffff81168103611ac05790565b919061020083820312611ac0576040519061261d82612349565b8193803560ff81168103611ac0578352612639602082016121fe565b602084015261264a604082016121fe565b6040840152606081013560608401526080810135608084015260a081013560a084015260c081013560c084015260e081013560e084015261010081013561010084015261269a6101208201612226565b6101208401526126ad6101408201612226565b6101408401526126c06101608201612226565b6101608401526101808101356101808401526101a08101356101a08401526126eb6101c08201612226565b6101c08401526101e08101359067ffffffffffffffff8211611ac0570181601f82011215611ac0576101e091816020612726933591016123fd565b910152565b9060ff612737836125e4565b166128f5576040820163ffffffff61274e826125f2565b1646036128bb57506127686127633684612603565b612db8565b92835f52600160205260405f209283549160ff8360a01c1661278981612173565b61288f5761279f61279a3684612603565b613e4a565b946127ad6080840135613f39565b947fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff7401000000000000000000000000000000000000000061282c612825610100880135976127ff61014082016122cf565b61280c61016083016122cf565b600254916101806101a08501359401359142918d613ca2565b8097612519565b967fffffffffffffffffffffff00000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff79ffffffffff0000000000000000000000000000000000000000004260a81b1695169116171716179055565b857f343e211e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6128c963ffffffff916125f2565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b60ff612900836125e4565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f525f6004521660245260445ffd5b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f51141615612979575b50505f606052604052565b3b153d17101561298a575f8061296e565b637939f4245f526004601cfd5b916129c195939181958051155f146129c357506040516129b8602082612382565b5f815293613fb4565b565b612a5b612a2f916040519283917f3866c1dc00000000000000000000000000000000000000000000000000000000602084015288602484015273ffffffffffffffffffffffffffffffffffffffff861660448401528760648401526080608484015260a483019061223b565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282612382565b93613fb4565b9190820180921161252657565b60405190612a7b82612349565b60606101e0835f81525f60208201525f60408201525f838201525f60808201525f60a08201525f60c08201525f60e08201525f6101008201525f6101208201525f6101408201525f6101608201525f6101808201525f6101a08201525f6101c08201520152565b519067ffffffffffffffff82168203611ac057565b612aff612a6e565b5080518101906020820190602081840312611ac057602081015167ffffffffffffffff8111611ac05701916102009083900312611ac05760405191612b4383612349565b602081015160ff81168103611ac0578352612b6060408201612440565b6020840152612b7160608201612440565b60408401526080810151606084015260a0810151608084015260c081015160a084015260e081015160c084015261010081015160e0840152610120810151610100840152612bc26101408201612ae2565b610120840152612bd56101608201612ae2565b610140840152612be86101808201612ae2565b6101608401526101a08101516101808401526101c08101516101a0840152612c136101e08201612ae2565b6101c084015261020081015167ffffffffffffffff8111611ac057602091010181601f82011215611ac057805190612c4a826123c3565b92612c586040519485612382565b82845260208383010111611ac057815f9260208093018386015e830101526101e082015290565b9091602063ffffffff91015116604051907f0a70b056000000000000000000000000000000000000000000000000000000008252600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611b51575f91612d99575b50805115908115612d7e575b50612d4c5750308103612d215750565b7fc21fa2e5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b63ffffffff907f6a96659e000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b63ffffffff9150602001511663ffffffff821614155f612d11565b612db2915060a03d60a011610576576105688183612382565b5f612d05565b906101e08201805151601f81018091116125265760051c90816012018060121161252657601360405193849283520160051b0160405267ffffffffffffffff6101c060208401956020875260ff815116604086015263ffffffff602082015116606086015263ffffffff6040820151166080860152606081015160a0860152608081015160c086015260a081015160e086015260c081015161010086015260e08101516101208601526101008101516101408601528261012082015116610160860152826101408201511661018086015282610160820151166101a0860152610180810151828601526101a08101516101e086015201511661020083015261020061022083015280515161024083015251805180612ef3575b50508051928360051b902092604051828260010160051b011490151060061b52565b602082016020826102608601940101905b818110612f73575050601f16908115612ed15760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80612ed1565b8051845260209384019301612f04565b91815f52600160205260405f208054600260ff8260a01c16612fa481612173565b146130cc57917fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff6040927f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a94612ff988613e4a565b9161300760808a0151613f39565b9073ffffffffffffffffffffffffffffffffffffffff8316996101008101518088105f146130c6575086915b74020000000000000000000000000000000000000000613053848a612519565b97889616179055868b15155f1461309b575050613072908a858b6142c9565b81613089575b5050505b82519182526020820152a3565b61309392886142c9565b5f8181613078565b9150916130c194935067ffffffffffffffff6101c06101e085015194015116938a612997565b61307c565b91613033565b837fb196a44a000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9067ffffffffffffffff8091169116019067ffffffffffffffff821161252657565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff81116125265767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b9594989390929192613181612a6e565b506040517f0a70b0560000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff169a9060a0816024818f5afa8015611b51576060915f916133e5575b50015173ffffffffffffffffffffffffffffffffffffffff16916040518080927f0a70b05600000000000000000000000000000000000000000000000000000000825263ffffffff169d8e60048301525a9260249160a094fa908115611b515773ffffffffffffffffffffffffffffffffffffffff916060915f916133c6575b500151169282156133945783156133685761329b9085612519565b906132a591612519565b934267ffffffffffffffff16966132bc90886130f8565b976040519b6132ca8d612349565b5f8d528c4663ffffffff16906020015260408d015273ffffffffffffffffffffffffffffffffffffffff1660608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015261014085015267ffffffffffffffff166101608401526101808301526101a08201526101c081015f905260405161335c602082612382565b5f81526101e082015290565b8b7fb825dd76000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff461660045260245ffd5b6133df915060a03d60a011610576576105688183612382565b5f613280565b6133fe915060a03d60a011610576576105688183612382565b5f613200565b63ffffffff6020820151164681036136a7575063ffffffff6040820151166040517f0a70b05600000000000000000000000000000000000000000000000000000000815281600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa908115611b51575f91613688575b50805115908115613665575b5061363a575067ffffffffffffffff6101608201511667ffffffffffffffff610140830151168082111561360c57505061010081018051801580156135ff575b6135cb57506101a08201519051908181101561359d57505073ffffffffffffffffffffffffffffffffffffffff6135216080830151613f39565b1615613575576101c0015167ffffffffffffffff16624c4b4081116135435750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f52600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e0830151907f8dd09d91000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5060e083015181116134e7565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff91506060015116155f6134a7565b6136a1915060a03d60a011610576576105688183612382565b5f61349b565b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b909391936136df82612db8565b9373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016916040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a081602481875afa908115611b51575f91613c02575b506080810173ffffffffffffffffffffffffffffffffffffffff8151166004602063ffffffff818601511692604051928380927f2c1219210000000000000000000000000000000000000000000000000000000082525afa908115611b51575f91613bb0575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f8d3638f4000000000000000000000000000000000000000000000000000000008352165afa8015611b51575f90613b73575b63ffffffff91501690808203613b45575050604086019460a063ffffffff8751166024604051809481937f0a70b05600000000000000000000000000000000000000000000000000000000835260048301525afa908115611b515763ffffffff916020915f91613b26575b5001511698806139465750879860a09473ffffffffffffffffffffffffffffffffffffffff947f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a9794866060816138f3979e9c9d9e5116920151169360e08c01516138eb8d614592565b953093614699565b63ffffffff6139056060880151613f39565b9151169560c08101519067ffffffffffffffff6101206101008301519201511691604051985f8a5260208a01526040890152606088015260808701521693a3565b6205573067ffffffffffffffff6101c0899795979694960151160167ffffffffffffffff81116125265767ffffffffffffffff1699624c4b408b11613af357899a6080899a9b990151926139998b614592565b604051916139a68361232d565b825260208201973089526040830193845260608301958652608083019182525173ffffffffffffffffffffffffffffffffffffffff16986060015173ffffffffffffffffffffffffffffffffffffffff169460e08d015193604051998a9460208601602090525160408601525160608501525167ffffffffffffffff1660808401525160a08301525160c0820160a0905260e08201613a449161223b565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081018752613a749087612382565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1692613ab597614699565b60a073ffffffffffffffffffffffffffffffffffffffff7f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a926138f3565b8a7f25ad8594000000000000000000000000000000000000000000000000000000005f52600452624c4b4060245260445ffd5b613b3f915060a03d60a011610576576105688183612382565b5f613881565b7fc9e030e8000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b506020813d602011613ba8575b81613b8d60209383612382565b81010312611ac057613ba363ffffffff91612440565b613816565b3d9150613b80565b90506020813d602011613bfa575b81613bcb60209383612382565b81010312611ac0576020613bf373ffffffffffffffffffffffffffffffffffffffff92612451565b91506137c1565b3d9150613bbe565b613c1b915060a03d60a011610576576105688183612382565b5f61375b565b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b959290939167ffffffffffffffff8091169416908181105f14613e4357505b5f93808210613ce9575b505050613cd89250612a61565b81811115613ce4575090565b905090565b613cf592939450612519565b90801580159081613de5575b5015613dde575050815b808311613dd6575b5081830291670de0b6b3a764000084158286860414170215613d485750670de0b6b3a7640000613cd892045b905f8080613ccb565b91670de0b6b3a76400007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8486098281108301900393850983670de0b6b3a76400001115613dc957613cd893828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066902613d3f565b63ae47f7025f526004601cfd5b91505f613d13565b0291613d0b565b9050613e1657807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0482115f613d01565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b9050613cc1565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a08160248173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000165afa8015611b5157606073ffffffffffffffffffffffffffffffffffffffff9160c0935f91613f1a575b50015116910151818103613eef575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b613f33915060a03d60a011610576576105688183612382565b5f613edd565b8060a01c613f5a5773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3d15613faf573d90613f96826123c3565b91613fa46040519384612382565b82523d5f602084013e565b606090565b945f9484156142bf5780511580156142b6575b6142a5575a61c35067ffffffffffffffff613ff3818616956707ffffffffffffff8160051c16906130f8565b160167ffffffffffffffff81116125265767ffffffffffffffff161161427557303b15611ac0575f61408c91604051809381927fac4fca8200000000000000000000000000000000000000000000000000000000835273ffffffffffffffffffffffffffffffffffffffff808916998a600486015216968760248501528a6044850152606484015260a0608484015260a483019061223b565b038183305af19081614260575b5061422e57506140a7613f85565b91849260248151146141bd575b5073ffffffffffffffffffffffffffffffffffffffff831696871515806141b3575b156141185750506020927f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9949261410d92876142c9565b5060405160018152a3565b602097507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a995935087949192507f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f73ffffffffffffffffffffffffffffffffffffffff88921698899384875286825260408720865f52825260405f2061419f828254612a61565b9055604051908152a45060405160028152a3565b50308814156140d6565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff00000000000000000000000000000000000000000000000000000000602460208401519301519216036140b45773ffffffffffffffffffffffffffffffffffffffff1692505f6140b4565b955050505060207f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a991604051908152a3565b61426d9196505f90612382565b5f945f614099565b505a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b505091939092506129c194506142c9565b50833b15613fc7565b5050505050505050565b9083156143af575f806040519473ffffffffffffffffffffffffffffffffffffffff60208701917fa9059cbb000000000000000000000000000000000000000000000000000000008352169586602482015287604482015260448152614330606482612382565b519082855af161433e613f85565b816143b5575b506143af57602073ffffffffffffffffffffffffffffffffffffffff7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f92855f525f835260405f208282165f52835260405f206143a2888254612a61565b90556040519687521694a4565b50505050565b80518015925082156143ca575b50505f614344565b8192509060209181010312611ac05760206143e59101612433565b5f806143c2565b9591989396929897949773ffffffffffffffffffffffffffffffffffffffff6040519761441889612366565b16875260208701526040519561442d876122e4565b865260208601968752604086019788526040519861444a8a612366565b308a5260208a01526e22d473030f116ddee9f6b43ac78ba33b15611ac057604051988998899889987f137c29fe000000000000000000000000000000000000000000000000000000008a5260048a019051906144c5916020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b516044890152516064880152805173ffffffffffffffffffffffffffffffffffffffff1660848801526020015160a487015273ffffffffffffffffffffffffffffffffffffffff1660c486015260e485015261010484016101409052610144840161452f9161223b565b908382037ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc01610124850152614564926124db565b035a925f6e22d473030f116ddee9f6b43ac78ba38195f18015611b51576145885750565b5f6129c191612382565b6146966101e091612a2f60405193849260208085015260ff815116604085015263ffffffff602082015116606085015263ffffffff6040820151166080850152606081015160a0850152608081015160c085015260a081015160e085015260c081015161010085015260e081015161012085015261010081015161014085015267ffffffffffffffff6101208201511661016085015267ffffffffffffffff6101408201511661018085015267ffffffffffffffff610160820151166101a08501526101808101516101c08501526101a08101518285015267ffffffffffffffff6101c082015116610200850152015161020061022084015261024083019061223b565b90565b969591909694939480601452816034526f095ea7b30000000000000000000000005f5260205f60446010828c5af18060015f51141615614793575b505f60345273ffffffffffffffffffffffffffffffffffffffff1693843b15611ac0575f9663ffffffff889461478293829973ffffffffffffffffffffffffffffffffffffffff6040519d8e9c8d9b8c9a7f779b432d000000000000000000000000000000000000000000000000000000008c5260048c01521660248a01528260448a0152166064880152608487015260a48601521660c484015261010060e484015261010483019061223b565b03925af18015611b51576145885750565b3d893b151710156147a5575b5f6146d4565b5f6034526f095ea7b30000000000000000000000005f525f3860446010838c5af1508160345260205f60446010828c5af18060015f511416156147e9575b5061479f565b3d893b151710156147fa575f6147e3565b633e3f8f735f526004601cfdfea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_\x805`\xE0\x1C\x80c\x16\xC3\x8B<\x14a \xACW\x80c\x1E\x83@\x9A\x14a\x1FoW\x80c%i)b\x14a\x1F\x08W\x80c1\xEE\xE4M\x14a\x1D\xB1W\x80c9\xC32\x15\x14a\x1DvW\x80cT\xD1\xF1=\x14a\x1D\x14W\x80cWxG*\x14a\x1C7W\x80c\\\x97Z\xBB\x14a\x1B\xF4W\x80c_\xDC|\x12\x14a\x17\xAFW\x80ch\x11Cp\x14a\x12LW\x80cj\xFD\xD8P\x14a\x12\x04W\x80cqP\x18\xA6\x14a\x11eW\x80cwo\xF3\xC7\x14a\x0EsW\x80cw\x83\x9A\x9E\x14a\x0E8W\x80cyP,U\x14a\r\xC9W\x80c\x85\xC1x0\x14a\r\x8DW\x80c\x8C\xDA\x96\xDE\x14a\r\x12W\x80c\x8D\xA5\xCB[\x14a\x0C\xA1W\x80c\x92\x8C`\xB9\x14a\nXW\x80c\x97\xC3k\xAE\x14a\t\x90W\x80c\xACO\xCA\x82\x14a\x08\x9EW\x80c\xAC\x96P\xD8\x14a\x07DW\x80c\xCCn\xECp\x14a\x07\tW\x80c\xD4W\x0C\x1C\x14a\x06\x91W\x80c\xDC\xB4\x1C(\x14a\x06\"W\x80c\xEA\xFAa\xA8\x14a\x03cW\x80c\xF0N(>\x14a\x02\xF7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9AW\x80c\xF3\x99\\g\x14a\x01\xB7Wc\xFE\xE8\x1C\xF4\x14a\x01dW_\x80\xFD[4a\x01\xB4W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x01\x9Ba!-V[\x90c8\x9Au\xE1`\x0CRR` \x80`\x0C T`@Q\x90\x81R\xF3[\x80\xFD[P4a\x01\xB4W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W\x80a\x01\xF0a!-V[`d5\x90`\xFF\x82\x16\x80\x92\x03a\x02\x96Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81;\x15a\x02\x96W\x82\x91`\xE4\x83\x92`@Q\x95\x86\x93\x84\x92\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R3`\x04\x85\x01R0`$\x85\x01R`$5`D\x85\x01R`D5`d\x85\x01R`\x84\x84\x01R`\x845`\xA4\x84\x01R`\xA45`\xC4\x84\x01RZ\xF1a\x02\x88W\x80\xF3[a\x02\x91\x91a#\x82V[_\x81\x80\xF3[PP\xFD[P` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x02\xCDa!-V[a\x02\xD5a%SV[\x80``\x1B\x15a\x02\xEAWa\x02\xE7\x90a<!V[\x80\xF3[ctH\xFB\xAE\x82R`\x04`\x1C\xFD[P` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x03*a!-V[a\x032a%SV[c8\x9Au\xE1`\x0CR\x80\x82R` `\x0C \x80TB\x11a\x03VW\x90\x82a\x02\xE7\x92Ua<!V[co^\x88\x18\x83R`\x04`\x1C\xFD[P4a\x01\xB4Wa\x01@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x03\x9Ca!\xD8V[`D5\x90`\x845`d5a\x03\xAEa!\xEBV[\x92a\x03\xB7a\"\x0FV[\x94a\x01$5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x06\x1EW\x82`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x856\x03\x01\x12a\x06\x1AW`\xFF`\x03T`@\x1C\x16a\x05\xF2W\x82\x85\x10\x15a\x05\xC2Wa\x04\x1C\x85\x84a%\x19V[\x86\x10\x15a\x05\x88Wa\x04x\x91`$a\x04Pa\x04q\x93a\x048a1\x1AV[\x9B\x8C\x91a\x01\x045\x91\x8C\x8C\x8B`\xE45\x94\x895\x903a1qV[\x95g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x04c\x84a\"\xCFV[\x16a\x01\xC0\x88\x01R\x01\x90a\"~V[6\x91a#\xFDV[a\x01\xE0\x83\x01Ra\x04\x87\x82a4\x04V[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x05}W``a\x05.\x93\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a\x053\x9A\x9B\x91a\x05NW[P\x01Q\x160\x903\x90a)0V[a6\xD2V[`@\x80Q\x91\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x82\x01R\xF3[a\x05p\x91P`\xA0=`\xA0\x11a\x05vW[a\x05h\x81\x83a#\x82V[\x81\x01\x90a$rV[_a\x05!V[P=a\x05^V[`@Q=\x8A\x82>=\x90\xFD[`D\x89\x87a\x05\x96\x88\x87a%\x19V[\x90\x7F\x1C\xDE1\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04R`$R\xFD[`D\x89\x84\x87\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04R`$R\xFD[`\x04\x89\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x88\x80\xFD[\x87\x80\xFD[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\xB4W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@a\x06\xE0a!-V[\x92\x82a\x06\xEAa!PV[\x94\x16\x81R\x80` R \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qa\x03\xE8\x81R\xF3[P` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xB4W6`#\x83\x01\x12\x15a\x01\xB4W\x81`\x04\x015\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x08\x9AW\x82`\x05\x1B\x906`$\x83\x83\x01\x01\x11a\x08\x96W4a\x08\x96W`@Q\x93\x84\x93` \x85R\x81\x85` \x01R`@\x85\x81\x01\x94\x80`$\x86\x01\x877\x86\x01\x01\x93\x84\x92a\x07\xFEW[PPPP\x80`@R\x03`@\x1B\x17\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[\x92\x94P\x90\x92[\x80\x92\x82`D\x82Q\x88\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x08\x8DW` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x85\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8B\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x94=\x01\x01R\x85\x84\x82\x10\x15a\x08~WP\x91a\x08\x04V[\x94PPP\x90P_\x80\x80\x80a\x07\xE0V[\x86=\x84\x82>=\x90\xFD[\x82\x80\xFD[P\x80\xFD[P4a\x01\xB4W`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x08\xD6a!-V[a\x08\xDEa!PV[\x90`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t\x8CWa\x08\xFF\x906\x90`\x04\x01a!\xAAV[\x90\x9103\x03a\tdW\x91a\t\x1C\x91a\x04q\x86\x94\x86`D5\x91a%\x8AV[\x91`@Q\x928\x91\x83` \x83Q\x93\x01\x91`d5\xF1=`$=\x11a\t\\W[\x80` \x91\x84R\x80\x85\x83\x86\x01>\x83\x01\x01`@R\x15a\tTWP\x80\xF3[\x80Q\x90` \x01\xFD[P`$a\t9V[`\x04\x85\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x83\x80\xFD[P4a\x01\xB4W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\x9AWa\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x826\x03\x01\x12a\x08\x9AW\x80a\nKa\nDa\x01\x04`@\x94\x015\x92a\n\x1Ca\x01D\x82\x01a\"\xCFV[a\n)a\x01d\x83\x01a\"\xCFV[`\x02T\x91a\x01\x84a\x01\xA4\x85\x015\x94\x015\x91`$5\x91\x88a<\xA2V[\x80\x92a%\x19V[\x90\x82Q\x91\x82R` \x82\x01R\xF3[P4a\x01\xB4W`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\n\x90a!\xD8V[\x90`D5\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x08\x9AW`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\x96Wa\n\xD3\x906\x90`\x04\x01a!\xAAV[\x90\x93h\x80\0\0\0\0\xAB\x14<\x06\\a\x0C\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x0ChW`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x0C]Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91``\x91\x87\x91a\x0C>W[P\x01Q\x16\x81\x03a\x0C\x13WPa\x0B\xE6a\x0B\xE1a\x0C\x04\x94\x95a\x0B\xF1\x936\x91a#\xFDV[a*\xF7V[\x80\x92`$5\x90a,\x7FV[`d5\x90a\x0B\xFE\x81a-\xB8V[\x90a/\x83V[\x80h\x80\0\0\0\0\xAB\x14<\x06]\x80\xF3[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04R`$\x83\xFD[a\x0CW\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a\x0B\xC0V[`@Q=\x87\x82>=\x90\xFD[`$\x84\x7FP\xCE>\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04R\xFD[c\xAB\x14<\x06\x84R`\x04`\x1C\xFD[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[P4a\x01\xB4W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045a\rMa%SV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\rbW`\x02U\x80\xF3[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04R`$\x90\xFD[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `\x02T`@Q\x90\x81R\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qa\x07\xD0\x81R\xF3[P4a\x01\xB4W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xB4W\x81`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x08\x9AWa\x0E\xEFa!PV[\x92`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\t\x8CW\x81`\x04\x01\x94``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x11aWh\x80\0\0\0\0\xAB\x14<\x06\\a\x11TW`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x11,Wa\x0Fg\x81\x85a'+V[\x97\x83\x85\x9B\x83\x98\x95\x96\x99`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\x0F\xB0``\x82a#\x82V[Q\x90 `@Q\x90a\x0F\xC2`\xA0\x83a#\x82V[`j\x82R\x7FFillAuthorization witness)FillAu` \x83\x01R\x7Fthorization(bytes32 orderId)Toke`@\x83\x01R\x7FnPermissions(address token,uint2``\x83\x01R\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x83\x01Ra\x10k`D\x84\x01\x85a\"~V[\x94\x90\x93`$\x015\x905\x88\x8Da\x01\x04\x8D\x015\x90a\x10\x86\x99aC\xECV[a\x01\xE4\x84\x01a\x10\x94\x91a\"~V[\x93a\x01\xC4\x01a\x10\xA2\x90a\"\xCFV[\x936\x90a\x10\xAE\x92a#\xFDV[\x91a\x10\xB9\x94\x88a)\x97V[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[`\x04\x85\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[c\xAB\x14<\x06\x85R`\x04`\x1C\xFD[\x84\x80\xFD[P\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x11\x97a%SV[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\x80\xF3[P4a\x01\xB4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4W` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[P4a\x01\xB4Wa\x01\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xB4Wa\x12\x85a!\xD8V[`D5a\x12\x90a!\xEBV[\x90a\x12\x99a\"\x0FV[\x90a\x01$5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x17\xABW\x82`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x856\x03\x01\x12a\x17\xA7Wa\x01D5\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x84\x03a\x06\x1EWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01d5\x11a\x06\x1EW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFCa\x01d56\x03\x01\x12a\x06\x1EW`\xFF`\x03T`@\x1C\x16a\x17\x7FW\x80`d5\x10\x15a\x17NWa\x13e`d5\x82a%\x19V[`\x845\x10\x15a\x17\x10W\x91`$a\x04Pa\x13\xA1\x93a\x13\xB0\x95a\x13\x84a1\x1AV[\x9A\x8B\x92a\x01\x045\x92`\xE45\x92`\x845\x91\x8C`d5\x92\x8A5\x91a1qV[\x91\x90a\x01\xE0\x85\x01\x926\x91a#\xFDV[\x81Ra\x13\xBB\x83a4\x04V[`@Q` \x81\x01\x90`d5\x82Rc\xFF\xFF\xFF\xFF\x86\x16`@\x82\x01R`\x845``\x82\x01R``\x81Ra\x13\xEB`\x80\x82a#\x82V[Q\x90 \x90`\xFF\x84Q\x16\x91c\xFF\xFF\xFF\xFF`@\x86\x01Q\x16\x91`\x80\x86\x01Q`\xE0\x87\x01Qa\x01\0\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x8A\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x8B\x01Q\x16\x90\x03\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x16\xE3W\x93a\x053\x9B\x9C\x96\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x94\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x98\x94a\x16\xD6\x9C\x9B\x98a\x01\x80\x88\x01Q\x94\x84a\x01\xC0a\x01\xA0\x8B\x01Q\x98Q` \x81Q\x91\x01 \x9A\x01Q\x16\x99`@Q\x9B` \x8D\x01\x9D\x8ER`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Ra\x01\x80\x82\x01Ra\x01\x80\x81Ra\x14\xF7a\x01\xA0\x82a#\x82V[Q\x90 \x91a\x15\x08`\xA0\x86\x01Qa?9V[\x92`\xE0\x86\x01Q\x92Pa\x01`\x91a\x15!`@Q\x93\x84a#\x82V[a\x01)\x83R\x7FOrderIntent witness)OrderIntent(` \x84\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x84\x01R\x7FId,bytes32 recipient,uint256 inp``\x84\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x84\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x84\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x84\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x84\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x84\x01R\x7FPermissions(address token,uint25a\x01 \x84\x01R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x84\x01Ra\x16\xBB`Da\x01d5\x01a\x01d5`\x04\x01a\"~V[\x95\x90\x94\x80`$a\x01d5\x015\x93a\x01d5`\x04\x015\x93aC\xECV[`\x845\x90`d5\x90a6\xD2V[`$\x8D\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11`\x04R\xFD[\x87a\x17 `D\x92`d5\x90a%\x19V[\x7F\x1C\xDE1\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x845`\x04R`$R\xFD[\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R`d5`\x04R`$R`D\x87\xFD[`\x04\x88\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\xFD[\x86\x80\xFD[\x85\x80\xFD[4a\x1A\xC0W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0Wa\x17\xFE\x906\x90`\x04\x01a!\xAAV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0Wa\x18\x1E\x906\x90`\x04\x01a!\xAAV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x1B\xE7W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x1B\xBFW`@Q\x90\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RF`\x04\x83\x01R`\xA0\x82`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x1BQWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80`\x04\x94` \x93_\x91a\x1B\xA0W[P\x01Q\x16`@Q\x93\x84\x80\x92\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x91\x82\x15a\x1BQW_\x92a\x1B\\W[P\x91` \x91a\x19\xAD\x93_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x80\x97\x81\x96\x82\x95\x7FW\xEC\xFD(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`@`\x04\x85\x01Ra\x19}\x8D\x8D`D\x87\x01\x91a$\xDBV[\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x85\x84\x03\x01`$\x86\x01Ra$\xDBV[\x03\x93\x16Z\xF1\x90\x81\x15a\x1BQW_\x91a\x1B\x17W[P\x15a\x1A\xEFWa\x01x\x81\x10a\x1A\xC4W\x80`\x08\x11a\x1A\xC0W\x80`\xD8\x11a\x1A\xC0W` \x91`\xB8\x81\x015\x82`\xF8\x11a\x1A\xC0W`\xD8\x82\x015\x92\x80a\x01\x18\x11a\x1A\xC0W`\xF8\x83\x015\x91\x81a\x01X\x11a\x1A\xC0Wa\x018\x84\x015\x95P\x81a\x01x\x11a\x1A\xC0W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x88a\x01x\x85\x01\x92\x01\x900\x81\x03a\x1A\x95Wa\x1A\x87`\x04a\x1A}\x89\x89a\x1Ax\x8A\x8Aa\x1Aka\x0B\xE16\x8C\x8Ea#\xFDV[\x96\x87\x92\x015`\xE0\x1Ca,\x7FV[a%\x19V[\x90a\x0B\xFE\x81a-\xB8V[_h\x80\0\0\0\0\xAB\x14<\x06]\0[\x7F\xC7(n\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[_\x80\xFD[\x7F\xA2\xAB\xF1\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FQM\x84\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P` \x81=` \x11a\x1BIW[\x81a\x1B2` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0Wa\x1BC\x90a$3V[\x83a\x19\xC0V[=\x91Pa\x1B%V[`@Q=_\x82>=\x90\xFD[\x91P\x91` \x82=` \x11a\x1B\x98W[\x81a\x1Bx` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0Wa\x19\xAD\x92a\x1B\x8F` \x93a$QV[\x92P\x90\x92a\x19\x1CV[=\x91Pa\x1BkV[a\x1B\xB9\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[\x89a\x18\xDEV[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[4a\x1A\xC0W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W_`@\x80Qa\x1Ct\x81a\"\xE4V[\x82\x81R\x82` \x82\x01R\x01R`\x045_R`\x01` R```@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x1C\xA2\x83a\"\xE4V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16` \x84\x01\x90a\x1C\xD1\x81a!sV[\x81R\x82`@\x85\x01\x92`\xA8\x1C\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x94Q\x16\x84RQa\x1D\x06\x81a!sV[` \x84\x01RQ\x16`@\x82\x01R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0Wc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[4a\x1A\xC0W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W` `@QbLK@\x81R\xF3[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0W\x80`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x1A\xC0Wh\x80\0\0\0\0\xAB\x14<\x06\\a\x1B\xE7W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x1B\xBFWa\x1E\xB8a\x1E\xAF\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x1E\xF1a\x1E\xC0a\x1E\xC7a\x01\xC4` \x98a\x1E\x8D3\x88a'+V[\x9B\x84\x9D\x82\x99\x93\x94\x96\x92\x95\x9Ca\x1E\xA4\x8703\x8Ba)0V[a\x01\xE4\x84\x01\x90a\"~V[\x97\x90\x92\x01a\"\xCFV[\x956\x91a#\xFDV[\x92\x89a)\x97V[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0Wc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0Wa\x1F\xA6a!-V[h\x80\0\0\0\0\xAB\x14<\x06\\a\x1B\xE7W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R` R`@_ T\x80\x15a \x84W` \x913_R_\x83R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x83R_`@\x81 Ua -\x823\x83a%\x8AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83\x843\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x1A\xC0W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x1A\xC0W`\x045\x80\x15\x15\x80\x91\x03a\x1A\xC0Wa \xF0a%SV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`\x03\x11\x15a!}WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x1A\xC0W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x1A\xC0W` \x83\x81\x86\x01\x95\x01\x01\x11a\x1A\xC0WV[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`\xA45\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[`\xC45\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x1A\xC0W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1A\xC0W` \x01\x91\x816\x03\x83\x13a\x1A\xC0WV[5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x1A\xC0W\x90V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[a\x02\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a#\0W`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a#\0W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a$\t\x82a#\xC3V[\x91a$\x17`@Q\x93\x84a#\x82V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x1A\xC0W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[Q\x90\x81\x15\x15\x82\x03a\x1A\xC0WV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[\x90\x81`\xA0\x91\x03\x12a\x1A\xC0Wa$\xD3`\x80`@Q\x92a$\x8F\x84a#-V[a$\x98\x81a$3V[\x84Ra$\xA6` \x82\x01a$@V[` \x85\x01Ra$\xB7`@\x82\x01a$@V[`@\x85\x01Ra$\xC8``\x82\x01a$QV[``\x85\x01R\x01a$QV[`\x80\x82\x01R\x90V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x91\x90\x82\x03\x91\x82\x11a%&WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a%}WV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a%\xC6W[PP_`4RV[;\x15=\x17\x10\x15a%\xD7W_\x80a%\xBEV[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5`\xFF\x81\x16\x81\x03a\x1A\xC0W\x90V[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x1A\xC0W\x90V[\x91\x90a\x02\0\x83\x82\x03\x12a\x1A\xC0W`@Q\x90a&\x1D\x82a#IV[\x81\x93\x805`\xFF\x81\x16\x81\x03a\x1A\xC0W\x83Ra&9` \x82\x01a!\xFEV[` \x84\x01Ra&J`@\x82\x01a!\xFEV[`@\x84\x01R``\x81\x015``\x84\x01R`\x80\x81\x015`\x80\x84\x01R`\xA0\x81\x015`\xA0\x84\x01R`\xC0\x81\x015`\xC0\x84\x01R`\xE0\x81\x015`\xE0\x84\x01Ra\x01\0\x81\x015a\x01\0\x84\x01Ra&\x9Aa\x01 \x82\x01a\"&V[a\x01 \x84\x01Ra&\xADa\x01@\x82\x01a\"&V[a\x01@\x84\x01Ra&\xC0a\x01`\x82\x01a\"&V[a\x01`\x84\x01Ra\x01\x80\x81\x015a\x01\x80\x84\x01Ra\x01\xA0\x81\x015a\x01\xA0\x84\x01Ra&\xEBa\x01\xC0\x82\x01a\"&V[a\x01\xC0\x84\x01Ra\x01\xE0\x81\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1A\xC0W\x01\x81`\x1F\x82\x01\x12\x15a\x1A\xC0Wa\x01\xE0\x91\x81` a'&\x935\x91\x01a#\xFDV[\x91\x01RV[\x90`\xFFa'7\x83a%\xE4V[\x16a(\xF5W`@\x82\x01c\xFF\xFF\xFF\xFFa'N\x82a%\xF2V[\x16F\x03a(\xBBWPa'ha'c6\x84a&\x03V[a-\xB8V[\x92\x83_R`\x01` R`@_ \x92\x83T\x91`\xFF\x83`\xA0\x1C\x16a'\x89\x81a!sV[a(\x8FWa'\x9Fa'\x9A6\x84a&\x03V[a>JV[\x94a'\xAD`\x80\x84\x015a?9V[\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a(,a(%a\x01\0\x88\x015\x97a'\xFFa\x01@\x82\x01a\"\xCFV[a(\x0Ca\x01`\x83\x01a\"\xCFV[`\x02T\x91a\x01\x80a\x01\xA0\x85\x015\x94\x015\x91B\x91\x8Da<\xA2V[\x80\x97a%\x19V[\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFy\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0B`\xA8\x1B\x16\x95\x16\x91\x16\x17\x17\x16\x17\x90UV[\x85\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a(\xC9c\xFF\xFF\xFF\xFF\x91a%\xF2V[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\xFFa)\0\x83a%\xE4V[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R\x16`$R`D_\xFD[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a)yW[PP_``R`@RV[;\x15=\x17\x10\x15a)\x8AW_\x80a)nV[cy9\xF4$_R`\x04`\x1C\xFD[\x91a)\xC1\x95\x93\x91\x81\x95\x80Q\x15_\x14a)\xC3WP`@Qa)\xB8` \x82a#\x82V[_\x81R\x93a?\xB4V[V[a*[a*/\x91`@Q\x92\x83\x91\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x88`$\x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`D\x84\x01R\x87`d\x84\x01R`\x80`\x84\x84\x01R`\xA4\x83\x01\x90a\";V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a#\x82V[\x93a?\xB4V[\x91\x90\x82\x01\x80\x92\x11a%&WV[`@Q\x90a*{\x82a#IV[``a\x01\xE0\x83_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R_`\xA0\x82\x01R_`\xC0\x82\x01R_`\xE0\x82\x01R_a\x01\0\x82\x01R_a\x01 \x82\x01R_a\x01@\x82\x01R_a\x01`\x82\x01R_a\x01\x80\x82\x01R_a\x01\xA0\x82\x01R_a\x01\xC0\x82\x01R\x01RV[Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x1A\xC0WV[a*\xFFa*nV[P\x80Q\x81\x01\x90` \x82\x01\x90` \x81\x84\x03\x12a\x1A\xC0W` \x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0W\x01\x91a\x02\0\x90\x83\x90\x03\x12a\x1A\xC0W`@Q\x91a+C\x83a#IV[` \x81\x01Q`\xFF\x81\x16\x81\x03a\x1A\xC0W\x83Ra+``@\x82\x01a$@V[` \x84\x01Ra+q``\x82\x01a$@V[`@\x84\x01R`\x80\x81\x01Q``\x84\x01R`\xA0\x81\x01Q`\x80\x84\x01R`\xC0\x81\x01Q`\xA0\x84\x01R`\xE0\x81\x01Q`\xC0\x84\x01Ra\x01\0\x81\x01Q`\xE0\x84\x01Ra\x01 \x81\x01Qa\x01\0\x84\x01Ra+\xC2a\x01@\x82\x01a*\xE2V[a\x01 \x84\x01Ra+\xD5a\x01`\x82\x01a*\xE2V[a\x01@\x84\x01Ra+\xE8a\x01\x80\x82\x01a*\xE2V[a\x01`\x84\x01Ra\x01\xA0\x81\x01Qa\x01\x80\x84\x01Ra\x01\xC0\x81\x01Qa\x01\xA0\x84\x01Ra,\x13a\x01\xE0\x82\x01a*\xE2V[a\x01\xC0\x84\x01Ra\x02\0\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC0W` \x91\x01\x01\x81`\x1F\x82\x01\x12\x15a\x1A\xC0W\x80Q\x90a,J\x82a#\xC3V[\x92a,X`@Q\x94\x85a#\x82V[\x82\x84R` \x83\x83\x01\x01\x11a\x1A\xC0W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01Ra\x01\xE0\x82\x01R\x90V[\x90\x91` c\xFF\xFF\xFF\xFF\x91\x01Q\x16`@Q\x90\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x1BQW_\x91a-\x99W[P\x80Q\x15\x90\x81\x15a-~W[Pa-LWP0\x81\x03a-!WPV[\x7F\xC2\x1F\xA2\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x90\x7Fj\x96e\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x91P` \x01Q\x16c\xFF\xFF\xFF\xFF\x82\x16\x14\x15_a-\x11V[a-\xB2\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a-\x05V[\x90a\x01\xE0\x82\x01\x80QQ`\x1F\x81\x01\x80\x91\x11a%&W`\x05\x1C\x90\x81`\x12\x01\x80`\x12\x11a%&W`\x13`@Q\x93\x84\x92\x83R\x01`\x05\x1B\x01`@Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0` \x84\x01\x95` \x87R`\xFF\x81Q\x16`@\x86\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16``\x86\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`\x80\x86\x01R``\x81\x01Q`\xA0\x86\x01R`\x80\x81\x01Q`\xC0\x86\x01R`\xA0\x81\x01Q`\xE0\x86\x01R`\xC0\x81\x01Qa\x01\0\x86\x01R`\xE0\x81\x01Qa\x01 \x86\x01Ra\x01\0\x81\x01Qa\x01@\x86\x01R\x82a\x01 \x82\x01Q\x16a\x01`\x86\x01R\x82a\x01@\x82\x01Q\x16a\x01\x80\x86\x01R\x82a\x01`\x82\x01Q\x16a\x01\xA0\x86\x01Ra\x01\x80\x81\x01Q\x82\x86\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x86\x01R\x01Q\x16a\x02\0\x83\x01Ra\x02\0a\x02 \x83\x01R\x80QQa\x02@\x83\x01RQ\x80Q\x80a.\xF3W[PP\x80Q\x92\x83`\x05\x1B\x90 \x92`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x86\x01\x94\x01\x01\x90[\x81\x81\x10a/sWPP`\x1F\x16\x90\x81\x15a.\xD1W`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a.\xD1V[\x80Q\x84R` \x93\x84\x01\x93\x01a/\x04V[\x91\x81_R`\x01` R`@_ \x80T`\x02`\xFF\x82`\xA0\x1C\x16a/\xA4\x81a!sV[\x14a0\xCCW\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x92\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x94a/\xF9\x88a>JV[\x91a0\x07`\x80\x8A\x01Qa?9V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x99a\x01\0\x81\x01Q\x80\x88\x10_\x14a0\xC6WP\x86\x91[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a0S\x84\x8Aa%\x19V[\x97\x88\x96\x16\x17\x90U\x86\x8B\x15\x15_\x14a0\x9BWPPa0r\x90\x8A\x85\x8BaB\xC9V[\x81a0\x89W[PPP[\x82Q\x91\x82R` \x82\x01R\xA3V[a0\x93\x92\x88aB\xC9V[_\x81\x81a0xV[\x91P\x91a0\xC1\x94\x93Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0a\x01\xE0\x85\x01Q\x94\x01Q\x16\x93\x8Aa)\x97V[a0|V[\x91a03V[\x83\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a%&WV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a%&Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[\x95\x94\x98\x93\x90\x92\x91\x92a1\x81a*nV[P`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9A\x90`\xA0\x81`$\x81\x8FZ\xFA\x80\x15a\x1BQW``\x91_\x91a3\xE5W[P\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91`@Q\x80\x80\x92\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Rc\xFF\xFF\xFF\xFF\x16\x9D\x8E`\x04\x83\x01RZ\x92`$\x91`\xA0\x94\xFA\x90\x81\x15a\x1BQWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91``\x91_\x91a3\xC6W[P\x01Q\x16\x92\x82\x15a3\x94W\x83\x15a3hWa2\x9B\x90\x85a%\x19V[\x90a2\xA5\x91a%\x19V[\x93Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96a2\xBC\x90\x88a0\xF8V[\x97`@Q\x9Ba2\xCA\x8Da#IV[_\x8DR\x8CFc\xFF\xFF\xFF\xFF\x16\x90` \x01R`@\x8D\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Ra\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01Ra\x01\xC0\x81\x01_\x90R`@Qa3\\` \x82a#\x82V[_\x81Ra\x01\xE0\x82\x01R\x90V[\x8B\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFFF\x16`\x04R`$_\xFD[a3\xDF\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a2\x80V[a3\xFE\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a2\0V[c\xFF\xFF\xFF\xFF` \x82\x01Q\x16F\x81\x03a6\xA7WPc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x81`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x1BQW_\x91a6\x88W[P\x80Q\x15\x90\x81\x15a6eW[Pa6:WPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x83\x01Q\x16\x80\x82\x11\x15a6\x0CWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a5\xFFW[a5\xCBWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a5\x9DWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa5!`\x80\x83\x01Qa?9V[\x16\x15a5uWa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a5CWPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x90\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a4\xE7V[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P``\x01Q\x16\x15_a4\xA7V[a6\xA1\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a4\x9BV[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x93\x91\x93a6\xDF\x82a-\xB8V[\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x87Z\xFA\x90\x81\x15a\x1BQW_\x91a<\x02W[P`\x80\x81\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16`\x04` c\xFF\xFF\xFF\xFF\x81\x86\x01Q\x16\x92`@Q\x92\x83\x80\x92\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x90\x81\x15a\x1BQW_\x91a;\xB0W[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7F\x8D68\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x80\x15a\x1BQW_\x90a;sW[c\xFF\xFF\xFF\xFF\x91P\x16\x90\x80\x82\x03a;EWPP`@\x86\x01\x94`\xA0c\xFF\xFF\xFF\xFF\x87Q\x16`$`@Q\x80\x94\x81\x93\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\x1BQWc\xFF\xFF\xFF\xFF\x91` \x91_\x91a;&W[P\x01Q\x16\x98\x80a9FWP\x87\x98`\xA0\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x97\x94\x86``\x81a8\xF3\x97\x9E\x9C\x9D\x9EQ\x16\x92\x01Q\x16\x93`\xE0\x8C\x01Qa8\xEB\x8DaE\x92V[\x950\x93aF\x99V[c\xFF\xFF\xFF\xFFa9\x05``\x88\x01Qa?9V[\x91Q\x16\x95`\xC0\x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01 a\x01\0\x83\x01Q\x92\x01Q\x16\x91`@Q\x98_\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R\x16\x93\xA3V[b\x05W0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x89\x97\x95\x97\x96\x94\x96\x01Q\x16\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a%&Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x99bLK@\x8B\x11a:\xF3W\x89\x9A`\x80\x89\x9A\x9B\x99\x01Q\x92a9\x99\x8BaE\x92V[`@Q\x91a9\xA6\x83a#-V[\x82R` \x82\x01\x970\x89R`@\x83\x01\x93\x84R``\x83\x01\x95\x86R`\x80\x83\x01\x91\x82RQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x98``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94`\xE0\x8D\x01Q\x93`@Q\x99\x8A\x94` \x86\x01` \x90RQ`@\x86\x01RQ``\x85\x01RQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x84\x01RQ`\xA0\x83\x01RQ`\xC0\x82\x01`\xA0\x90R`\xE0\x82\x01a:D\x91a\";V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x87Ra:t\x90\x87a#\x82V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92a:\xB5\x97aF\x99V[`\xA0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x92a8\xF3V[\x8A\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RbLK@`$R`D_\xFD[a;?\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a8\x81V[\x7F\xC9\xE00\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P` \x81=` \x11a;\xA8W[\x81a;\x8D` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0Wa;\xA3c\xFF\xFF\xFF\xFF\x91a$@V[a8\x16V[=\x91Pa;\x80V[\x90P` \x81=` \x11a;\xFAW[\x81a;\xCB` \x93\x83a#\x82V[\x81\x01\x03\x12a\x1A\xC0W` a;\xF3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a$QV[\x91Pa7\xC1V[=\x91Pa;\xBEV[a<\x1B\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a7[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x95\x92\x90\x93\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x90\x81\x81\x10_\x14a>CWP[_\x93\x80\x82\x10a<\xE9W[PPPa<\xD8\x92Pa*aV[\x81\x81\x11\x15a<\xE4WP\x90V[\x90P\x90V[a<\xF5\x92\x93\x94Pa%\x19V[\x90\x80\x15\x80\x15\x90\x81a=\xE5W[P\x15a=\xDEWPP\x81[\x80\x83\x11a=\xD6W[P\x81\x83\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x84\x15\x82\x86\x86\x04\x14\x17\x02\x15a=HWPg\r\xE0\xB6\xB3\xA7d\0\0a<\xD8\x92\x04[\x90_\x80\x80a<\xCBV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x86\t\x82\x81\x10\x83\x01\x90\x03\x93\x85\t\x83g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a=\xC9Wa<\xD8\x93\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02a=?V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x91P_a=\x13V[\x02\x91a=\x0BV[\x90Pa>\x16W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11_a=\x01V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90Pa<\xC1V[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x80\x15a\x1BQW``s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\xC0\x93_\x91a?\x1AW[P\x01Q\x16\x91\x01Q\x81\x81\x03a>\xEFWP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a?3\x91P`\xA0=`\xA0\x11a\x05vWa\x05h\x81\x83a#\x82V[_a>\xDDV[\x80`\xA0\x1Ca?ZWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[=\x15a?\xAFW=\x90a?\x96\x82a#\xC3V[\x91a?\xA4`@Q\x93\x84a#\x82V[\x82R=_` \x84\x01>V[``\x90V[\x94_\x94\x84\x15aB\xBFW\x80Q\x15\x80\x15aB\xB6W[aB\xA5WZa\xC3Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa?\xF3\x81\x86\x16\x95g\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`\x05\x1C\x16\x90a0\xF8V[\x16\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a%&Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aBuW0;\x15a\x1A\xC0W_a@\x8C\x91`@Q\x80\x93\x81\x92\x7F\xACO\xCA\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x99\x8A`\x04\x86\x01R\x16\x96\x87`$\x85\x01R\x8A`D\x85\x01R`d\x84\x01R`\xA0`\x84\x84\x01R`\xA4\x83\x01\x90a\";V[\x03\x81\x830Z\xF1\x90\x81aB`W[PaB.WPa@\xA7a?\x85V[\x91\x84\x92`$\x81Q\x14aA\xBDW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x96\x87\x15\x15\x80aA\xB3W[\x15aA\x18WPP` \x92\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x94\x92aA\r\x92\x87aB\xC9V[P`@Q`\x01\x81R\xA3V[` \x97P\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x95\x93P\x87\x94\x91\x92P\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x92\x16\x98\x89\x93\x84\x87R\x86\x82R`@\x87 \x86_R\x82R`@_ aA\x9F\x82\x82Ta*aV[\x90U`@Q\x90\x81R\xA4P`@Q`\x02\x81R\xA3V[P0\x88\x14\x15a@\xD6V[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x03a@\xB4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P_a@\xB4V[\x95PPPP` \x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x91`@Q\x90\x81R\xA3V[aBm\x91\x96P_\x90a#\x82V[_\x94_a@\x99V[PZ\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[PP\x91\x93\x90\x92Pa)\xC1\x94PaB\xC9V[P\x83;\x15a?\xC7V[PPPPPPPPV[\x90\x83\x15aC\xAFW_\x80`@Q\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x87\x01\x91\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16\x95\x86`$\x82\x01R\x87`D\x82\x01R`D\x81RaC0`d\x82a#\x82V[Q\x90\x82\x85Z\xF1aC>a?\x85V[\x81aC\xB5W[PaC\xAFW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x92\x85_R_\x83R`@_ \x82\x82\x16_R\x83R`@_ aC\xA2\x88\x82Ta*aV[\x90U`@Q\x96\x87R\x16\x94\xA4V[PPPPV[\x80Q\x80\x15\x92P\x82\x15aC\xCAW[PP_aCDV[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x1A\xC0W` aC\xE5\x91\x01a$3V[_\x80aC\xC2V[\x95\x91\x98\x93\x96\x92\x98\x97\x94\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x97aD\x18\x89a#fV[\x16\x87R` \x87\x01R`@Q\x95aD-\x87a\"\xE4V[\x86R` \x86\x01\x96\x87R`@\x86\x01\x97\x88R`@Q\x98aDJ\x8Aa#fV[0\x8AR` \x8A\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x1A\xC0W`@Q\x98\x89\x98\x89\x98\x89\x98\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8AR`\x04\x8A\x01\x90Q\x90aD\xC5\x91` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[Q`D\x89\x01RQ`d\x88\x01R\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x84\x88\x01R` \x01Q`\xA4\x87\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC4\x86\x01R`\xE4\x85\x01Ra\x01\x04\x84\x01a\x01@\x90Ra\x01D\x84\x01aE/\x91a\";V[\x90\x83\x82\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x01a\x01$\x85\x01RaEd\x92a$\xDBV[\x03Z\x92_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81\x95\xF1\x80\x15a\x1BQWaE\x88WPV[_a)\xC1\x91a#\x82V[aF\x96a\x01\xE0\x91a*/`@Q\x93\x84\x92` \x80\x85\x01R`\xFF\x81Q\x16`@\x85\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16``\x85\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`\x80\x85\x01R``\x81\x01Q`\xA0\x85\x01R`\x80\x81\x01Q`\xC0\x85\x01R`\xA0\x81\x01Q`\xE0\x85\x01R`\xC0\x81\x01Qa\x01\0\x85\x01R`\xE0\x81\x01Qa\x01 \x85\x01Ra\x01\0\x81\x01Qa\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01 \x82\x01Q\x16a\x01`\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x82\x01Q\x16a\x01\x80\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x82\x01Q\x16a\x02\0\x85\x01R\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a\";V[\x90V[\x96\x95\x91\x90\x96\x94\x93\x94\x80`\x14R\x81`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x8CZ\xF1\x80`\x01_Q\x14\x16\x15aG\x93W[P_`4Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x84;\x15a\x1A\xC0W_\x96c\xFF\xFF\xFF\xFF\x88\x94aG\x82\x93\x82\x99s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x9D\x8E\x9C\x8D\x9B\x8C\x9A\x7Fw\x9BC-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8CR`\x04\x8C\x01R\x16`$\x8A\x01R\x82`D\x8A\x01R\x16`d\x88\x01R`\x84\x87\x01R`\xA4\x86\x01R\x16`\xC4\x84\x01Ra\x01\0`\xE4\x84\x01Ra\x01\x04\x83\x01\x90a\";V[\x03\x92Z\xF1\x80\x15a\x1BQWaE\x88WPV[=\x89;\x15\x17\x10\x15aG\xA5W[_aF\xD4V[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x8CZ\xF1P\x81`4R` _`D`\x10\x82\x8CZ\xF1\x80`\x01_Q\x14\x16\x15aG\xE9W[PaG\x9FV[=\x89;\x15\x17\x10\x15aG\xFAW_aG\xE3V[c>?\x8Fs_R`\x04`\x1C\xFD\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CallbackResult(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<CallbackResult> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl CallbackResult {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for CallbackResult {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<CallbackResult> for u8 {
            fn from(value: CallbackResult) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CallbackResult {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CallbackResult {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FillStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<FillStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl FillStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for FillStatus {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<FillStatus> for u8 {
            fn from(value: FillStatus) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FillStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FillStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Execution { uint64 gasLimit; bytes data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Execution {
        #[allow(missing_docs)]
        pub gasLimit: u64,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, alloy::sol_types::private::Bytes);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Execution> for UnderlyingRustTuple<'_> {
            fn from(value: Execution) -> Self {
                (value.gasLimit, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Execution {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    gasLimit: tuple.0,
                    data: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Execution {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Execution {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasLimit),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Execution {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Execution {
            const NAME: &'static str = "Execution";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Execution(uint64 gasLimit,bytes data)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gasLimit)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Execution {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasLimit,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Order { uint8 bridgeType; uint32 srcChainId; uint32 dstChainId; bytes32 sender; bytes32 recipient; bytes32 inputToken; bytes32 outputToken; uint256 inputAmount; uint256 outputAmount; uint64 nonce; uint64 startTime; uint64 expectedDeliveryTime; uint256 discountRate; uint256 baseFee; uint64 callbackGasLimit; bytes hookData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Order {
        #[allow(missing_docs)]
        pub bridgeType: u8,
        #[allow(missing_docs)]
        pub srcChainId: u32,
        #[allow(missing_docs)]
        pub dstChainId: u32,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputToken: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub outputToken: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub outputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nonce: u64,
        #[allow(missing_docs)]
        pub startTime: u64,
        #[allow(missing_docs)]
        pub expectedDeliveryTime: u64,
        #[allow(missing_docs)]
        pub discountRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callbackGasLimit: u64,
        #[allow(missing_docs)]
        pub hookData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            u32,
            u32,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            u64,
            u64,
            u64,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            u64,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Order> for UnderlyingRustTuple<'_> {
            fn from(value: Order) -> Self {
                (
                    value.bridgeType,
                    value.srcChainId,
                    value.dstChainId,
                    value.sender,
                    value.recipient,
                    value.inputToken,
                    value.outputToken,
                    value.inputAmount,
                    value.outputAmount,
                    value.nonce,
                    value.startTime,
                    value.expectedDeliveryTime,
                    value.discountRate,
                    value.baseFee,
                    value.callbackGasLimit,
                    value.hookData,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Order {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bridgeType: tuple.0,
                    srcChainId: tuple.1,
                    dstChainId: tuple.2,
                    sender: tuple.3,
                    recipient: tuple.4,
                    inputToken: tuple.5,
                    outputToken: tuple.6,
                    inputAmount: tuple.7,
                    outputAmount: tuple.8,
                    nonce: tuple.9,
                    startTime: tuple.10,
                    expectedDeliveryTime: tuple.11,
                    discountRate: tuple.12,
                    baseFee: tuple.13,
                    callbackGasLimit: tuple.14,
                    hookData: tuple.15,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Order {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Order {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bridgeType),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.srcChainId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.dstChainId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.sender),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.recipient),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputToken),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputToken),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTime),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedDeliveryTime),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.discountRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFee),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.callbackGasLimit),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Order {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Order {
            const NAME: &'static str = "Order";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Order(uint8 bridgeType,uint32 srcChainId,uint32 dstChainId,bytes32 sender,bytes32 recipient,bytes32 inputToken,bytes32 outputToken,uint256 inputAmount,uint256 outputAmount,uint64 nonce,uint64 startTime,uint64 expectedDeliveryTime,uint256 discountRate,uint256 baseFee,uint64 callbackGasLimit,bytes hookData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.bridgeType)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.srcChainId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.dstChainId)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sender)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.recipient)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.inputToken)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.outputToken)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.inputAmount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.outputAmount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startTime)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.expectedDeliveryTime,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.discountRate)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.baseFee)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.callbackGasLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hookData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Order {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bridgeType,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.srcChainId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.dstChainId,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sender,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipient,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inputToken,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inputAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTime,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expectedDeliveryTime,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.discountRate,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callbackGasLimit,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hookData,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bridgeType,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.srcChainId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.dstChainId,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sender,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.recipient,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inputToken,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inputAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTime,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expectedDeliveryTime,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.discountRate,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callbackGasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hookData,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OrderRecord { address filler; FillStatus status; uint40 fillTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrderRecord {
        #[allow(missing_docs)]
        pub filler: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub status: <FillStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub fillTime: alloy::sol_types::private::primitives::aliases::U40,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            FillStatus,
            alloy::sol_types::sol_data::Uint<40>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            <FillStatus as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U40,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OrderRecord> for UnderlyingRustTuple<'_> {
            fn from(value: OrderRecord) -> Self {
                (value.filler, value.status, value.fillTime)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrderRecord {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    filler: tuple.0,
                    status: tuple.1,
                    fillTime: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OrderRecord {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OrderRecord {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.filler,
                    ),
                    <FillStatus as alloy_sol_types::SolType>::tokenize(&self.status),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.fillTime),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OrderRecord {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OrderRecord {
            const NAME: &'static str = "OrderRecord";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OrderRecord(address filler,uint8 status,uint40 fillTime)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.filler,
                        )
                        .0,
                    <FillStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fillTime)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OrderRecord {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.filler,
                    )
                    + <FillStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fillTime,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.filler,
                    out,
                );
                <FillStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    40,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fillTime,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [13u8, 193u8, 73u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadySettled(bytes32)` and selector `0xb196a44a`.
```solidity
error AlreadySettled(bytes32 orderId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadySettled {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadySettled> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadySettled) -> Self {
                (value.orderId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadySettled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { orderId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadySettled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadySettled(bytes32)";
            const SELECTOR: [u8; 4] = [177u8, 150u8, 164u8, 74u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DomainMismatch(uint32,uint32)` and selector `0xc9e030e8`.
```solidity
error DomainMismatch(uint32 configured, uint32 onchain);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DomainMismatch {
        #[allow(missing_docs)]
        pub configured: u32,
        #[allow(missing_docs)]
        pub onchain: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DomainMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: DomainMismatch) -> Self {
                (value.configured, value.onchain)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DomainMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    configured: tuple.0,
                    onchain: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DomainMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DomainMismatch(uint32,uint32)";
            const SELECTOR: [u8; 4] = [201u8, 224u8, 48u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.configured),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.onchain),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientCallbackGas(uint256,uint256)` and selector `0x588700c7`.
```solidity
error InsufficientCallbackGas(uint256 available, uint256 callbackGasLimit);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientCallbackGas {
        #[allow(missing_docs)]
        pub available: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callbackGasLimit: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientCallbackGas> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientCallbackGas) -> Self {
                (value.available, value.callbackGasLimit)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientCallbackGas {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    available: tuple.0,
                    callbackGasLimit: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientCallbackGas {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientCallbackGas(uint256,uint256)";
            const SELECTOR: [u8; 4] = [88u8, 135u8, 0u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.available),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.callbackGasLimit),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidAddress(bytes32)` and selector `0x2bf95065`.
```solidity
error InvalidAddress(bytes32 b);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAddress {
        #[allow(missing_docs)]
        pub b: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAddress> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAddress) -> Self {
                (value.b,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { b: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAddress(bytes32)";
            const SELECTOR: [u8; 4] = [43u8, 249u8, 80u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.b),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBaseFee(uint256,uint256)` and selector `0x8d00b91b`.
```solidity
error InvalidBaseFee(uint256 baseFee, uint256 outputAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBaseFee {
        #[allow(missing_docs)]
        pub baseFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub outputAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBaseFee> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBaseFee) -> Self {
                (value.baseFee, value.outputAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBaseFee {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    baseFee: tuple.0,
                    outputAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBaseFee {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBaseFee(uint256,uint256)";
            const SELECTOR: [u8; 4] = [141u8, 0u8, 185u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidCallbackGasLimit(uint64,uint64)` and selector `0x25ad8594`.
```solidity
error InvalidCallbackGasLimit(uint64 callbackGasLimit, uint64 maxCallbackGasLimit);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCallbackGasLimit {
        #[allow(missing_docs)]
        pub callbackGasLimit: u64,
        #[allow(missing_docs)]
        pub maxCallbackGasLimit: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidCallbackGasLimit> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCallbackGasLimit) -> Self {
                (value.callbackGasLimit, value.maxCallbackGasLimit)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCallbackGasLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    callbackGasLimit: tuple.0,
                    maxCallbackGasLimit: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCallbackGasLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCallbackGasLimit(uint64,uint64)";
            const SELECTOR: [u8; 4] = [37u8, 173u8, 133u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.callbackGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxCallbackGasLimit),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidMaxFeeRate(uint256)` and selector `0xad6bb6d1`.
```solidity
error InvalidMaxFeeRate(uint256 maxFeeRate);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidMaxFeeRate {
        #[allow(missing_docs)]
        pub maxFeeRate: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidMaxFeeRate> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidMaxFeeRate) -> Self {
                (value.maxFeeRate,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidMaxFeeRate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { maxFeeRate: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidMaxFeeRate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidMaxFeeRate(uint256)";
            const SELECTOR: [u8; 4] = [173u8, 107u8, 182u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFeeRate),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidOutputAmount(uint256,uint256)` and selector `0x8dd09d91`.
```solidity
error InvalidOutputAmount(uint256 outputAmount, uint256 inputAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOutputAmount {
        #[allow(missing_docs)]
        pub outputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidOutputAmount> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOutputAmount) -> Self {
                (value.outputAmount, value.inputAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOutputAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    outputAmount: tuple.0,
                    inputAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOutputAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOutputAmount(uint256,uint256)";
            const SELECTOR: [u8; 4] = [141u8, 208u8, 157u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidWindow(uint64,uint64)` and selector `0x2802dd9e`.
```solidity
error InvalidWindow(uint64 startTime, uint64 expectedDeliveryTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidWindow {
        #[allow(missing_docs)]
        pub startTime: u64,
        #[allow(missing_docs)]
        pub expectedDeliveryTime: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidWindow> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidWindow) -> Self {
                (value.startTime, value.expectedDeliveryTime)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidWindow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startTime: tuple.0,
                    expectedDeliveryTime: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidWindow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidWindow(uint64,uint64)";
            const SELECTOR: [u8; 4] = [40u8, 2u8, 221u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTime),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedDeliveryTime),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MaxFeeTooHigh(uint256,uint256)` and selector `0xed2bc1ea`.
```solidity
error MaxFeeTooHigh(uint256 maxFee, uint256 inputAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxFeeTooHigh {
        #[allow(missing_docs)]
        pub maxFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MaxFeeTooHigh> for UnderlyingRustTuple<'_> {
            fn from(value: MaxFeeTooHigh) -> Self {
                (value.maxFee, value.inputAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxFeeTooHigh {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxFee: tuple.0,
                    inputAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxFeeTooHigh {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxFeeTooHigh(uint256,uint256)";
            const SELECTOR: [u8; 4] = [237u8, 43u8, 193u8, 234u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MessageTooShort(uint256)` and selector `0xa2abf1b6`.
```solidity
error MessageTooShort(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MessageTooShort {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MessageTooShort> for UnderlyingRustTuple<'_> {
            fn from(value: MessageTooShort) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MessageTooShort {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MessageTooShort {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MessageTooShort(uint256)";
            const SELECTOR: [u8; 4] = [162u8, 171u8, 241u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MintFeeTooHigh(uint256,uint256)` and selector `0x1cde3111`.
```solidity
error MintFeeTooHigh(uint256 mintFee, uint256 maxOutput);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MintFeeTooHigh {
        #[allow(missing_docs)]
        pub mintFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxOutput: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MintFeeTooHigh> for UnderlyingRustTuple<'_> {
            fn from(value: MintFeeTooHigh) -> Self {
                (value.mintFee, value.maxOutput)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MintFeeTooHigh {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    mintFee: tuple.0,
                    maxOutput: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MintFeeTooHigh {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MintFeeTooHigh(uint256,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 222u8, 49u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxOutput),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MintRecipientMismatch(bytes32)` and selector `0xc7286ea1`.
```solidity
error MintRecipientMismatch(bytes32 mintRecipient);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MintRecipientMismatch {
        #[allow(missing_docs)]
        pub mintRecipient: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MintRecipientMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: MintRecipientMismatch) -> Self {
                (value.mintRecipient,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MintRecipientMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { mintRecipient: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MintRecipientMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MintRecipientMismatch(bytes32)";
            const SELECTOR: [u8; 4] = [199u8, 40u8, 110u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintRecipient),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NewOwnerIsZeroAddress()` and selector `0x7448fbae`.
```solidity
error NewOwnerIsZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NewOwnerIsZeroAddress;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NewOwnerIsZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: NewOwnerIsZeroAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NewOwnerIsZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NewOwnerIsZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NewOwnerIsZeroAddress()";
            const SELECTOR: [u8; 4] = [116u8, 72u8, 251u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NoHandoverRequest()` and selector `0x6f5e8818`.
```solidity
error NoHandoverRequest();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoHandoverRequest;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NoHandoverRequest> for UnderlyingRustTuple<'_> {
            fn from(value: NoHandoverRequest) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoHandoverRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoHandoverRequest {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoHandoverRequest()";
            const SELECTOR: [u8; 4] = [111u8, 94u8, 136u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotSourceChain(uint32)` and selector `0x1b2f5167`.
```solidity
error NotSourceChain(uint32 srcChainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSourceChain {
        #[allow(missing_docs)]
        pub srcChainId: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotSourceChain> for UnderlyingRustTuple<'_> {
            fn from(value: NotSourceChain) -> Self {
                (value.srcChainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSourceChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { srcChainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSourceChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSourceChain(uint32)";
            const SELECTOR: [u8; 4] = [27u8, 47u8, 81u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.srcChainId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NothingToClaim()` and selector `0x969bf728`.
```solidity
error NothingToClaim();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NothingToClaim;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NothingToClaim> for UnderlyingRustTuple<'_> {
            fn from(value: NothingToClaim) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NothingToClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NothingToClaim {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NothingToClaim()";
            const SELECTOR: [u8; 4] = [150u8, 155u8, 247u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlySelf()` and selector `0x14d4a4e8`.
```solidity
error OnlySelf();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySelf;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlySelf> for UnderlyingRustTuple<'_> {
            fn from(value: OnlySelf) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlySelf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySelf {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySelf()";
            const SELECTOR: [u8; 4] = [20u8, 212u8, 164u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OrderAlreadyActive(bytes32)` and selector `0x343e211e`.
```solidity
error OrderAlreadyActive(bytes32 orderId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrderAlreadyActive {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OrderAlreadyActive> for UnderlyingRustTuple<'_> {
            fn from(value: OrderAlreadyActive) -> Self {
                (value.orderId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrderAlreadyActive {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { orderId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OrderAlreadyActive {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OrderAlreadyActive(bytes32)";
            const SELECTOR: [u8; 4] = [52u8, 62u8, 33u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `Paused()` and selector `0x9e87fac8`.
```solidity
error Paused();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Paused;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Paused> for UnderlyingRustTuple<'_> {
            fn from(value: Paused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Paused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Paused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Paused()";
            const SELECTOR: [u8; 4] = [158u8, 135u8, 250u8, 200u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReceiveMessageFailed()` and selector `0x514d840a`.
```solidity
error ReceiveMessageFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReceiveMessageFailed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReceiveMessageFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ReceiveMessageFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReceiveMessageFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReceiveMessageFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReceiveMessageFailed()";
            const SELECTOR: [u8; 4] = [81u8, 77u8, 132u8, 10u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `RedirectFunds(address)` and selector `0xf7c3b366`.
```solidity
error RedirectFunds(address dest);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RedirectFunds {
        #[allow(missing_docs)]
        pub dest: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RedirectFunds> for UnderlyingRustTuple<'_> {
            fn from(value: RedirectFunds) -> Self {
                (value.dest,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RedirectFunds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { dest: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RedirectFunds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RedirectFunds(address)";
            const SELECTOR: [u8; 4] = [247u8, 195u8, 179u8, 102u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.dest,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `Reentrancy()` and selector `0xab143c06`.
```solidity
error Reentrancy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Reentrancy;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Reentrancy> for UnderlyingRustTuple<'_> {
            fn from(value: Reentrancy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Reentrancy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Reentrancy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Reentrancy()";
            const SELECTOR: [u8; 4] = [171u8, 20u8, 60u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `Unauthorized()` and selector `0x82b42900`.
```solidity
error Unauthorized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Unauthorized;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Unauthorized> for UnderlyingRustTuple<'_> {
            fn from(value: Unauthorized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Unauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Unauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Unauthorized()";
            const SELECTOR: [u8; 4] = [130u8, 180u8, 41u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnsupportedChain(uint32)` and selector `0xb825dd76`.
```solidity
error UnsupportedChain(uint32 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnsupportedChain {
        #[allow(missing_docs)]
        pub chainId: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnsupportedChain> for UnderlyingRustTuple<'_> {
            fn from(value: UnsupportedChain) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnsupportedChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnsupportedChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnsupportedChain(uint32)";
            const SELECTOR: [u8; 4] = [184u8, 37u8, 221u8, 118u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UntrustedExecutor(address)` and selector `0x50ce3ed9`.
```solidity
error UntrustedExecutor(address caller);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UntrustedExecutor {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UntrustedExecutor> for UnderlyingRustTuple<'_> {
            fn from(value: UntrustedExecutor) -> Self {
                (value.caller,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UntrustedExecutor {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { caller: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UntrustedExecutor {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UntrustedExecutor(address)";
            const SELECTOR: [u8; 4] = [80u8, 206u8, 62u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.caller,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UntrustedSender(bytes32)` and selector `0xc21fa2e5`.
```solidity
error UntrustedSender(bytes32 messageSender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UntrustedSender {
        #[allow(missing_docs)]
        pub messageSender: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UntrustedSender> for UnderlyingRustTuple<'_> {
            fn from(value: UntrustedSender) -> Self {
                (value.messageSender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UntrustedSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { messageSender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UntrustedSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UntrustedSender(bytes32)";
            const SELECTOR: [u8; 4] = [194u8, 31u8, 162u8, 229u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageSender),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UntrustedSourceDomain(uint32)` and selector `0x6a96659e`.
```solidity
error UntrustedSourceDomain(uint32 sourceDomain);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UntrustedSourceDomain {
        #[allow(missing_docs)]
        pub sourceDomain: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UntrustedSourceDomain> for UnderlyingRustTuple<'_> {
            fn from(value: UntrustedSourceDomain) -> Self {
                (value.sourceDomain,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UntrustedSourceDomain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sourceDomain: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UntrustedSourceDomain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UntrustedSourceDomain(uint32)";
            const SELECTOR: [u8; 4] = [106u8, 150u8, 101u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.sourceDomain),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongBridgeType(uint8,uint8)` and selector `0xb2c3b6fd`.
```solidity
error WrongBridgeType(uint8 expected, uint8 got);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongBridgeType {
        #[allow(missing_docs)]
        pub expected: u8,
        #[allow(missing_docs)]
        pub got: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<8>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8, u8);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WrongBridgeType> for UnderlyingRustTuple<'_> {
            fn from(value: WrongBridgeType) -> Self {
                (value.expected, value.got)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongBridgeType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    got: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongBridgeType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongBridgeType(uint8,uint8)";
            const SELECTOR: [u8; 4] = [178u8, 195u8, 182u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.expected),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.got),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongDestinationChain(uint32)` and selector `0x8dae2d2b`.
```solidity
error WrongDestinationChain(uint32 expected);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongDestinationChain {
        #[allow(missing_docs)]
        pub expected: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WrongDestinationChain> for UnderlyingRustTuple<'_> {
            fn from(value: WrongDestinationChain) -> Self {
                (value.expected,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongDestinationChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { expected: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongDestinationChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongDestinationChain(uint32)";
            const SELECTOR: [u8; 4] = [141u8, 174u8, 45u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.expected),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongOutputToken(bytes32)` and selector `0x2e775c7c`.
```solidity
error WrongOutputToken(bytes32 outputToken);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongOutputToken {
        #[allow(missing_docs)]
        pub outputToken: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WrongOutputToken> for UnderlyingRustTuple<'_> {
            fn from(value: WrongOutputToken) -> Self {
                (value.outputToken,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongOutputToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { outputToken: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongOutputToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongOutputToken(bytes32)";
            const SELECTOR: [u8; 4] = [46u8, 119u8, 92u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputToken),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroCctpExecutor()` and selector `0x69504bd0`.
```solidity
error ZeroCctpExecutor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroCctpExecutor;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroCctpExecutor> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroCctpExecutor) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroCctpExecutor {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroCctpExecutor {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroCctpExecutor()";
            const SELECTOR: [u8; 4] = [105u8, 80u8, 75u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroRecipient()` and selector `0xd27b4443`.
```solidity
error ZeroRecipient();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroRecipient;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroRecipient> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroRecipient) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroRecipient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroRecipient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroRecipient()";
            const SELECTOR: [u8; 4] = [210u8, 123u8, 68u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Claimed(address,address,uint256)` and selector `0xf7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd3992683`.
```solidity
event Claimed(address indexed account, address indexed token, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Claimed {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Claimed {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Claimed(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                247u8, 164u8, 0u8, 119u8, 255u8, 122u8, 4u8, 199u8, 230u8, 31u8, 111u8,
                38u8, 251u8, 19u8, 119u8, 66u8, 89u8, 221u8, 241u8, 182u8, 188u8, 233u8,
                236u8, 242u8, 106u8, 130u8, 118u8, 205u8, 211u8, 153u8, 38u8, 131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    token: topics.2,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone(), self.token.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.token,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Claimed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Claimed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Claimed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DestinationCallback(bytes32,address,uint8)` and selector `0x9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9`.
```solidity
event DestinationCallback(bytes32 indexed id, address indexed fundsTo, CallbackResult result);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DestinationCallback {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub fundsTo: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub result: <CallbackResult as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DestinationCallback {
            type DataTuple<'a> = (CallbackResult,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DestinationCallback(bytes32,address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                158u8, 61u8, 157u8, 59u8, 191u8, 221u8, 70u8, 223u8, 20u8, 85u8, 124u8,
                66u8, 124u8, 120u8, 160u8, 202u8, 126u8, 41u8, 80u8, 170u8, 113u8, 84u8,
                171u8, 247u8, 19u8, 245u8, 183u8, 9u8, 158u8, 186u8, 18u8, 169u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    fundsTo: topics.2,
                    result: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<CallbackResult as alloy_sol_types::SolType>::tokenize(&self.result),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.fundsTo.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.fundsTo,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DestinationCallback {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DestinationCallback> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DestinationCallback) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderCreated(bytes32,uint8,address,uint32,bytes32,uint256,uint64)` and selector `0x2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a`.
```solidity
event OrderCreated(bytes32 indexed orderId, uint8 bridgeType, address indexed sender, uint32 dstChainId, bytes32 outputToken, uint256 outputAmount, uint64 nonce);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderCreated {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub bridgeType: u8,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub dstChainId: u32,
        #[allow(missing_docs)]
        pub outputToken: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub outputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nonce: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OrderCreated(bytes32,uint8,address,uint32,bytes32,uint256,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                41u8, 85u8, 160u8, 199u8, 161u8, 251u8, 175u8, 206u8, 169u8, 78u8, 10u8,
                214u8, 210u8, 196u8, 132u8, 74u8, 31u8, 23u8, 159u8, 41u8, 99u8, 68u8,
                16u8, 183u8, 93u8, 227u8, 165u8, 162u8, 151u8, 127u8, 234u8, 26u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    bridgeType: data.0,
                    sender: topics.2,
                    dstChainId: data.1,
                    outputToken: data.2,
                    outputAmount: data.3,
                    nonce: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bridgeType),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.dstChainId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputToken),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderFilled(bytes32,address,uint256,uint256,uint40)` and selector `0xb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d`.
```solidity
event OrderFilled(bytes32 indexed orderId, address indexed filler, uint256 payoutToRecipient, uint256 feeToFiller, uint40 fillTime);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderFilled {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub filler: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub payoutToRecipient: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub feeToFiller: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fillTime: alloy::sol_types::private::primitives::aliases::U40,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderFilled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<40>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OrderFilled(bytes32,address,uint256,uint256,uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                182u8, 122u8, 11u8, 139u8, 20u8, 68u8, 105u8, 228u8, 4u8, 194u8, 44u8,
                119u8, 196u8, 232u8, 107u8, 151u8, 69u8, 169u8, 189u8, 146u8, 138u8,
                78u8, 134u8, 167u8, 153u8, 51u8, 231u8, 177u8, 105u8, 102u8, 183u8, 141u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    filler: topics.2,
                    payoutToRecipient: data.0,
                    feeToFiller: data.1,
                    fillTime: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.payoutToRecipient),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeToFiller),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.fillTime),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.filler.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.filler,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderFilled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderFilled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderFilled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OrderSettled(bytes32,address,uint256,uint256)` and selector `0x26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a`.
```solidity
event OrderSettled(bytes32 indexed orderId, address indexed filler, uint256 arrivedAmount, uint256 surplusToRecipient);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OrderSettled {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub filler: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub arrivedAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub surplusToRecipient: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OrderSettled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OrderSettled(bytes32,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                38u8, 235u8, 188u8, 162u8, 147u8, 173u8, 98u8, 165u8, 108u8, 214u8,
                171u8, 163u8, 44u8, 189u8, 16u8, 193u8, 28u8, 60u8, 237u8, 108u8, 215u8,
                56u8, 220u8, 203u8, 168u8, 17u8, 216u8, 237u8, 215u8, 153u8, 26u8, 154u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    filler: topics.2,
                    arrivedAmount: data.0,
                    surplusToRecipient: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.arrivedAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.surplusToRecipient),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.filler.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.filler,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OrderSettled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OrderSettled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OrderSettled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipHandoverCanceled(address)` and selector `0xfa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c92`.
```solidity
event OwnershipHandoverCanceled(address indexed pendingOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipHandoverCanceled {
        #[allow(missing_docs)]
        pub pendingOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipHandoverCanceled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipHandoverCanceled(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                250u8, 123u8, 142u8, 171u8, 125u8, 166u8, 127u8, 65u8, 44u8, 201u8, 87u8,
                94u8, 212u8, 52u8, 100u8, 70u8, 143u8, 155u8, 251u8, 174u8, 137u8, 209u8,
                103u8, 89u8, 23u8, 52u8, 108u8, 166u8, 216u8, 254u8, 60u8, 146u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { pendingOwner: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.pendingOwner.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.pendingOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipHandoverCanceled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipHandoverCanceled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OwnershipHandoverCanceled,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipHandoverRequested(address)` and selector `0xdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d`.
```solidity
event OwnershipHandoverRequested(address indexed pendingOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipHandoverRequested {
        #[allow(missing_docs)]
        pub pendingOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipHandoverRequested {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipHandoverRequested(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                219u8, 243u8, 106u8, 16u8, 125u8, 161u8, 158u8, 73u8, 82u8, 122u8, 113u8,
                118u8, 161u8, 186u8, 191u8, 150u8, 59u8, 75u8, 15u8, 248u8, 205u8, 227u8,
                94u8, 227u8, 93u8, 108u8, 216u8, 241u8, 249u8, 172u8, 126u8, 29u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { pendingOwner: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.pendingOwner.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.pendingOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipHandoverRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipHandoverRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OwnershipHandoverRequested,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub oldOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.oldOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.oldOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PayoutDeferred(bytes32,address,address,uint256)` and selector `0x9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f`.
```solidity
event PayoutDeferred(bytes32 indexed id, address indexed to, address indexed token, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PayoutDeferred {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PayoutDeferred {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PayoutDeferred(bytes32,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                148u8, 40u8, 117u8, 124u8, 23u8, 55u8, 119u8, 138u8, 81u8, 147u8, 25u8,
                231u8, 156u8, 33u8, 6u8, 112u8, 196u8, 147u8, 147u8, 191u8, 190u8, 251u8,
                132u8, 16u8, 160u8, 173u8, 185u8, 103u8, 13u8, 203u8, 101u8, 159u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    to: topics.2,
                    token: topics.3,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.id.clone(),
                    self.to.clone(),
                    self.token.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.token,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PayoutDeferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PayoutDeferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PayoutDeferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address config_, address owner_, uint256 maxFeeRate_, address cctpExecutor_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub config_: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub owner_: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxFeeRate_: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cctpExecutor_: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.config_, value.owner_, value.maxFeeRate_, value.cctpExecutor_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        config_: tuple.0,
                        owner_: tuple.1,
                        maxFeeRate_: tuple.2,
                        cctpExecutor_: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.config_,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner_,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFeeRate_),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.cctpExecutor_,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `FINALITY_FAST()` and selector `0xcc6eec70`.
```solidity
function FINALITY_FAST() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FINALITY_FASTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`FINALITY_FAST()`](FINALITY_FASTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FINALITY_FASTReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FINALITY_FASTCall> for UnderlyingRustTuple<'_> {
                fn from(value: FINALITY_FASTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for FINALITY_FASTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FINALITY_FASTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: FINALITY_FASTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for FINALITY_FASTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for FINALITY_FASTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FINALITY_FAST()";
            const SELECTOR: [u8; 4] = [204u8, 110u8, 236u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: FINALITY_FASTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: FINALITY_FASTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `FINALITY_FINALIZED()` and selector `0x77839a9e`.
```solidity
function FINALITY_FINALIZED() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FINALITY_FINALIZEDCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`FINALITY_FINALIZED()`](FINALITY_FINALIZEDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FINALITY_FINALIZEDReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FINALITY_FINALIZEDCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: FINALITY_FINALIZEDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FINALITY_FINALIZEDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FINALITY_FINALIZEDReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: FINALITY_FINALIZEDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FINALITY_FINALIZEDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for FINALITY_FINALIZEDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FINALITY_FINALIZED()";
            const SELECTOR: [u8; 4] = [119u8, 131u8, 154u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: FINALITY_FINALIZEDReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: FINALITY_FINALIZEDReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MAX_CALLBACK_GAS_LIMIT()` and selector `0x39c33215`.
```solidity
function MAX_CALLBACK_GAS_LIMIT() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_CALLBACK_GAS_LIMITCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_CALLBACK_GAS_LIMIT()`](MAX_CALLBACK_GAS_LIMITCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_CALLBACK_GAS_LIMITReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_CALLBACK_GAS_LIMITCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_CALLBACK_GAS_LIMITCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_CALLBACK_GAS_LIMITCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_CALLBACK_GAS_LIMITReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_CALLBACK_GAS_LIMITReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_CALLBACK_GAS_LIMITReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_CALLBACK_GAS_LIMITCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_CALLBACK_GAS_LIMIT()";
            const SELECTOR: [u8; 4] = [57u8, 195u8, 50u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MAX_CALLBACK_GAS_LIMITReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: MAX_CALLBACK_GAS_LIMITReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `PERMIT2()` and selector `0x6afdd850`.
```solidity
function PERMIT2() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERMIT2Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`PERMIT2()`](PERMIT2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PERMIT2Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PERMIT2Call> for UnderlyingRustTuple<'_> {
                fn from(value: PERMIT2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PERMIT2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PERMIT2Return> for UnderlyingRustTuple<'_> {
                fn from(value: PERMIT2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PERMIT2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PERMIT2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PERMIT2()";
            const SELECTOR: [u8; 4] = [106u8, 253u8, 216u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: PERMIT2Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: PERMIT2Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `_executeDelivery(address,address,uint256,uint256,bytes)` and selector `0xac4fca82`.
```solidity
function _executeDelivery(address token, address recipient, uint256 amount, uint256 gasLimit, bytes memory callbackData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _executeDeliveryCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub gasLimit: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub callbackData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`_executeDelivery(address,address,uint256,uint256,bytes)`](_executeDeliveryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _executeDeliveryReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<_executeDeliveryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: _executeDeliveryCall) -> Self {
                    (
                        value.token,
                        value.recipient,
                        value.amount,
                        value.gasLimit,
                        value.callbackData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _executeDeliveryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        recipient: tuple.1,
                        amount: tuple.2,
                        gasLimit: tuple.3,
                        callbackData: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<_executeDeliveryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: _executeDeliveryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _executeDeliveryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl _executeDeliveryReturn {
            fn _tokenize(
                &self,
            ) -> <_executeDeliveryCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _executeDeliveryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = _executeDeliveryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_executeDelivery(address,address,uint256,uint256,bytes)";
            const SELECTOR: [u8; 4] = [172u8, 79u8, 202u8, 130u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasLimit),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.callbackData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                _executeDeliveryReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelOwnershipHandover()` and selector `0x54d1f13d`.
```solidity
function cancelOwnershipHandover() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelOwnershipHandoverCall;
    ///Container type for the return parameters of the [`cancelOwnershipHandover()`](cancelOwnershipHandoverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelOwnershipHandoverReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelOwnershipHandoverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelOwnershipHandoverCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelOwnershipHandoverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cancelOwnershipHandoverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelOwnershipHandoverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelOwnershipHandoverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelOwnershipHandoverReturn {
            fn _tokenize(
                &self,
            ) -> <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelOwnershipHandoverCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelOwnershipHandoverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelOwnershipHandover()";
            const SELECTOR: [u8; 4] = [84u8, 209u8, 241u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelOwnershipHandoverReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cctpExecutor()` and selector `0xdcb41c28`.
```solidity
function cctpExecutor() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cctpExecutorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`cctpExecutor()`](cctpExecutorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cctpExecutorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cctpExecutorCall> for UnderlyingRustTuple<'_> {
                fn from(value: cctpExecutorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cctpExecutorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cctpExecutorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cctpExecutorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cctpExecutorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cctpExecutorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cctpExecutor()";
            const SELECTOR: [u8; 4] = [220u8, 180u8, 28u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: cctpExecutorReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: cctpExecutorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `claim(address)` and selector `0x1e83409a`.
```solidity
function claim(address token) external returns (uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claim(address)`](claimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimReturn {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimCall) -> Self {
                    (value.token,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { token: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimReturn) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claim(address)";
            const SELECTOR: [u8; 4] = [30u8, 131u8, 64u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: claimReturn = r.into();
                        r.amount
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: claimReturn = r.into();
                        r.amount
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `claimable(address,address)` and selector `0xd4570c1c`.
```solidity
function claimable(address account, address token) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimableCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claimable(address,address)`](claimableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimableReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimableCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimableCall) -> Self {
                    (value.account, value.token)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        token: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimableReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimableReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimableCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimable(address,address)";
            const SELECTOR: [u8; 4] = [212u8, 87u8, 12u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: claimableReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: claimableReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `completeOwnershipHandover(address)` and selector `0xf04e283e`.
```solidity
function completeOwnershipHandover(address pendingOwner) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeOwnershipHandoverCall {
        #[allow(missing_docs)]
        pub pendingOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`completeOwnershipHandover(address)`](completeOwnershipHandoverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeOwnershipHandoverReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeOwnershipHandoverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeOwnershipHandoverCall) -> Self {
                    (value.pendingOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeOwnershipHandoverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pendingOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeOwnershipHandoverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeOwnershipHandoverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeOwnershipHandoverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl completeOwnershipHandoverReturn {
            fn _tokenize(
                &self,
            ) -> <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeOwnershipHandoverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeOwnershipHandoverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeOwnershipHandover(address)";
            const SELECTOR: [u8; 4] = [240u8, 78u8, 40u8, 62u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.pendingOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                completeOwnershipHandoverReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `config()` and selector `0x79502c55`.
```solidity
function config() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`config()`](configCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<configCall> for UnderlyingRustTuple<'_> {
                fn from(value: configCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<configReturn> for UnderlyingRustTuple<'_> {
                fn from(value: configReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "config()";
            const SELECTOR: [u8; 4] = [121u8, 80u8, 44u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: configReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: configReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `fill((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes))` and selector `0x31eee44d`.
```solidity
function fill(Order memory order) external returns (bytes32 orderId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fillCall {
        #[allow(missing_docs)]
        pub order: <Order as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`fill((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes))`](fillCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fillReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Order,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Order as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<fillCall> for UnderlyingRustTuple<'_> {
                fn from(value: fillCall) -> Self {
                    (value.order,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fillCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { order: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<fillReturn> for UnderlyingRustTuple<'_> {
                fn from(value: fillReturn) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fillReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fillCall {
            type Parameters<'a> = (Order,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fill((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes))";
            const SELECTOR: [u8; 4] = [49u8, 238u8, 228u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Order as alloy_sol_types::SolType>::tokenize(&self.order),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: fillReturn = r.into();
                        r.orderId
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: fillReturn = r.into();
                        r.orderId
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `fillFor((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes),address,(uint256,uint256,bytes))` and selector `0x776ff3c7`.
```solidity
function fillFor(Order memory order, address filler, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fillForCall {
        #[allow(missing_docs)]
        pub order: <Order as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub filler: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permit: <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`fillFor((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes),address,(uint256,uint256,bytes))`](fillForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fillForReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                Order,
                alloy::sol_types::sol_data::Address,
                PermitLib::Permit2Data,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Order as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<fillForCall> for UnderlyingRustTuple<'_> {
                fn from(value: fillForCall) -> Self {
                    (value.order, value.filler, value.permit)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fillForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        order: tuple.0,
                        filler: tuple.1,
                        permit: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<fillForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: fillForReturn) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for fillForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fillForCall {
            type Parameters<'a> = (
                Order,
                alloy::sol_types::sol_data::Address,
                PermitLib::Permit2Data,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fillFor((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes),address,(uint256,uint256,bytes))";
            const SELECTOR: [u8; 4] = [119u8, 111u8, 243u8, 199u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Order as alloy_sol_types::SolType>::tokenize(&self.order),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.filler,
                    ),
                    <PermitLib::Permit2Data as alloy_sol_types::SolType>::tokenize(
                        &self.permit,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: fillForReturn = r.into();
                        r.orderId
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: fillForReturn = r.into();
                        r.orderId
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOrder(bytes32)` and selector `0x5778472a`.
```solidity
function getOrder(bytes32 orderId) external view returns (OrderRecord memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOrderCall {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOrder(bytes32)`](getOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOrderReturn {
        #[allow(missing_docs)]
        pub _0: <OrderRecord as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOrderCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOrderCall) -> Self {
                    (value.orderId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { orderId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (OrderRecord,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OrderRecord as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOrderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOrderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <OrderRecord as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (OrderRecord,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOrder(bytes32)";
            const SELECTOR: [u8; 4] = [87u8, 120u8, 71u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<OrderRecord as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getOrderReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getOrderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initiateCCTP(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))` and selector `0xeafa61a8`.
```solidity
function initiateCCTP(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint256 mintFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec) external returns (bytes32 orderId, uint64 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateCCTPCall {
        #[allow(missing_docs)]
        pub dstChainId: u32,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub mintFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minFinalityThreshold: u32,
        #[allow(missing_docs)]
        pub deliveryWindow: u64,
        #[allow(missing_docs)]
        pub discountRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub exec: <Execution as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`initiateCCTP(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))`](initiateCCTPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateCCTPReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub nonce: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Execution,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u32,
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <Execution as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initiateCCTPCall> for UnderlyingRustTuple<'_> {
                fn from(value: initiateCCTPCall) -> Self {
                    (
                        value.dstChainId,
                        value.recipient,
                        value.inputAmount,
                        value.maxFee,
                        value.mintFee,
                        value.minFinalityThreshold,
                        value.deliveryWindow,
                        value.discountRate,
                        value.baseFee,
                        value.exec,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initiateCCTPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dstChainId: tuple.0,
                        recipient: tuple.1,
                        inputAmount: tuple.2,
                        maxFee: tuple.3,
                        mintFee: tuple.4,
                        minFinalityThreshold: tuple.5,
                        deliveryWindow: tuple.6,
                        discountRate: tuple.7,
                        baseFee: tuple.8,
                        exec: tuple.9,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u64,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initiateCCTPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initiateCCTPReturn) -> Self {
                    (value.orderId, value.nonce)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initiateCCTPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        nonce: tuple.1,
                    }
                }
            }
        }
        impl initiateCCTPReturn {
            fn _tokenize(
                &self,
            ) -> <initiateCCTPCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initiateCCTPCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Execution,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initiateCCTPReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initiateCCTP(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))";
            const SELECTOR: [u8; 4] = [234u8, 250u8, 97u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.dstChainId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.recipient),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintFee),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minFinalityThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.deliveryWindow),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.discountRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFee),
                    <Execution as alloy_sol_types::SolType>::tokenize(&self.exec),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initiateCCTPReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initiateCCTPFor(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))` and selector `0x68114370`.
```solidity
function initiateCCTPFor(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint256 mintFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec, address from, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId, uint64 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateCCTPForCall {
        #[allow(missing_docs)]
        pub dstChainId: u32,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub mintFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minFinalityThreshold: u32,
        #[allow(missing_docs)]
        pub deliveryWindow: u64,
        #[allow(missing_docs)]
        pub discountRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub exec: <Execution as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permit: <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`initiateCCTPFor(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))`](initiateCCTPForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateCCTPForReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub nonce: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Execution,
                alloy::sol_types::sol_data::Address,
                PermitLib::Permit2Data,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u32,
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <Execution as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initiateCCTPForCall> for UnderlyingRustTuple<'_> {
                fn from(value: initiateCCTPForCall) -> Self {
                    (
                        value.dstChainId,
                        value.recipient,
                        value.inputAmount,
                        value.maxFee,
                        value.mintFee,
                        value.minFinalityThreshold,
                        value.deliveryWindow,
                        value.discountRate,
                        value.baseFee,
                        value.exec,
                        value.from,
                        value.permit,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initiateCCTPForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dstChainId: tuple.0,
                        recipient: tuple.1,
                        inputAmount: tuple.2,
                        maxFee: tuple.3,
                        mintFee: tuple.4,
                        minFinalityThreshold: tuple.5,
                        deliveryWindow: tuple.6,
                        discountRate: tuple.7,
                        baseFee: tuple.8,
                        exec: tuple.9,
                        from: tuple.10,
                        permit: tuple.11,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u64,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initiateCCTPForReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initiateCCTPForReturn) -> Self {
                    (value.orderId, value.nonce)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initiateCCTPForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        nonce: tuple.1,
                    }
                }
            }
        }
        impl initiateCCTPForReturn {
            fn _tokenize(
                &self,
            ) -> <initiateCCTPForCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initiateCCTPForCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Execution,
                alloy::sol_types::sol_data::Address,
                PermitLib::Permit2Data,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initiateCCTPForReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initiateCCTPFor(uint32,bytes32,uint256,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))";
            const SELECTOR: [u8; 4] = [104u8, 17u8, 67u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.dstChainId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.recipient),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintFee),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.minFinalityThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.deliveryWindow),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.discountRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseFee),
                    <Execution as alloy_sol_types::SolType>::tokenize(&self.exec),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <PermitLib::Permit2Data as alloy_sol_types::SolType>::tokenize(
                        &self.permit,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initiateCCTPForReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maxFeeRate()` and selector `0x85c17830`.
```solidity
function maxFeeRate() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxFeeRateCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maxFeeRate()`](maxFeeRateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxFeeRateReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxFeeRateCall> for UnderlyingRustTuple<'_> {
                fn from(value: maxFeeRateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxFeeRateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxFeeRateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: maxFeeRateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxFeeRateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxFeeRateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxFeeRate()";
            const SELECTOR: [u8; 4] = [133u8, 193u8, 120u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: maxFeeRateReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: maxFeeRateReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `multicall(bytes[])` and selector `0xac9650d8`.
```solidity
function multicall(bytes[] memory data) external payable returns (bytes[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallCall {
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`multicall(bytes[])`](multicallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<multicallCall> for UnderlyingRustTuple<'_> {
                fn from(value: multicallCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<multicallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multicallReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multicallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multicall(bytes[])";
            const SELECTOR: [u8; 4] = [172u8, 150u8, 80u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: multicallReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: multicallReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `onCctpExecute(uint32,bytes32,address,uint256,bytes)` and selector `0x928c60b9`.
```solidity
function onCctpExecute(uint32 sourceDomain, bytes32 sender, address usdc, uint256 amount, bytes memory payload) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onCctpExecuteCall {
        #[allow(missing_docs)]
        pub sourceDomain: u32,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub usdc: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub payload: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`onCctpExecute(uint32,bytes32,address,uint256,bytes)`](onCctpExecuteCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onCctpExecuteReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onCctpExecuteCall> for UnderlyingRustTuple<'_> {
                fn from(value: onCctpExecuteCall) -> Self {
                    (
                        value.sourceDomain,
                        value.sender,
                        value.usdc,
                        value.amount,
                        value.payload,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onCctpExecuteCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sourceDomain: tuple.0,
                        sender: tuple.1,
                        usdc: tuple.2,
                        amount: tuple.3,
                        payload: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<onCctpExecuteReturn> for UnderlyingRustTuple<'_> {
                fn from(value: onCctpExecuteReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for onCctpExecuteReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl onCctpExecuteReturn {
            fn _tokenize(
                &self,
            ) -> <onCctpExecuteCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onCctpExecuteCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onCctpExecuteReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onCctpExecute(uint32,bytes32,address,uint256,bytes)";
            const SELECTOR: [u8; 4] = [146u8, 140u8, 96u8, 185u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.sourceDomain),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.sender),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.usdc,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                onCctpExecuteReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address result);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value.result,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { result: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r.result
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownerReturn = r.into();
                        r.result
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ownershipHandoverExpiresAt(address)` and selector `0xfee81cf4`.
```solidity
function ownershipHandoverExpiresAt(address pendingOwner) external view returns (uint256 result);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownershipHandoverExpiresAtCall {
        #[allow(missing_docs)]
        pub pendingOwner: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ownershipHandoverExpiresAt(address)`](ownershipHandoverExpiresAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownershipHandoverExpiresAtReturn {
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownershipHandoverExpiresAtCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ownershipHandoverExpiresAtCall) -> Self {
                    (value.pendingOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ownershipHandoverExpiresAtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pendingOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownershipHandoverExpiresAtReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ownershipHandoverExpiresAtReturn) -> Self {
                    (value.result,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ownershipHandoverExpiresAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { result: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownershipHandoverExpiresAtCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ownershipHandoverExpiresAt(address)";
            const SELECTOR: [u8; 4] = [254u8, 232u8, 28u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.pendingOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: ownershipHandoverExpiresAtReturn = r.into();
                        r.result
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: ownershipHandoverExpiresAtReturn = r.into();
                        r.result
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `quoteFill((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes),uint256)` and selector `0x97c36bae`.
```solidity
function quoteFill(Order memory order, uint256 fillTime) external view returns (uint256 payoutToRecipient, uint256 feeToFiller);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteFillCall {
        #[allow(missing_docs)]
        pub order: <Order as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub fillTime: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`quoteFill((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes),uint256)`](quoteFillCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quoteFillReturn {
        #[allow(missing_docs)]
        pub payoutToRecipient: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub feeToFiller: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (Order, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Order as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteFillCall> for UnderlyingRustTuple<'_> {
                fn from(value: quoteFillCall) -> Self {
                    (value.order, value.fillTime)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quoteFillCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        order: tuple.0,
                        fillTime: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quoteFillReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quoteFillReturn) -> Self {
                    (value.payoutToRecipient, value.feeToFiller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quoteFillReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        payoutToRecipient: tuple.0,
                        feeToFiller: tuple.1,
                    }
                }
            }
        }
        impl quoteFillReturn {
            fn _tokenize(
                &self,
            ) -> <quoteFillCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.payoutToRecipient),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeToFiller),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quoteFillCall {
            type Parameters<'a> = (Order, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quoteFillReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quoteFill((uint8,uint32,uint32,bytes32,bytes32,bytes32,bytes32,uint256,uint256,uint64,uint64,uint64,uint256,uint256,uint64,bytes),uint256)";
            const SELECTOR: [u8; 4] = [151u8, 195u8, 107u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Order as alloy_sol_types::SolType>::tokenize(&self.order),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fillTime),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                quoteFillReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `requestOwnershipHandover()` and selector `0x25692962`.
```solidity
function requestOwnershipHandover() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestOwnershipHandoverCall;
    ///Container type for the return parameters of the [`requestOwnershipHandover()`](requestOwnershipHandoverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestOwnershipHandoverReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestOwnershipHandoverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestOwnershipHandoverCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestOwnershipHandoverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestOwnershipHandoverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestOwnershipHandoverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestOwnershipHandoverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl requestOwnershipHandoverReturn {
            fn _tokenize(
                &self,
            ) -> <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestOwnershipHandoverCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestOwnershipHandoverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestOwnershipHandover()";
            const SELECTOR: [u8; 4] = [37u8, 105u8, 41u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                requestOwnershipHandoverReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3995c67`.
```solidity
function selfPermit(address token, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct selfPermitCall {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)`](selfPermitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct selfPermitReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<selfPermitCall> for UnderlyingRustTuple<'_> {
                fn from(value: selfPermitCall) -> Self {
                    (value.token, value.value, value.deadline, value.v, value.r, value.s)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for selfPermitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        value: tuple.1,
                        deadline: tuple.2,
                        v: tuple.3,
                        r: tuple.4,
                        s: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<selfPermitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: selfPermitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for selfPermitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl selfPermitReturn {
            fn _tokenize(
                &self,
            ) -> <selfPermitCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for selfPermitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = selfPermitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [243u8, 153u8, 92u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                selfPermitReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setMaxFeeRate(uint256)` and selector `0x8cda96de`.
```solidity
function setMaxFeeRate(uint256 newMaxFeeRate) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMaxFeeRateCall {
        #[allow(missing_docs)]
        pub newMaxFeeRate: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setMaxFeeRate(uint256)`](setMaxFeeRateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMaxFeeRateReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMaxFeeRateCall> for UnderlyingRustTuple<'_> {
                fn from(value: setMaxFeeRateCall) -> Self {
                    (value.newMaxFeeRate,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMaxFeeRateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newMaxFeeRate: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMaxFeeRateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setMaxFeeRateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMaxFeeRateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setMaxFeeRateReturn {
            fn _tokenize(
                &self,
            ) -> <setMaxFeeRateCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMaxFeeRateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMaxFeeRateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMaxFeeRate(uint256)";
            const SELECTOR: [u8; 4] = [140u8, 218u8, 150u8, 222u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMaxFeeRate),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setMaxFeeRateReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setPaused(bool)` and selector `0x16c38b3c`.
```solidity
function setPaused(bool newPaused) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPausedCall {
        #[allow(missing_docs)]
        pub newPaused: bool,
    }
    ///Container type for the return parameters of the [`setPaused(bool)`](setPausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPausedReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPausedCall) -> Self {
                    (value.newPaused,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPaused: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setPausedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setPausedReturn {
            fn _tokenize(
                &self,
            ) -> <setPausedCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPausedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPausedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPaused(bool)";
            const SELECTOR: [u8; 4] = [22u8, 195u8, 139u8, 60u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.newPaused,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setPausedReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `settle(bytes,bytes)` and selector `0x5fdc7c12`.
```solidity
function settle(bytes memory message, bytes memory attestation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct settleCall {
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub attestation: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`settle(bytes,bytes)`](settleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct settleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<settleCall> for UnderlyingRustTuple<'_> {
                fn from(value: settleCall) -> Self {
                    (value.message, value.attestation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        message: tuple.0,
                        attestation: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<settleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: settleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl settleReturn {
            fn _tokenize(
                &self,
            ) -> <settleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for settleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = settleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "settle(bytes,bytes)";
            const SELECTOR: [u8; 4] = [95u8, 220u8, 124u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.attestation,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                settleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`CctpAdapter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum CctpAdapterCalls {
        #[allow(missing_docs)]
        FINALITY_FAST(FINALITY_FASTCall),
        #[allow(missing_docs)]
        FINALITY_FINALIZED(FINALITY_FINALIZEDCall),
        #[allow(missing_docs)]
        MAX_CALLBACK_GAS_LIMIT(MAX_CALLBACK_GAS_LIMITCall),
        #[allow(missing_docs)]
        PERMIT2(PERMIT2Call),
        #[allow(missing_docs)]
        _executeDelivery(_executeDeliveryCall),
        #[allow(missing_docs)]
        cancelOwnershipHandover(cancelOwnershipHandoverCall),
        #[allow(missing_docs)]
        cctpExecutor(cctpExecutorCall),
        #[allow(missing_docs)]
        claim(claimCall),
        #[allow(missing_docs)]
        claimable(claimableCall),
        #[allow(missing_docs)]
        completeOwnershipHandover(completeOwnershipHandoverCall),
        #[allow(missing_docs)]
        config(configCall),
        #[allow(missing_docs)]
        fill(fillCall),
        #[allow(missing_docs)]
        fillFor(fillForCall),
        #[allow(missing_docs)]
        getOrder(getOrderCall),
        #[allow(missing_docs)]
        initiateCCTP(initiateCCTPCall),
        #[allow(missing_docs)]
        initiateCCTPFor(initiateCCTPForCall),
        #[allow(missing_docs)]
        maxFeeRate(maxFeeRateCall),
        #[allow(missing_docs)]
        multicall(multicallCall),
        #[allow(missing_docs)]
        onCctpExecute(onCctpExecuteCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        ownershipHandoverExpiresAt(ownershipHandoverExpiresAtCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        quoteFill(quoteFillCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        requestOwnershipHandover(requestOwnershipHandoverCall),
        #[allow(missing_docs)]
        selfPermit(selfPermitCall),
        #[allow(missing_docs)]
        setMaxFeeRate(setMaxFeeRateCall),
        #[allow(missing_docs)]
        setPaused(setPausedCall),
        #[allow(missing_docs)]
        settle(settleCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
    }
    impl CctpAdapterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [22u8, 195u8, 139u8, 60u8],
            [30u8, 131u8, 64u8, 154u8],
            [37u8, 105u8, 41u8, 98u8],
            [49u8, 238u8, 228u8, 77u8],
            [57u8, 195u8, 50u8, 21u8],
            [84u8, 209u8, 241u8, 61u8],
            [87u8, 120u8, 71u8, 42u8],
            [92u8, 151u8, 90u8, 187u8],
            [95u8, 220u8, 124u8, 18u8],
            [104u8, 17u8, 67u8, 112u8],
            [106u8, 253u8, 216u8, 80u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 111u8, 243u8, 199u8],
            [119u8, 131u8, 154u8, 158u8],
            [121u8, 80u8, 44u8, 85u8],
            [133u8, 193u8, 120u8, 48u8],
            [140u8, 218u8, 150u8, 222u8],
            [141u8, 165u8, 203u8, 91u8],
            [146u8, 140u8, 96u8, 185u8],
            [151u8, 195u8, 107u8, 174u8],
            [172u8, 79u8, 202u8, 130u8],
            [172u8, 150u8, 80u8, 216u8],
            [204u8, 110u8, 236u8, 112u8],
            [212u8, 87u8, 12u8, 28u8],
            [220u8, 180u8, 28u8, 40u8],
            [234u8, 250u8, 97u8, 168u8],
            [240u8, 78u8, 40u8, 62u8],
            [242u8, 253u8, 227u8, 139u8],
            [243u8, 153u8, 92u8, 103u8],
            [254u8, 232u8, 28u8, 244u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setPaused),
            ::core::stringify!(claim),
            ::core::stringify!(requestOwnershipHandover),
            ::core::stringify!(fill),
            ::core::stringify!(MAX_CALLBACK_GAS_LIMIT),
            ::core::stringify!(cancelOwnershipHandover),
            ::core::stringify!(getOrder),
            ::core::stringify!(paused),
            ::core::stringify!(settle),
            ::core::stringify!(initiateCCTPFor),
            ::core::stringify!(PERMIT2),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(fillFor),
            ::core::stringify!(FINALITY_FINALIZED),
            ::core::stringify!(config),
            ::core::stringify!(maxFeeRate),
            ::core::stringify!(setMaxFeeRate),
            ::core::stringify!(owner),
            ::core::stringify!(onCctpExecute),
            ::core::stringify!(quoteFill),
            ::core::stringify!(_executeDelivery),
            ::core::stringify!(multicall),
            ::core::stringify!(FINALITY_FAST),
            ::core::stringify!(claimable),
            ::core::stringify!(cctpExecutor),
            ::core::stringify!(initiateCCTP),
            ::core::stringify!(completeOwnershipHandover),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(selfPermit),
            ::core::stringify!(ownershipHandoverExpiresAt),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setPausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimCall as alloy_sol_types::SolCall>::SIGNATURE,
            <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fillCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <settleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initiateCCTPForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PERMIT2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fillForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <configCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setMaxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <onCctpExecuteCall as alloy_sol_types::SolCall>::SIGNATURE,
            <quoteFillCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_executeDeliveryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multicallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <FINALITY_FASTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimableCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cctpExecutorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initiateCCTPCall as alloy_sol_types::SolCall>::SIGNATURE,
            <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <selfPermitCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CctpAdapterCalls {
        const NAME: &'static str = "CctpAdapterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::FINALITY_FAST(_) => {
                    <FINALITY_FASTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::FINALITY_FINALIZED(_) => {
                    <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_CALLBACK_GAS_LIMIT(_) => {
                    <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PERMIT2(_) => <PERMIT2Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::_executeDelivery(_) => {
                    <_executeDeliveryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelOwnershipHandover(_) => {
                    <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cctpExecutor(_) => {
                    <cctpExecutorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claim(_) => <claimCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::claimable(_) => {
                    <claimableCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeOwnershipHandover(_) => {
                    <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::config(_) => <configCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::fill(_) => <fillCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::fillFor(_) => <fillForCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getOrder(_) => <getOrderCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initiateCCTP(_) => {
                    <initiateCCTPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initiateCCTPFor(_) => {
                    <initiateCCTPForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxFeeRate(_) => {
                    <maxFeeRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multicall(_) => {
                    <multicallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onCctpExecute(_) => {
                    <onCctpExecuteCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ownershipHandoverExpiresAt(_) => {
                    <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quoteFill(_) => {
                    <quoteFillCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestOwnershipHandover(_) => {
                    <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::selfPermit(_) => {
                    <selfPermitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMaxFeeRate(_) => {
                    <setMaxFeeRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPaused(_) => {
                    <setPausedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::settle(_) => <settleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<CctpAdapterCalls>] = &[
                {
                    fn setPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <setPausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::setPaused)
                    }
                    setPaused
                },
                {
                    fn claim(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <claimCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::claim)
                    }
                    claim
                },
                {
                    fn requestOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::requestOwnershipHandover)
                    }
                    requestOwnershipHandover
                },
                {
                    fn fill(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <fillCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::fill)
                    }
                    fill
                },
                {
                    fn MAX_CALLBACK_GAS_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::MAX_CALLBACK_GAS_LIMIT)
                    }
                    MAX_CALLBACK_GAS_LIMIT
                },
                {
                    fn cancelOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::cancelOwnershipHandover)
                    }
                    cancelOwnershipHandover
                },
                {
                    fn getOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <getOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::getOrder)
                    }
                    getOrder
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn settle(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <settleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::settle)
                    }
                    settle
                },
                {
                    fn initiateCCTPFor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <initiateCCTPForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::initiateCCTPFor)
                    }
                    initiateCCTPFor
                },
                {
                    fn PERMIT2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <PERMIT2Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::PERMIT2)
                    }
                    PERMIT2
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn fillFor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <fillForCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::fillFor)
                    }
                    fillFor
                },
                {
                    fn FINALITY_FINALIZED(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::FINALITY_FINALIZED)
                    }
                    FINALITY_FINALIZED
                },
                {
                    fn config(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <configCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::config)
                    }
                    config
                },
                {
                    fn maxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <maxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::maxFeeRate)
                    }
                    maxFeeRate
                },
                {
                    fn setMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <setMaxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::setMaxFeeRate)
                    }
                    setMaxFeeRate
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn onCctpExecute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <onCctpExecuteCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::onCctpExecute)
                    }
                    onCctpExecute
                },
                {
                    fn quoteFill(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <quoteFillCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::quoteFill)
                    }
                    quoteFill
                },
                {
                    fn _executeDelivery(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <_executeDeliveryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::_executeDelivery)
                    }
                    _executeDelivery
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::multicall)
                    }
                    multicall
                },
                {
                    fn FINALITY_FAST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <FINALITY_FASTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::FINALITY_FAST)
                    }
                    FINALITY_FAST
                },
                {
                    fn claimable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <claimableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::claimable)
                    }
                    claimable
                },
                {
                    fn cctpExecutor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <cctpExecutorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::cctpExecutor)
                    }
                    cctpExecutor
                },
                {
                    fn initiateCCTP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <initiateCCTPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::initiateCCTP)
                    }
                    initiateCCTP
                },
                {
                    fn completeOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::completeOwnershipHandover)
                    }
                    completeOwnershipHandover
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn selfPermit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <selfPermitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::selfPermit)
                    }
                    selfPermit
                },
                {
                    fn ownershipHandoverExpiresAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterCalls::ownershipHandoverExpiresAt)
                    }
                    ownershipHandoverExpiresAt
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<CctpAdapterCalls>] = &[
                {
                    fn setPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <setPausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::setPaused)
                    }
                    setPaused
                },
                {
                    fn claim(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <claimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::claim)
                    }
                    claim
                },
                {
                    fn requestOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::requestOwnershipHandover)
                    }
                    requestOwnershipHandover
                },
                {
                    fn fill(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <fillCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::fill)
                    }
                    fill
                },
                {
                    fn MAX_CALLBACK_GAS_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::MAX_CALLBACK_GAS_LIMIT)
                    }
                    MAX_CALLBACK_GAS_LIMIT
                },
                {
                    fn cancelOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::cancelOwnershipHandover)
                    }
                    cancelOwnershipHandover
                },
                {
                    fn getOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <getOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::getOrder)
                    }
                    getOrder
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn settle(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <settleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::settle)
                    }
                    settle
                },
                {
                    fn initiateCCTPFor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <initiateCCTPForCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::initiateCCTPFor)
                    }
                    initiateCCTPFor
                },
                {
                    fn PERMIT2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <PERMIT2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::PERMIT2)
                    }
                    PERMIT2
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn fillFor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <fillForCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::fillFor)
                    }
                    fillFor
                },
                {
                    fn FINALITY_FINALIZED(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::FINALITY_FINALIZED)
                    }
                    FINALITY_FINALIZED
                },
                {
                    fn config(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <configCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::config)
                    }
                    config
                },
                {
                    fn maxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <maxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::maxFeeRate)
                    }
                    maxFeeRate
                },
                {
                    fn setMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <setMaxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::setMaxFeeRate)
                    }
                    setMaxFeeRate
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn onCctpExecute(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <onCctpExecuteCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::onCctpExecute)
                    }
                    onCctpExecute
                },
                {
                    fn quoteFill(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <quoteFillCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::quoteFill)
                    }
                    quoteFill
                },
                {
                    fn _executeDelivery(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <_executeDeliveryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::_executeDelivery)
                    }
                    _executeDelivery
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::multicall)
                    }
                    multicall
                },
                {
                    fn FINALITY_FAST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <FINALITY_FASTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::FINALITY_FAST)
                    }
                    FINALITY_FAST
                },
                {
                    fn claimable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <claimableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::claimable)
                    }
                    claimable
                },
                {
                    fn cctpExecutor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <cctpExecutorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::cctpExecutor)
                    }
                    cctpExecutor
                },
                {
                    fn initiateCCTP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <initiateCCTPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::initiateCCTP)
                    }
                    initiateCCTP
                },
                {
                    fn completeOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::completeOwnershipHandover)
                    }
                    completeOwnershipHandover
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn selfPermit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <selfPermitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::selfPermit)
                    }
                    selfPermit
                },
                {
                    fn ownershipHandoverExpiresAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterCalls::ownershipHandoverExpiresAt)
                    }
                    ownershipHandoverExpiresAt
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::FINALITY_FAST(inner) => {
                    <FINALITY_FASTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FINALITY_FINALIZED(inner) => {
                    <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_CALLBACK_GAS_LIMIT(inner) => {
                    <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PERMIT2(inner) => {
                    <PERMIT2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::_executeDelivery(inner) => {
                    <_executeDeliveryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelOwnershipHandover(inner) => {
                    <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cctpExecutor(inner) => {
                    <cctpExecutorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claim(inner) => {
                    <claimCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::claimable(inner) => {
                    <claimableCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::completeOwnershipHandover(inner) => {
                    <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::config(inner) => {
                    <configCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::fill(inner) => {
                    <fillCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::fillFor(inner) => {
                    <fillForCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getOrder(inner) => {
                    <getOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initiateCCTP(inner) => {
                    <initiateCCTPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initiateCCTPFor(inner) => {
                    <initiateCCTPForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxFeeRate(inner) => {
                    <maxFeeRateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::onCctpExecute(inner) => {
                    <onCctpExecuteCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ownershipHandoverExpiresAt(inner) => {
                    <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quoteFill(inner) => {
                    <quoteFillCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestOwnershipHandover(inner) => {
                    <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::selfPermit(inner) => {
                    <selfPermitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setMaxFeeRate(inner) => {
                    <setMaxFeeRateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPaused(inner) => {
                    <setPausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::settle(inner) => {
                    <settleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::FINALITY_FAST(inner) => {
                    <FINALITY_FASTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FINALITY_FINALIZED(inner) => {
                    <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_CALLBACK_GAS_LIMIT(inner) => {
                    <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PERMIT2(inner) => {
                    <PERMIT2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::_executeDelivery(inner) => {
                    <_executeDeliveryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelOwnershipHandover(inner) => {
                    <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cctpExecutor(inner) => {
                    <cctpExecutorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claim(inner) => {
                    <claimCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::claimable(inner) => {
                    <claimableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeOwnershipHandover(inner) => {
                    <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::config(inner) => {
                    <configCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::fill(inner) => {
                    <fillCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::fillFor(inner) => {
                    <fillForCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getOrder(inner) => {
                    <getOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initiateCCTP(inner) => {
                    <initiateCCTPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initiateCCTPFor(inner) => {
                    <initiateCCTPForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxFeeRate(inner) => {
                    <maxFeeRateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onCctpExecute(inner) => {
                    <onCctpExecuteCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::ownershipHandoverExpiresAt(inner) => {
                    <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quoteFill(inner) => {
                    <quoteFillCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestOwnershipHandover(inner) => {
                    <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::selfPermit(inner) => {
                    <selfPermitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMaxFeeRate(inner) => {
                    <setMaxFeeRateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPaused(inner) => {
                    <setPausedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::settle(inner) => {
                    <settleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`CctpAdapter`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum CctpAdapterErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AlreadySettled(AlreadySettled),
        #[allow(missing_docs)]
        DomainMismatch(DomainMismatch),
        #[allow(missing_docs)]
        InsufficientCallbackGas(InsufficientCallbackGas),
        #[allow(missing_docs)]
        InvalidAddress(InvalidAddress),
        #[allow(missing_docs)]
        InvalidBaseFee(InvalidBaseFee),
        #[allow(missing_docs)]
        InvalidCallbackGasLimit(InvalidCallbackGasLimit),
        #[allow(missing_docs)]
        InvalidMaxFeeRate(InvalidMaxFeeRate),
        #[allow(missing_docs)]
        InvalidOutputAmount(InvalidOutputAmount),
        #[allow(missing_docs)]
        InvalidWindow(InvalidWindow),
        #[allow(missing_docs)]
        MaxFeeTooHigh(MaxFeeTooHigh),
        #[allow(missing_docs)]
        MessageTooShort(MessageTooShort),
        #[allow(missing_docs)]
        MintFeeTooHigh(MintFeeTooHigh),
        #[allow(missing_docs)]
        MintRecipientMismatch(MintRecipientMismatch),
        #[allow(missing_docs)]
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        #[allow(missing_docs)]
        NoHandoverRequest(NoHandoverRequest),
        #[allow(missing_docs)]
        NotSourceChain(NotSourceChain),
        #[allow(missing_docs)]
        NothingToClaim(NothingToClaim),
        #[allow(missing_docs)]
        OnlySelf(OnlySelf),
        #[allow(missing_docs)]
        OrderAlreadyActive(OrderAlreadyActive),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        ReceiveMessageFailed(ReceiveMessageFailed),
        #[allow(missing_docs)]
        RedirectFunds(RedirectFunds),
        #[allow(missing_docs)]
        Reentrancy(Reentrancy),
        #[allow(missing_docs)]
        Unauthorized(Unauthorized),
        #[allow(missing_docs)]
        UnsupportedChain(UnsupportedChain),
        #[allow(missing_docs)]
        UntrustedExecutor(UntrustedExecutor),
        #[allow(missing_docs)]
        UntrustedSender(UntrustedSender),
        #[allow(missing_docs)]
        UntrustedSourceDomain(UntrustedSourceDomain),
        #[allow(missing_docs)]
        WrongBridgeType(WrongBridgeType),
        #[allow(missing_docs)]
        WrongDestinationChain(WrongDestinationChain),
        #[allow(missing_docs)]
        WrongOutputToken(WrongOutputToken),
        #[allow(missing_docs)]
        ZeroCctpExecutor(ZeroCctpExecutor),
        #[allow(missing_docs)]
        ZeroRecipient(ZeroRecipient),
    }
    impl CctpAdapterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 193u8, 73u8, 240u8],
            [20u8, 212u8, 164u8, 232u8],
            [27u8, 47u8, 81u8, 103u8],
            [28u8, 222u8, 49u8, 17u8],
            [37u8, 173u8, 133u8, 148u8],
            [40u8, 2u8, 221u8, 158u8],
            [43u8, 249u8, 80u8, 101u8],
            [46u8, 119u8, 92u8, 124u8],
            [52u8, 62u8, 33u8, 30u8],
            [80u8, 206u8, 62u8, 217u8],
            [81u8, 77u8, 132u8, 10u8],
            [88u8, 135u8, 0u8, 199u8],
            [105u8, 80u8, 75u8, 208u8],
            [106u8, 150u8, 101u8, 158u8],
            [111u8, 94u8, 136u8, 24u8],
            [116u8, 72u8, 251u8, 174u8],
            [130u8, 180u8, 41u8, 0u8],
            [141u8, 0u8, 185u8, 27u8],
            [141u8, 174u8, 45u8, 43u8],
            [141u8, 208u8, 157u8, 145u8],
            [150u8, 155u8, 247u8, 40u8],
            [158u8, 135u8, 250u8, 200u8],
            [162u8, 171u8, 241u8, 182u8],
            [171u8, 20u8, 60u8, 6u8],
            [173u8, 107u8, 182u8, 209u8],
            [177u8, 150u8, 164u8, 74u8],
            [178u8, 195u8, 182u8, 253u8],
            [184u8, 37u8, 221u8, 118u8],
            [194u8, 31u8, 162u8, 229u8],
            [199u8, 40u8, 110u8, 161u8],
            [201u8, 224u8, 48u8, 232u8],
            [210u8, 123u8, 68u8, 67u8],
            [237u8, 43u8, 193u8, 234u8],
            [247u8, 195u8, 179u8, 102u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(OnlySelf),
            ::core::stringify!(NotSourceChain),
            ::core::stringify!(MintFeeTooHigh),
            ::core::stringify!(InvalidCallbackGasLimit),
            ::core::stringify!(InvalidWindow),
            ::core::stringify!(InvalidAddress),
            ::core::stringify!(WrongOutputToken),
            ::core::stringify!(OrderAlreadyActive),
            ::core::stringify!(UntrustedExecutor),
            ::core::stringify!(ReceiveMessageFailed),
            ::core::stringify!(InsufficientCallbackGas),
            ::core::stringify!(ZeroCctpExecutor),
            ::core::stringify!(UntrustedSourceDomain),
            ::core::stringify!(NoHandoverRequest),
            ::core::stringify!(NewOwnerIsZeroAddress),
            ::core::stringify!(Unauthorized),
            ::core::stringify!(InvalidBaseFee),
            ::core::stringify!(WrongDestinationChain),
            ::core::stringify!(InvalidOutputAmount),
            ::core::stringify!(NothingToClaim),
            ::core::stringify!(Paused),
            ::core::stringify!(MessageTooShort),
            ::core::stringify!(Reentrancy),
            ::core::stringify!(InvalidMaxFeeRate),
            ::core::stringify!(AlreadySettled),
            ::core::stringify!(WrongBridgeType),
            ::core::stringify!(UnsupportedChain),
            ::core::stringify!(UntrustedSender),
            ::core::stringify!(MintRecipientMismatch),
            ::core::stringify!(DomainMismatch),
            ::core::stringify!(ZeroRecipient),
            ::core::stringify!(MaxFeeTooHigh),
            ::core::stringify!(RedirectFunds),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <OnlySelf as alloy_sol_types::SolError>::SIGNATURE,
            <NotSourceChain as alloy_sol_types::SolError>::SIGNATURE,
            <MintFeeTooHigh as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidCallbackGasLimit as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidWindow as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidAddress as alloy_sol_types::SolError>::SIGNATURE,
            <WrongOutputToken as alloy_sol_types::SolError>::SIGNATURE,
            <OrderAlreadyActive as alloy_sol_types::SolError>::SIGNATURE,
            <UntrustedExecutor as alloy_sol_types::SolError>::SIGNATURE,
            <ReceiveMessageFailed as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientCallbackGas as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroCctpExecutor as alloy_sol_types::SolError>::SIGNATURE,
            <UntrustedSourceDomain as alloy_sol_types::SolError>::SIGNATURE,
            <NoHandoverRequest as alloy_sol_types::SolError>::SIGNATURE,
            <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::SIGNATURE,
            <Unauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidBaseFee as alloy_sol_types::SolError>::SIGNATURE,
            <WrongDestinationChain as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidOutputAmount as alloy_sol_types::SolError>::SIGNATURE,
            <NothingToClaim as alloy_sol_types::SolError>::SIGNATURE,
            <Paused as alloy_sol_types::SolError>::SIGNATURE,
            <MessageTooShort as alloy_sol_types::SolError>::SIGNATURE,
            <Reentrancy as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidMaxFeeRate as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadySettled as alloy_sol_types::SolError>::SIGNATURE,
            <WrongBridgeType as alloy_sol_types::SolError>::SIGNATURE,
            <UnsupportedChain as alloy_sol_types::SolError>::SIGNATURE,
            <UntrustedSender as alloy_sol_types::SolError>::SIGNATURE,
            <MintRecipientMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <DomainMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRecipient as alloy_sol_types::SolError>::SIGNATURE,
            <MaxFeeTooHigh as alloy_sol_types::SolError>::SIGNATURE,
            <RedirectFunds as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CctpAdapterErrors {
        const NAME: &'static str = "CctpAdapterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadySettled(_) => {
                    <AlreadySettled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DomainMismatch(_) => {
                    <DomainMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientCallbackGas(_) => {
                    <InsufficientCallbackGas as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAddress(_) => {
                    <InvalidAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBaseFee(_) => {
                    <InvalidBaseFee as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCallbackGasLimit(_) => {
                    <InvalidCallbackGasLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidMaxFeeRate(_) => {
                    <InvalidMaxFeeRate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOutputAmount(_) => {
                    <InvalidOutputAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWindow(_) => {
                    <InvalidWindow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxFeeTooHigh(_) => {
                    <MaxFeeTooHigh as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MessageTooShort(_) => {
                    <MessageTooShort as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MintFeeTooHigh(_) => {
                    <MintFeeTooHigh as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MintRecipientMismatch(_) => {
                    <MintRecipientMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NewOwnerIsZeroAddress(_) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoHandoverRequest(_) => {
                    <NoHandoverRequest as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSourceChain(_) => {
                    <NotSourceChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NothingToClaim(_) => {
                    <NothingToClaim as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlySelf(_) => <OnlySelf as alloy_sol_types::SolError>::SELECTOR,
                Self::OrderAlreadyActive(_) => {
                    <OrderAlreadyActive as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Paused(_) => <Paused as alloy_sol_types::SolError>::SELECTOR,
                Self::ReceiveMessageFailed(_) => {
                    <ReceiveMessageFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RedirectFunds(_) => {
                    <RedirectFunds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reentrancy(_) => {
                    <Reentrancy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedChain(_) => {
                    <UnsupportedChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UntrustedExecutor(_) => {
                    <UntrustedExecutor as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UntrustedSender(_) => {
                    <UntrustedSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UntrustedSourceDomain(_) => {
                    <UntrustedSourceDomain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongBridgeType(_) => {
                    <WrongBridgeType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongDestinationChain(_) => {
                    <WrongDestinationChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongOutputToken(_) => {
                    <WrongOutputToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroCctpExecutor(_) => {
                    <ZeroCctpExecutor as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroRecipient(_) => {
                    <ZeroRecipient as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<CctpAdapterErrors>] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OnlySelf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <OnlySelf as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CctpAdapterErrors::OnlySelf)
                    }
                    OnlySelf
                },
                {
                    fn NotSourceChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NotSourceChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::NotSourceChain)
                    }
                    NotSourceChain
                },
                {
                    fn MintFeeTooHigh(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MintFeeTooHigh as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::MintFeeTooHigh)
                    }
                    MintFeeTooHigh
                },
                {
                    fn InvalidCallbackGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidCallbackGasLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidCallbackGasLimit)
                    }
                    InvalidCallbackGasLimit
                },
                {
                    fn InvalidWindow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidWindow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidWindow)
                    }
                    InvalidWindow
                },
                {
                    fn InvalidAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidAddress)
                    }
                    InvalidAddress
                },
                {
                    fn WrongOutputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <WrongOutputToken as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::WrongOutputToken)
                    }
                    WrongOutputToken
                },
                {
                    fn OrderAlreadyActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <OrderAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::OrderAlreadyActive)
                    }
                    OrderAlreadyActive
                },
                {
                    fn UntrustedExecutor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UntrustedExecutor as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::UntrustedExecutor)
                    }
                    UntrustedExecutor
                },
                {
                    fn ReceiveMessageFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <ReceiveMessageFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::ReceiveMessageFailed)
                    }
                    ReceiveMessageFailed
                },
                {
                    fn InsufficientCallbackGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InsufficientCallbackGas as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InsufficientCallbackGas)
                    }
                    InsufficientCallbackGas
                },
                {
                    fn ZeroCctpExecutor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <ZeroCctpExecutor as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::ZeroCctpExecutor)
                    }
                    ZeroCctpExecutor
                },
                {
                    fn UntrustedSourceDomain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UntrustedSourceDomain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::UntrustedSourceDomain)
                    }
                    UntrustedSourceDomain
                },
                {
                    fn NoHandoverRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NoHandoverRequest as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::NoHandoverRequest)
                    }
                    NoHandoverRequest
                },
                {
                    fn NewOwnerIsZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::NewOwnerIsZeroAddress)
                    }
                    NewOwnerIsZeroAddress
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CctpAdapterErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn InvalidBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidBaseFee as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidBaseFee)
                    }
                    InvalidBaseFee
                },
                {
                    fn WrongDestinationChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <WrongDestinationChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::WrongDestinationChain)
                    }
                    WrongDestinationChain
                },
                {
                    fn InvalidOutputAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidOutputAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidOutputAmount)
                    }
                    InvalidOutputAmount
                },
                {
                    fn NothingToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NothingToClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::NothingToClaim)
                    }
                    NothingToClaim
                },
                {
                    fn Paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <Paused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CctpAdapterErrors::Paused)
                    }
                    Paused
                },
                {
                    fn MessageTooShort(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MessageTooShort as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::MessageTooShort)
                    }
                    MessageTooShort
                },
                {
                    fn Reentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <Reentrancy as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CctpAdapterErrors::Reentrancy)
                    }
                    Reentrancy
                },
                {
                    fn InvalidMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidMaxFeeRate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidMaxFeeRate)
                    }
                    InvalidMaxFeeRate
                },
                {
                    fn AlreadySettled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <AlreadySettled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::AlreadySettled)
                    }
                    AlreadySettled
                },
                {
                    fn WrongBridgeType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <WrongBridgeType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::WrongBridgeType)
                    }
                    WrongBridgeType
                },
                {
                    fn UnsupportedChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UnsupportedChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::UnsupportedChain)
                    }
                    UnsupportedChain
                },
                {
                    fn UntrustedSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UntrustedSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::UntrustedSender)
                    }
                    UntrustedSender
                },
                {
                    fn MintRecipientMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MintRecipientMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::MintRecipientMismatch)
                    }
                    MintRecipientMismatch
                },
                {
                    fn DomainMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <DomainMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::DomainMismatch)
                    }
                    DomainMismatch
                },
                {
                    fn ZeroRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <ZeroRecipient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::ZeroRecipient)
                    }
                    ZeroRecipient
                },
                {
                    fn MaxFeeTooHigh(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MaxFeeTooHigh as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::MaxFeeTooHigh)
                    }
                    MaxFeeTooHigh
                },
                {
                    fn RedirectFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <RedirectFunds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CctpAdapterErrors::RedirectFunds)
                    }
                    RedirectFunds
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<CctpAdapterErrors>] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OnlySelf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <OnlySelf as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::OnlySelf)
                    }
                    OnlySelf
                },
                {
                    fn NotSourceChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NotSourceChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::NotSourceChain)
                    }
                    NotSourceChain
                },
                {
                    fn MintFeeTooHigh(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MintFeeTooHigh as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::MintFeeTooHigh)
                    }
                    MintFeeTooHigh
                },
                {
                    fn InvalidCallbackGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidCallbackGasLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidCallbackGasLimit)
                    }
                    InvalidCallbackGasLimit
                },
                {
                    fn InvalidWindow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidWindow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidWindow)
                    }
                    InvalidWindow
                },
                {
                    fn InvalidAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidAddress)
                    }
                    InvalidAddress
                },
                {
                    fn WrongOutputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <WrongOutputToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::WrongOutputToken)
                    }
                    WrongOutputToken
                },
                {
                    fn OrderAlreadyActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <OrderAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::OrderAlreadyActive)
                    }
                    OrderAlreadyActive
                },
                {
                    fn UntrustedExecutor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UntrustedExecutor as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::UntrustedExecutor)
                    }
                    UntrustedExecutor
                },
                {
                    fn ReceiveMessageFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <ReceiveMessageFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::ReceiveMessageFailed)
                    }
                    ReceiveMessageFailed
                },
                {
                    fn InsufficientCallbackGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InsufficientCallbackGas as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InsufficientCallbackGas)
                    }
                    InsufficientCallbackGas
                },
                {
                    fn ZeroCctpExecutor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <ZeroCctpExecutor as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::ZeroCctpExecutor)
                    }
                    ZeroCctpExecutor
                },
                {
                    fn UntrustedSourceDomain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UntrustedSourceDomain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::UntrustedSourceDomain)
                    }
                    UntrustedSourceDomain
                },
                {
                    fn NoHandoverRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NoHandoverRequest as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::NoHandoverRequest)
                    }
                    NoHandoverRequest
                },
                {
                    fn NewOwnerIsZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::NewOwnerIsZeroAddress)
                    }
                    NewOwnerIsZeroAddress
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn InvalidBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidBaseFee as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidBaseFee)
                    }
                    InvalidBaseFee
                },
                {
                    fn WrongDestinationChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <WrongDestinationChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::WrongDestinationChain)
                    }
                    WrongDestinationChain
                },
                {
                    fn InvalidOutputAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidOutputAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidOutputAmount)
                    }
                    InvalidOutputAmount
                },
                {
                    fn NothingToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <NothingToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::NothingToClaim)
                    }
                    NothingToClaim
                },
                {
                    fn Paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <Paused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::Paused)
                    }
                    Paused
                },
                {
                    fn MessageTooShort(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MessageTooShort as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::MessageTooShort)
                    }
                    MessageTooShort
                },
                {
                    fn Reentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <Reentrancy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::Reentrancy)
                    }
                    Reentrancy
                },
                {
                    fn InvalidMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <InvalidMaxFeeRate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::InvalidMaxFeeRate)
                    }
                    InvalidMaxFeeRate
                },
                {
                    fn AlreadySettled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <AlreadySettled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::AlreadySettled)
                    }
                    AlreadySettled
                },
                {
                    fn WrongBridgeType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <WrongBridgeType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::WrongBridgeType)
                    }
                    WrongBridgeType
                },
                {
                    fn UnsupportedChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UnsupportedChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::UnsupportedChain)
                    }
                    UnsupportedChain
                },
                {
                    fn UntrustedSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <UntrustedSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::UntrustedSender)
                    }
                    UntrustedSender
                },
                {
                    fn MintRecipientMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MintRecipientMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::MintRecipientMismatch)
                    }
                    MintRecipientMismatch
                },
                {
                    fn DomainMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <DomainMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::DomainMismatch)
                    }
                    DomainMismatch
                },
                {
                    fn ZeroRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <ZeroRecipient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::ZeroRecipient)
                    }
                    ZeroRecipient
                },
                {
                    fn MaxFeeTooHigh(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <MaxFeeTooHigh as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::MaxFeeTooHigh)
                    }
                    MaxFeeTooHigh
                },
                {
                    fn RedirectFunds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterErrors> {
                        <RedirectFunds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CctpAdapterErrors::RedirectFunds)
                    }
                    RedirectFunds
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AlreadySettled(inner) => {
                    <AlreadySettled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DomainMismatch(inner) => {
                    <DomainMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientCallbackGas(inner) => {
                    <InsufficientCallbackGas as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAddress(inner) => {
                    <InvalidAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBaseFee(inner) => {
                    <InvalidBaseFee as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCallbackGasLimit(inner) => {
                    <InvalidCallbackGasLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidMaxFeeRate(inner) => {
                    <InvalidMaxFeeRate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOutputAmount(inner) => {
                    <InvalidOutputAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidWindow(inner) => {
                    <InvalidWindow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MaxFeeTooHigh(inner) => {
                    <MaxFeeTooHigh as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MessageTooShort(inner) => {
                    <MessageTooShort as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MintFeeTooHigh(inner) => {
                    <MintFeeTooHigh as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MintRecipientMismatch(inner) => {
                    <MintRecipientMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NewOwnerIsZeroAddress(inner) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoHandoverRequest(inner) => {
                    <NoHandoverRequest as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSourceChain(inner) => {
                    <NotSourceChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NothingToClaim(inner) => {
                    <NothingToClaim as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlySelf(inner) => {
                    <OnlySelf as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OrderAlreadyActive(inner) => {
                    <OrderAlreadyActive as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Paused(inner) => {
                    <Paused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ReceiveMessageFailed(inner) => {
                    <ReceiveMessageFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RedirectFunds(inner) => {
                    <RedirectFunds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::Reentrancy(inner) => {
                    <Reentrancy as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UnsupportedChain(inner) => {
                    <UnsupportedChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UntrustedExecutor(inner) => {
                    <UntrustedExecutor as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UntrustedSender(inner) => {
                    <UntrustedSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UntrustedSourceDomain(inner) => {
                    <UntrustedSourceDomain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongBridgeType(inner) => {
                    <WrongBridgeType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongDestinationChain(inner) => {
                    <WrongDestinationChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongOutputToken(inner) => {
                    <WrongOutputToken as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroCctpExecutor(inner) => {
                    <ZeroCctpExecutor as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroRecipient(inner) => {
                    <ZeroRecipient as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AlreadySettled(inner) => {
                    <AlreadySettled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DomainMismatch(inner) => {
                    <DomainMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientCallbackGas(inner) => {
                    <InsufficientCallbackGas as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAddress(inner) => {
                    <InvalidAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBaseFee(inner) => {
                    <InvalidBaseFee as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCallbackGasLimit(inner) => {
                    <InvalidCallbackGasLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidMaxFeeRate(inner) => {
                    <InvalidMaxFeeRate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOutputAmount(inner) => {
                    <InvalidOutputAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidWindow(inner) => {
                    <InvalidWindow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxFeeTooHigh(inner) => {
                    <MaxFeeTooHigh as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MessageTooShort(inner) => {
                    <MessageTooShort as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MintFeeTooHigh(inner) => {
                    <MintFeeTooHigh as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MintRecipientMismatch(inner) => {
                    <MintRecipientMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NewOwnerIsZeroAddress(inner) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoHandoverRequest(inner) => {
                    <NoHandoverRequest as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSourceChain(inner) => {
                    <NotSourceChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NothingToClaim(inner) => {
                    <NothingToClaim as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlySelf(inner) => {
                    <OnlySelf as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OrderAlreadyActive(inner) => {
                    <OrderAlreadyActive as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Paused(inner) => {
                    <Paused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ReceiveMessageFailed(inner) => {
                    <ReceiveMessageFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RedirectFunds(inner) => {
                    <RedirectFunds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Reentrancy(inner) => {
                    <Reentrancy as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnsupportedChain(inner) => {
                    <UnsupportedChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UntrustedExecutor(inner) => {
                    <UntrustedExecutor as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UntrustedSender(inner) => {
                    <UntrustedSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UntrustedSourceDomain(inner) => {
                    <UntrustedSourceDomain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongBridgeType(inner) => {
                    <WrongBridgeType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongDestinationChain(inner) => {
                    <WrongDestinationChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongOutputToken(inner) => {
                    <WrongOutputToken as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroCctpExecutor(inner) => {
                    <ZeroCctpExecutor as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroRecipient(inner) => {
                    <ZeroRecipient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`CctpAdapter`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum CctpAdapterEvents {
        #[allow(missing_docs)]
        Claimed(Claimed),
        #[allow(missing_docs)]
        DestinationCallback(DestinationCallback),
        #[allow(missing_docs)]
        OrderCreated(OrderCreated),
        #[allow(missing_docs)]
        OrderFilled(OrderFilled),
        #[allow(missing_docs)]
        OrderSettled(OrderSettled),
        #[allow(missing_docs)]
        OwnershipHandoverCanceled(OwnershipHandoverCanceled),
        #[allow(missing_docs)]
        OwnershipHandoverRequested(OwnershipHandoverRequested),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        PayoutDeferred(PayoutDeferred),
    }
    impl CctpAdapterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                38u8, 235u8, 188u8, 162u8, 147u8, 173u8, 98u8, 165u8, 108u8, 214u8,
                171u8, 163u8, 44u8, 189u8, 16u8, 193u8, 28u8, 60u8, 237u8, 108u8, 215u8,
                56u8, 220u8, 203u8, 168u8, 17u8, 216u8, 237u8, 215u8, 153u8, 26u8, 154u8,
            ],
            [
                41u8, 85u8, 160u8, 199u8, 161u8, 251u8, 175u8, 206u8, 169u8, 78u8, 10u8,
                214u8, 210u8, 196u8, 132u8, 74u8, 31u8, 23u8, 159u8, 41u8, 99u8, 68u8,
                16u8, 183u8, 93u8, 227u8, 165u8, 162u8, 151u8, 127u8, 234u8, 26u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                148u8, 40u8, 117u8, 124u8, 23u8, 55u8, 119u8, 138u8, 81u8, 147u8, 25u8,
                231u8, 156u8, 33u8, 6u8, 112u8, 196u8, 147u8, 147u8, 191u8, 190u8, 251u8,
                132u8, 16u8, 160u8, 173u8, 185u8, 103u8, 13u8, 203u8, 101u8, 159u8,
            ],
            [
                158u8, 61u8, 157u8, 59u8, 191u8, 221u8, 70u8, 223u8, 20u8, 85u8, 124u8,
                66u8, 124u8, 120u8, 160u8, 202u8, 126u8, 41u8, 80u8, 170u8, 113u8, 84u8,
                171u8, 247u8, 19u8, 245u8, 183u8, 9u8, 158u8, 186u8, 18u8, 169u8,
            ],
            [
                182u8, 122u8, 11u8, 139u8, 20u8, 68u8, 105u8, 228u8, 4u8, 194u8, 44u8,
                119u8, 196u8, 232u8, 107u8, 151u8, 69u8, 169u8, 189u8, 146u8, 138u8,
                78u8, 134u8, 167u8, 153u8, 51u8, 231u8, 177u8, 105u8, 102u8, 183u8, 141u8,
            ],
            [
                219u8, 243u8, 106u8, 16u8, 125u8, 161u8, 158u8, 73u8, 82u8, 122u8, 113u8,
                118u8, 161u8, 186u8, 191u8, 150u8, 59u8, 75u8, 15u8, 248u8, 205u8, 227u8,
                94u8, 227u8, 93u8, 108u8, 216u8, 241u8, 249u8, 172u8, 126u8, 29u8,
            ],
            [
                247u8, 164u8, 0u8, 119u8, 255u8, 122u8, 4u8, 199u8, 230u8, 31u8, 111u8,
                38u8, 251u8, 19u8, 119u8, 66u8, 89u8, 221u8, 241u8, 182u8, 188u8, 233u8,
                236u8, 242u8, 106u8, 130u8, 118u8, 205u8, 211u8, 153u8, 38u8, 131u8,
            ],
            [
                250u8, 123u8, 142u8, 171u8, 125u8, 166u8, 127u8, 65u8, 44u8, 201u8, 87u8,
                94u8, 212u8, 52u8, 100u8, 70u8, 143u8, 155u8, 251u8, 174u8, 137u8, 209u8,
                103u8, 89u8, 23u8, 52u8, 108u8, 166u8, 216u8, 254u8, 60u8, 146u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(OrderSettled),
            ::core::stringify!(OrderCreated),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(PayoutDeferred),
            ::core::stringify!(DestinationCallback),
            ::core::stringify!(OrderFilled),
            ::core::stringify!(OwnershipHandoverRequested),
            ::core::stringify!(Claimed),
            ::core::stringify!(OwnershipHandoverCanceled),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <OrderSettled as alloy_sol_types::SolEvent>::SIGNATURE,
            <OrderCreated as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <PayoutDeferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <DestinationCallback as alloy_sol_types::SolEvent>::SIGNATURE,
            <OrderFilled as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipHandoverRequested as alloy_sol_types::SolEvent>::SIGNATURE,
            <Claimed as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipHandoverCanceled as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for CctpAdapterEvents {
        const NAME: &'static str = "CctpAdapterEvents";
        const COUNT: usize = 9usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Claimed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Claimed as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Claimed)
                }
                Some(
                    <DestinationCallback as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DestinationCallback as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DestinationCallback)
                }
                Some(<OrderCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderCreated)
                }
                Some(<OrderFilled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderFilled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderFilled)
                }
                Some(<OrderSettled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OrderSettled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OrderSettled)
                }
                Some(
                    <OwnershipHandoverCanceled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipHandoverCanceled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipHandoverCanceled)
                }
                Some(
                    <OwnershipHandoverRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipHandoverRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipHandoverRequested)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<PayoutDeferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PayoutDeferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PayoutDeferred)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for CctpAdapterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Claimed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DestinationCallback(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderFilled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OrderSettled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipHandoverCanceled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipHandoverRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PayoutDeferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Claimed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DestinationCallback(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderFilled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OrderSettled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipHandoverCanceled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipHandoverRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PayoutDeferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`CctpAdapter`](self) contract instance.

See the [wrapper's documentation](`CctpAdapterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> CctpAdapterInstance<P, N> {
        CctpAdapterInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        config_: alloy::sol_types::private::Address,
        owner_: alloy::sol_types::private::Address,
        maxFeeRate_: alloy::sol_types::private::primitives::aliases::U256,
        cctpExecutor_: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<CctpAdapterInstance<P, N>>,
    > {
        CctpAdapterInstance::<
            P,
            N,
        >::deploy(__provider, config_, owner_, maxFeeRate_, cctpExecutor_)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        config_: alloy::sol_types::private::Address,
        owner_: alloy::sol_types::private::Address,
        maxFeeRate_: alloy::sol_types::private::primitives::aliases::U256,
        cctpExecutor_: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        CctpAdapterInstance::<
            P,
            N,
        >::deploy_builder(__provider, config_, owner_, maxFeeRate_, cctpExecutor_)
    }
    /**A [`CctpAdapter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`CctpAdapter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CctpAdapterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for CctpAdapterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CctpAdapterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CctpAdapterInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`CctpAdapter`](self) contract instance.

See the [wrapper's documentation](`CctpAdapterInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            config_: alloy::sol_types::private::Address,
            owner_: alloy::sol_types::private::Address,
            maxFeeRate_: alloy::sol_types::private::primitives::aliases::U256,
            cctpExecutor_: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<CctpAdapterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                config_,
                owner_,
                maxFeeRate_,
                cctpExecutor_,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            config_: alloy::sol_types::private::Address,
            owner_: alloy::sol_types::private::Address,
            maxFeeRate_: alloy::sol_types::private::primitives::aliases::U256,
            cctpExecutor_: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            config_,
                            owner_,
                            maxFeeRate_,
                            cctpExecutor_,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> CctpAdapterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CctpAdapterInstance<P, N> {
            CctpAdapterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CctpAdapterInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`FINALITY_FAST`] function.
        pub fn FINALITY_FAST(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, FINALITY_FASTCall, N> {
            self.call_builder(&FINALITY_FASTCall)
        }
        ///Creates a new call builder for the [`FINALITY_FINALIZED`] function.
        pub fn FINALITY_FINALIZED(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, FINALITY_FINALIZEDCall, N> {
            self.call_builder(&FINALITY_FINALIZEDCall)
        }
        ///Creates a new call builder for the [`MAX_CALLBACK_GAS_LIMIT`] function.
        pub fn MAX_CALLBACK_GAS_LIMIT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_CALLBACK_GAS_LIMITCall, N> {
            self.call_builder(&MAX_CALLBACK_GAS_LIMITCall)
        }
        ///Creates a new call builder for the [`PERMIT2`] function.
        pub fn PERMIT2(&self) -> alloy_contract::SolCallBuilder<&P, PERMIT2Call, N> {
            self.call_builder(&PERMIT2Call)
        }
        ///Creates a new call builder for the [`_executeDelivery`] function.
        pub fn _executeDelivery(
            &self,
            token: alloy::sol_types::private::Address,
            recipient: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            gasLimit: alloy::sol_types::private::primitives::aliases::U256,
            callbackData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, _executeDeliveryCall, N> {
            self.call_builder(
                &_executeDeliveryCall {
                    token,
                    recipient,
                    amount,
                    gasLimit,
                    callbackData,
                },
            )
        }
        ///Creates a new call builder for the [`cancelOwnershipHandover`] function.
        pub fn cancelOwnershipHandover(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, cancelOwnershipHandoverCall, N> {
            self.call_builder(&cancelOwnershipHandoverCall)
        }
        ///Creates a new call builder for the [`cctpExecutor`] function.
        pub fn cctpExecutor(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, cctpExecutorCall, N> {
            self.call_builder(&cctpExecutorCall)
        }
        ///Creates a new call builder for the [`claim`] function.
        pub fn claim(
            &self,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, claimCall, N> {
            self.call_builder(&claimCall { token })
        }
        ///Creates a new call builder for the [`claimable`] function.
        pub fn claimable(
            &self,
            account: alloy::sol_types::private::Address,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, claimableCall, N> {
            self.call_builder(&claimableCall { account, token })
        }
        ///Creates a new call builder for the [`completeOwnershipHandover`] function.
        pub fn completeOwnershipHandover(
            &self,
            pendingOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, completeOwnershipHandoverCall, N> {
            self.call_builder(
                &completeOwnershipHandoverCall {
                    pendingOwner,
                },
            )
        }
        ///Creates a new call builder for the [`config`] function.
        pub fn config(&self) -> alloy_contract::SolCallBuilder<&P, configCall, N> {
            self.call_builder(&configCall)
        }
        ///Creates a new call builder for the [`fill`] function.
        pub fn fill(
            &self,
            order: <Order as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, fillCall, N> {
            self.call_builder(&fillCall { order })
        }
        ///Creates a new call builder for the [`fillFor`] function.
        pub fn fillFor(
            &self,
            order: <Order as alloy::sol_types::SolType>::RustType,
            filler: alloy::sol_types::private::Address,
            permit: <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, fillForCall, N> {
            self.call_builder(
                &fillForCall {
                    order,
                    filler,
                    permit,
                },
            )
        }
        ///Creates a new call builder for the [`getOrder`] function.
        pub fn getOrder(
            &self,
            orderId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getOrderCall, N> {
            self.call_builder(&getOrderCall { orderId })
        }
        ///Creates a new call builder for the [`initiateCCTP`] function.
        pub fn initiateCCTP(
            &self,
            dstChainId: u32,
            recipient: alloy::sol_types::private::FixedBytes<32>,
            inputAmount: alloy::sol_types::private::primitives::aliases::U256,
            maxFee: alloy::sol_types::private::primitives::aliases::U256,
            mintFee: alloy::sol_types::private::primitives::aliases::U256,
            minFinalityThreshold: u32,
            deliveryWindow: u64,
            discountRate: alloy::sol_types::private::primitives::aliases::U256,
            baseFee: alloy::sol_types::private::primitives::aliases::U256,
            exec: <Execution as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initiateCCTPCall, N> {
            self.call_builder(
                &initiateCCTPCall {
                    dstChainId,
                    recipient,
                    inputAmount,
                    maxFee,
                    mintFee,
                    minFinalityThreshold,
                    deliveryWindow,
                    discountRate,
                    baseFee,
                    exec,
                },
            )
        }
        ///Creates a new call builder for the [`initiateCCTPFor`] function.
        pub fn initiateCCTPFor(
            &self,
            dstChainId: u32,
            recipient: alloy::sol_types::private::FixedBytes<32>,
            inputAmount: alloy::sol_types::private::primitives::aliases::U256,
            maxFee: alloy::sol_types::private::primitives::aliases::U256,
            mintFee: alloy::sol_types::private::primitives::aliases::U256,
            minFinalityThreshold: u32,
            deliveryWindow: u64,
            discountRate: alloy::sol_types::private::primitives::aliases::U256,
            baseFee: alloy::sol_types::private::primitives::aliases::U256,
            exec: <Execution as alloy::sol_types::SolType>::RustType,
            from: alloy::sol_types::private::Address,
            permit: <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initiateCCTPForCall, N> {
            self.call_builder(
                &initiateCCTPForCall {
                    dstChainId,
                    recipient,
                    inputAmount,
                    maxFee,
                    mintFee,
                    minFinalityThreshold,
                    deliveryWindow,
                    discountRate,
                    baseFee,
                    exec,
                    from,
                    permit,
                },
            )
        }
        ///Creates a new call builder for the [`maxFeeRate`] function.
        pub fn maxFeeRate(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maxFeeRateCall, N> {
            self.call_builder(&maxFeeRateCall)
        }
        ///Creates a new call builder for the [`multicall`] function.
        pub fn multicall(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<&P, multicallCall, N> {
            self.call_builder(&multicallCall { data })
        }
        ///Creates a new call builder for the [`onCctpExecute`] function.
        pub fn onCctpExecute(
            &self,
            sourceDomain: u32,
            sender: alloy::sol_types::private::FixedBytes<32>,
            usdc: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            payload: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, onCctpExecuteCall, N> {
            self.call_builder(
                &onCctpExecuteCall {
                    sourceDomain,
                    sender,
                    usdc,
                    amount,
                    payload,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`ownershipHandoverExpiresAt`] function.
        pub fn ownershipHandoverExpiresAt(
            &self,
            pendingOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, ownershipHandoverExpiresAtCall, N> {
            self.call_builder(
                &ownershipHandoverExpiresAtCall {
                    pendingOwner,
                },
            )
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`quoteFill`] function.
        pub fn quoteFill(
            &self,
            order: <Order as alloy::sol_types::SolType>::RustType,
            fillTime: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, quoteFillCall, N> {
            self.call_builder(&quoteFillCall { order, fillTime })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`requestOwnershipHandover`] function.
        pub fn requestOwnershipHandover(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, requestOwnershipHandoverCall, N> {
            self.call_builder(&requestOwnershipHandoverCall)
        }
        ///Creates a new call builder for the [`selfPermit`] function.
        pub fn selfPermit(
            &self,
            token: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, selfPermitCall, N> {
            self.call_builder(
                &selfPermitCall {
                    token,
                    value,
                    deadline,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`setMaxFeeRate`] function.
        pub fn setMaxFeeRate(
            &self,
            newMaxFeeRate: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setMaxFeeRateCall, N> {
            self.call_builder(&setMaxFeeRateCall { newMaxFeeRate })
        }
        ///Creates a new call builder for the [`setPaused`] function.
        pub fn setPaused(
            &self,
            newPaused: bool,
        ) -> alloy_contract::SolCallBuilder<&P, setPausedCall, N> {
            self.call_builder(&setPausedCall { newPaused })
        }
        ///Creates a new call builder for the [`settle`] function.
        pub fn settle(
            &self,
            message: alloy::sol_types::private::Bytes,
            attestation: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, settleCall, N> {
            self.call_builder(&settleCall { message, attestation })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CctpAdapterInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Claimed`] event.
        pub fn Claimed_filter(&self) -> alloy_contract::Event<&P, Claimed, N> {
            self.event_filter::<Claimed>()
        }
        ///Creates a new event filter for the [`DestinationCallback`] event.
        pub fn DestinationCallback_filter(
            &self,
        ) -> alloy_contract::Event<&P, DestinationCallback, N> {
            self.event_filter::<DestinationCallback>()
        }
        ///Creates a new event filter for the [`OrderCreated`] event.
        pub fn OrderCreated_filter(&self) -> alloy_contract::Event<&P, OrderCreated, N> {
            self.event_filter::<OrderCreated>()
        }
        ///Creates a new event filter for the [`OrderFilled`] event.
        pub fn OrderFilled_filter(&self) -> alloy_contract::Event<&P, OrderFilled, N> {
            self.event_filter::<OrderFilled>()
        }
        ///Creates a new event filter for the [`OrderSettled`] event.
        pub fn OrderSettled_filter(&self) -> alloy_contract::Event<&P, OrderSettled, N> {
            self.event_filter::<OrderSettled>()
        }
        ///Creates a new event filter for the [`OwnershipHandoverCanceled`] event.
        pub fn OwnershipHandoverCanceled_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipHandoverCanceled, N> {
            self.event_filter::<OwnershipHandoverCanceled>()
        }
        ///Creates a new event filter for the [`OwnershipHandoverRequested`] event.
        pub fn OwnershipHandoverRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipHandoverRequested, N> {
            self.event_filter::<OwnershipHandoverRequested>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PayoutDeferred`] event.
        pub fn PayoutDeferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, PayoutDeferred, N> {
            self.event_filter::<PayoutDeferred>()
        }
    }
}
