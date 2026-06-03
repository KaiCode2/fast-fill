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
    error MintRecipientMismatch(bytes32 mintRecipient);
    error NewOwnerIsZeroAddress();
    error NoHandoverRequest();
    error NotSourceChain(uint32 srcChainId);
    error NothingToClaim();
    error OnlySelf();
    error OrderAlreadyActive(bytes32 orderId);
    error Paused();
    error ReceiveMessageFailed();
    error Reentrancy();
    error Unauthorized();
    error UnsupportedChain(uint32 chainId);
    error UntrustedSender(bytes32 messageSender);
    error UntrustedSourceDomain(uint32 sourceDomain);
    error WrongBridgeType(uint8 expected, uint8 got);
    error WrongDestinationChain(uint32 expected);
    error WrongOutputToken(bytes32 outputToken);
    error ZeroRecipient();

    event Claimed(address indexed account, address indexed token, uint256 amount);
    event DestinationCallback(bytes32 indexed orderId, address indexed fundsTo, CallbackResult result);
    event OrderCreated(bytes32 indexed orderId, uint8 bridgeType, address indexed sender, uint32 dstChainId, bytes32 outputToken, uint256 outputAmount, uint64 nonce);
    event OrderFilled(bytes32 indexed orderId, address indexed filler, uint256 payoutToRecipient, uint256 feeToFiller, uint40 fillTime);
    event OrderSettled(bytes32 indexed orderId, address indexed filler, uint256 arrivedAmount, uint256 surplusToRecipient);
    event OwnershipHandoverCanceled(address indexed pendingOwner);
    event OwnershipHandoverRequested(address indexed pendingOwner);
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
    event PayoutDeferred(bytes32 indexed orderId, address indexed to, address indexed token, uint256 amount);

    constructor(address config_, address owner_, uint256 maxFeeRate_);

    function FINALITY_FAST() external view returns (uint32);
    function FINALITY_FINALIZED() external view returns (uint32);
    function MAX_CALLBACK_GAS_LIMIT() external view returns (uint64);
    function PERMIT2() external view returns (address);
    function _executeDelivery(address token, address recipient, uint256 amount, bytes32 orderId, bytes memory hookData, uint256 gasLimit) external;
    function cancelOwnershipHandover() external payable;
    function claim(address token) external returns (uint256 amount);
    function claimable(address account, address token) external view returns (uint256);
    function completeOwnershipHandover(address pendingOwner) external payable;
    function config() external view returns (address);
    function fill(Order memory order) external returns (bytes32 orderId);
    function fillFor(Order memory order, address filler, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId);
    function getOrder(bytes32 orderId) external view returns (OrderRecord memory);
    function initiateCCTP(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec) external returns (bytes32 orderId, uint64 nonce);
    function initiateCCTPFor(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec, address from, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId, uint64 nonce);
    function maxFeeRate() external view returns (uint256);
    function multicall(bytes[] memory data) external payable returns (bytes[] memory);
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
        "name": "orderId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "gasLimit",
        "type": "uint256",
        "internalType": "uint256"
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
        "name": "orderId",
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
        "name": "orderId",
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
    ///0x60a03461010457601f6147a338819003918201601f19168301916001600160401b0383118484101761010857808492606094604052833981010312610104576100478161011c565b9060406100566020830161011c565b91015190670de0b6b3a764000082116100f1576001600160a01b0316638b78c6d8198190555f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a36002556001600160a01b031660805260405161467290816101318239608051818181610a460152818161131a01528181611ea20152818161295d0152818161309f01528181613b840152613e7a0152f35b5063ad6bb6d160e01b5f5260045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101045756fe60806040526004361015610011575f80fd5b5f3560e01c806316c38b3c146101d45780631e83409a146101cf57806325692962146101ca57806331eee44d146101c557806334236a39146101c057806339c33215146101bb57806354d1f13d146101b65780635778472a146101b15780635c975abb146101ac5780635fdc7c12146101a7578063617c537c146101a25780636afdd8501461019d578063715018a614610198578063776ff3c71461019357806377839a9e1461018e57806379502c551461018957806385c17830146101845780638cda96de1461017f5780638da5cb5b1461017a57806397c36bae14610175578063ac9650d814610170578063cc6eec701461016b578063d4570c1c14610166578063e3bb93e814610161578063f04e283e1461015c578063f2fde38b14610157578063f3995c67146101525763fee81cf41461014d575f80fd5b6119c9565b6118e0565b61186d565b6117fc565b61176a565b6116d1565b611697565b611559565b611462565b6113f2565b611379565b61133e565b6112d0565b611296565b61101b565b610f7e565b610f37565b610dda565b610968565b6108f7565b610848565b610769565b61072e565b610662565b610485565b61040f565b610297565b6101e7565b801515036101e357565b5f80fd5b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357600435610222816101d9565b61022a611f4f565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff000000000000000060035492151560401b169116176003555f80f35b73ffffffffffffffffffffffffffffffffffffffff8116036101e357565b610124359061029582610269565b565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356102d281610269565b688000000000ab143c065c6103f8576001688000000000ab143c065d335f5260016020526103218160405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5480156103d0576103cc91335f5260016020525f61036082604083209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5561036c823383611f86565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd399268360203392a35f688000000000ab143c065d6040519081529081906020820190565b0390f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b5f9103126101e357565b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b90816102009103126101e35790565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e3576104d4903690600401610476565b688000000000ab143c065c6103f8576001688000000000ab143c065d60ff60035460401c166105c1576101c06105786105717fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d6105a26103cc9561053833826121ae565b98939992918a98829661054d843033886123cf565b61055b6101e0820182611a1d565b96909101359561056a8761061c565b3691611b72565b9289612580565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d6040519081529081906020820190565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ffffffff8116036101e357565b60043590610295826105e9565b60843590610295826105e9565b3590610295826105e9565b67ffffffffffffffff8116036101e357565b60a435906102958261061c565b35906102958261061c565b908160409103126101e35790565b908160609103126101e35790565b346101e3576101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35761069a6105f7565b602435906044356064356106ac610604565b6106b461062e565b60c4359060e435926101043567ffffffffffffffff81116101e3576106dd903690600401610646565b946106e6610287565b96610144359967ffffffffffffffff8b116101e35761070c6107129b3690600401610654565b99611ba8565b6040805192835267ffffffffffffffff91909116602083015290f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576020604051624c4b408152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b600311156107d557565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b919091604064ffffffffff81606084019573ffffffffffffffffffffffffffffffffffffffff8151168552602081015161083b816107cb565b6020860152015116910152565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004355f6040805161088881611aa8565b82815282602082015201525f525f6020526103cc60405f2064ffffffffff604051916108b383611aa8565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c166108dd816107cb565b602084015260a81c16604082015260405191829182610802565b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357602060ff60035460401c166040519015158152f35b9181601f840112156101e35782359167ffffffffffffffff83116101e357602083818601950101116101e357565b346101e35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e3576109b790369060040161093a565b9060243567ffffffffffffffff81116101e3576109d890369060040161093a565b9290688000000000ab143c065c6103f8576001688000000000ab143c065d60ff60035460401c166105c1576040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169460a082602481895afa918215610d0557610ac4610aab610aab60806004966020955f91610dbb575b50015173ffffffffffffffffffffffffffffffffffffffff1690565b73ffffffffffffffffffffffffffffffffffffffff1690565b604051938480927f2c1219210000000000000000000000000000000000000000000000000000000082525afa8015610d0557610b52936020935f92610d8c575b505f73ffffffffffffffffffffffffffffffffffffffff6040518097819682957f57ecfd280000000000000000000000000000000000000000000000000000000084528b8d60048601611d95565b0393165af1908115610d05575f91610d5d575b5015610d3557610b7491613330565b9196959294939291610b8530610aab565b93848103610d0a5750610ba7610ba2610bfb9360a0933691611b72565b613452565b96610bb9602089015163ffffffff1690565b9060405180809581947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa908115610d05575f91610cd6575b50610c1f610c1b8251151590565b1590565b908115610cbe575b50610c8d57508103610c5f5750610c5192610c4191611de9565b90610c4b8161358f565b906137fa565b5f688000000000ab143c065d005b7fc21fa2e5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5ffd5b7f6a96659e000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6020015163ffffffff8381169116141590505f610c27565b610cf8915060a03d60a011610cfe575b610cf08183611ac9565b810190611c9c565b5f610c0d565b503d610ce6565b611d22565b7fc7286ea1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f514d840a000000000000000000000000000000000000000000000000000000005f5260045ffd5b610d7f915060203d602011610d85575b610d778183611ac9565b810190611d42565b5f610b65565b503d610d6d565b610dad919250843d8611610db4575b610da58183611ac9565b810190611d2d565b905f610b04565b503d610d9b565b610dd4915060a03d60a011610cfe57610cf08183611ac9565b5f610a8f565b346101e35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357600435610e1581610269565b60243590610e2282610269565b6044356064359260843567ffffffffffffffff81116101e357610e4990369060040161093a565b909460a43593303303610f0f57610f0096610efb9382610e6d610ecf94888b611f86565b73ffffffffffffffffffffffffffffffffffffffff604051998a967f3866c1dc000000000000000000000000000000000000000000000000000000006020890152602488015216604486015260648501526080608485015260a4840191611d57565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101855284611ac9565b61396e565b91905015610f0a57005b6139ac565b7f14d4a4e8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760206040516e22d473030f116ddee9f6b43ac78ba38152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357610faf611f4f565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a35f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755005b346101e35760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e35761106a903690600401610476565b60243561107681610269565b60443567ffffffffffffffff81116101e357611096903690600401610654565b688000000000ab143c065c6103f8576001688000000000ab143c065d60ff60035460401c166105c1576110c982846121ae565b959291849791949585610100840135823592602081013560405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152611120606082611ac9565b519020906040519261113360a085611ac9565b606a8452602084017f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c41759052604084017f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b659052606084017f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e74329052608084017f353620616d6f756e7429000000000000000000000000000000000000000000009052604081016111e091611a1d565b956111ee9791958c8c6140b0565b846111fd6101e0840184611a1d565b936101c00161120b90611a6e565b93369061121792611b72565b916112229488612580565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a35f688000000000ab143c065d604051908152602090f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760206040516107d08152f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576020600254604051908152f35b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356113b3611f4f565b670de0b6b3a764000081116113c757600255005b7fad6bb6d1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b346101e35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e3576114b1903690600401610476565b6114f560243561010083013592610140810135916114ce8361061c565b610160820135906114de8261061c565b610180830135916101a06002549401359487613a35565b9081810390811161151157604080519182526020820192909252f35b611dbc565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e357366023820112156101e357806004013567ffffffffffffffff81116101e3578060051b36602482850101116101e357346101e3576040519283926020845280846020015260408481019380602485018637850101928391611602575b505050806040520360401b17613b3f565b919350915b80915f6044825187016024810135918291018537389084305af41561168e57602067ffffffffffffffe0603f5f937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08a87030181528301943d81523d858583013e3d010116933d0101528483821015611681575090611607565b93505090505f80806115f1565b853d5f823e3d90fd5b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760206040516103e88152f35b346101e35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357602061176160043561171181610269565b73ffffffffffffffffffffffffffffffffffffffff6024359161173383610269565b165f526001835260405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b54604051908152f35b346101e3576101207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356117a6816105e9565b602435906044356064356084356117bc816105e9565b60a435906117c98261061c565b60c4359260e43594610104359767ffffffffffffffff89116101e3576117f6610712993690600401610646565b97611df6565b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043561183281610269565b61183a611f4f565b63389a75e1600c52805f526020600c209081544211611860575f61185e92556139b4565b005b636f5e88185f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356118a381610269565b6118ab611f4f565b8060601b156118bd5761185e906139b4565b637448fbae5f526004601cfd5b60ff8116036101e357565b3590610295826118ca565b346101e35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043561191b81610269565b6024356044356064359261192e846118ca565b73ffffffffffffffffffffffffffffffffffffffff169160a43590608435843b156101e3575f9460e493869260ff604051998a9889977fd505accf0000000000000000000000000000000000000000000000000000000089523360048a01523060248a01526044890152606488015216608486015260a485015260c48401525af16119b557005b806119c35f61185e93611ac9565b80610405565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357600435611a0481610269565b63389a75e1600c525f52602080600c2054604051908152f35b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101e3570180359067ffffffffffffffff82116101e3576020019181360383136101e357565b35611a788161061c565b90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6060810190811067ffffffffffffffff821117611ac457604052565b611a7b565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117611ac457604052565b6040519061029561020083611ac9565b60405190610295604083611ac9565b60405190610295606083611ac9565b67ffffffffffffffff8111611ac457601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b929192611b7e82611b38565b91611b8c6040519384611ac9565b8294818452818301116101e3578281602093845f960137010152565b9296919a999498959093979860ff60035460401c166105c1578b881015611c615787611bd2612843565b9c611bdf968e968a61290b565b948035611beb8161061c565b67ffffffffffffffff166101c087015260208101611c0891611a1d565b3690611c1392611b72565b6101e0860152611c2285612bbf565b604080516020810185815263ffffffff871682840152918152611c46606082611ac9565b51902090611c549286612f74565b611c5d9261304c565b9190565b8b887fed2bc1ea000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5190610295826105e9565b908160a09103126101e3576040519060a082019082821067ffffffffffffffff831117611ac4576080916040528051611cd4816101d9565b83526020810151611ce4816105e9565b60208401526040810151611cf7816105e9565b60408401526060810151611d0a81610269565b60608401520151611d1a81610269565b608082015290565b6040513d5f823e3d90fd5b908160209103126101e35751611a7881610269565b908160209103126101e35751611a78816101d9565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b9290611dae90611a789593604086526040860191611d57565b926020818503910152611d57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9190820391821161151157565b929498979395909896919660ff60035460401c166105c15787861015611f1d579261056a92611e3692611e5f95888b611e2d612843565b9e8f963361290b565b92611e55611e4382611a6e565b67ffffffffffffffff166101c0860152565b6020810190611a1d565b6101e0820152611e6e81612bbf565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a0816024817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa948515610d0557611f0f6060611c5d97611f18945f91610dbb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b309033906123cf565b61304c565b7fed2bc1ea000000000000000000000000000000000000000000000000000000005f526004869052602488905260445ffd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927543303611f7957565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f51141615611fc2575b50505f603452565b3b153d171015611fd3575f80611fba565b6390b8ec185f526004601cfd5b35611a78816118ca565b35611a78816105e9565b9080601f830112156101e357816020611a7893359101611b72565b919091610200818403126101e357612025611b0a565b9261202f826118d5565b845261203d60208301610611565b602085015261204e60408301610611565b6040850152606082013560608501526080820135608085015260a082013560a085015260c082013560c085015260e082013560e085015261010082013561010085015261209e610120830161063b565b6101208501526120b1610140830161063b565b6101408501526120c4610160830161063b565b6101608501526101808201356101808501526101a08201356101a08501526120ef6101c0830161063b565b6101c08501526101e082013567ffffffffffffffff81116101e3576121149201611ff4565b6101e0830152565b61212660016107cb565b740100000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b61216f60026107cb565b740200000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b91909160ff6121bc82611fe0565b1661239057604081016121da6121d182611fea565b63ffffffff1690565b460361235357506121f36121ee368361200f565b61358f565b92612205845f525f60205260405f2090565b91612215835460ff9060a01c1690565b61221e816107cb565b6123265761223461222f368361200f565b613b50565b936102956122456080840135613c3b565b946122d461229261228b610100870135966122636101408201611a6e565b906122716101608201611a6e565b91610180820135906101a06002549301359342918c613a35565b8096611de9565b95829073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b6122dd8161211c565b80547fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff164260a81b79ffffffffff00000000000000000000000000000000000000000016179055565b7f343e211e000000000000000000000000000000000000000000000000000000005f52600485905260245ffd5b61235f610c8a91611fea565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5263ffffffff16600452602490565b61239c610c8a91611fe0565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f90815260045260ff16602452604490565b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f51141615612418575b50505f606052604052565b3b153d171015612429575f8061240d565b637939f4245f526004601cfd5b67ffffffffffffffff61c3509116019067ffffffffffffffff821161151157565b9067ffffffffffffffff8091169116019067ffffffffffffffff821161151157565b94909367ffffffffffffffff9373ffffffffffffffffffffffffffffffffffffffff6124ca948160a0989b9a9b1689521660208801526040870152606086015260c0608086015260c0850190611516565b9416910152565b905f60208301926124e1826107cb565b52565b90600260208301926124e1826107cb565b90600160208301926124e1826107cb565b60405190612515602083611ac9565b5f8252565b3d15612544573d9061252b82611b38565b916125396040519384611ac9565b82523d5f602084013e565b606090565b906020820180921161151157565b90601f820180921161151157565b601201908160121161151157565b9190820180921161151157565b9394909194831561283b578051158015612832575b612825575a6125ca6125bd6125b8600586901c6707ffffffffffffff1686612457565b612436565b67ffffffffffffffff1690565b116127e857303b156101e357612614915f9160405193849283927f617c537c00000000000000000000000000000000000000000000000000000000845289898c8a60048801612479565b038183305af190816127d4575b5061278c5761263661263161251a565b613dc1565b9073ffffffffffffffffffffffffffffffffffffffff82169485151580612782575b1561269d57509061266a929184613c87565b7f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a960405180612698816124f5565b0390a3565b73ffffffffffffffffffffffffffffffffffffffff8096508293508161272a87946127077f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9573ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b9073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b612735878254612573565b9055169586936127516040519283921696829190602083019252565b0390a47f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a960405180612698816124e4565b5030861415612658565b50507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a973ffffffffffffffffffffffffffffffffffffffff60405193169280612698816124d1565b806119c35f6127e293611ac9565b5f612621565b610c8a825a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045267ffffffffffffffff16602452604490565b5050610295939192613c87565b50853b15612595565b505050505050565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff81116115115767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b6128a2611b0a565b905f82525f60208301525f60408301525f60608301525f60808301525f60a08301525f60c08301525f60e08301525f6101008301525f6101208301525f6101408301525f6101608301525f6101808301525f6101a08301525f6101c083015260606101e0830152565b94939792909761291961289a565b506040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016969060a0816024818b5afa8015610d05578b6129bd606060a0936129ff955f91612ba85750015173ffffffffffffffffffffffffffffffffffffffff1690565b9960405180809581947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa908115610d0557612a33916060915f91610dbb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b9073ffffffffffffffffffffffffffffffffffffffff881615612b765773ffffffffffffffffffffffffffffffffffffffff821615612b445773ffffffffffffffffffffffffffffffffffffffff9081169791811692911690612a969084611de9565b934267ffffffffffffffff1696612aad9088612457565b97612ab6611b0a565b5f815263ffffffff461660208201529b63ffffffff1660408d015260608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015267ffffffffffffffff1661014085015267ffffffffffffffff166101608401526101808301526101a08201525f6101c0820152612b3b612506565b6101e082015290565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff8b1660045260245ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f524663ffffffff1660045260245ffd5b610dd49150853d8711610cfe57610cf08183611ac9565b602081015163ffffffff16468103612d8b5750612beb612be6604083015163ffffffff1690565b613e40565b61016081015161014082015167ffffffffffffffff918216911680821115612d505750506101008101805180158015612d43575b612d0d57506101a082015190519081811015612cdf57505073ffffffffffffffffffffffffffffffffffffffff612c596080830151613c3b565b1615612cb7576101c0015167ffffffffffffffff16624c4b408111612c7b5750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff16600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e08301517f8dd09d91000000000000000000000000000000000000000000000000000000005f5260049190915260245260445ffd5b5060e08301518111612c1f565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff9081166004521660245260445ffd5b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b9067ffffffffffffffff8091169116039067ffffffffffffffff821161151157565b60405190612dee61016083611ac9565b61012982527f3620616d6f756e74290000000000000000000000000000000000000000000000610140837f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208201527f75696e743820627269646765547970652c75696e74333220647374436861696e60408201527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608201527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808201527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08201527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08201527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08201527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008201527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208201520152565b9161029593919261302360a061301b612f8e845160ff1690565b84612fa0604082015163ffffffff1690565b9760808201519860e08301998a51610100850151612fe3612fcd61016088015167ffffffffffffffff1690565b61014088015167ffffffffffffffff1690612dbc565b91610180870151936101a0880151956130156101c06101e08b0151602081519101209a015167ffffffffffffffff1690565b99613f50565b920151613c3b565b9251823560208401359180613044613039612dde565b966040810190611a1d565b9890976140b0565b909291926130598261358f565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015290949073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001660a082602481845afa918215610d05575f926132ba575b508192608061316a930193613113613102865173ffffffffffffffffffffffffffffffffffffffff1690565b602083015163ffffffff169061425a565b60a06040880193613128855163ffffffff1690565b9060405180809881947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa918215610d055789967f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a9661320a60606131ee6131d3602061321f9973ffffffffffffffffffffffffffffffffffffffff9c5f9161329b575b50015163ffffffff1690565b935173ffffffffffffffffffffffffffffffffffffffff1690565b95015173ffffffffffffffffffffffffffffffffffffffff1690565b60e08b0151906132198c6143a4565b95614542565b61269861323d6132326060880151613c3b565b925163ffffffff1690565b9560c08101519061326061012061010083015192015167ffffffffffffffff1690565b604080515f815263ffffffff909a1660208b0152890192909252606088015267ffffffffffffffff16608087015291169390819060a0820190565b6132b4915060a03d60a011610cfe57610cf08183611ac9565b5f6131c7565b61316a92506132d79060a03d60a011610cfe57610cf08183611ac9565b916130d6565b909392938483116101e35784116101e3578101920390565b359060208110613303575090565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9060200360031b1b1690565b61017882106133ca57816008116101e357600481013560e01c9261336961336361335a60b8612549565b60b886866132dd565b906132f5565b9161338361336361337a60d8612549565b60d887856132dd565b916133c66133a061336361339760f8612549565b60f889876132dd565b956133bb6133636101386133b381612549565b9084886132dd565b9381610178916132dd565b9091565b507fa2abf1b6000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5190610295826118ca565b51906102958261061c565b81601f820112156101e35780519061342382611b38565b926134316040519485611ac9565b828452602083830101116101e357815f9260208093018386015e8301015290565b61345a61289a565b5080518101906020818303126101e35760208101519067ffffffffffffffff82116101e3570190610200828203126101e357613494611b0a565b916134a1602082016133f6565b83526134af60408201611c91565b60208401526134c060608201611c91565b60408401526080810151606084015260a0810151608084015260c081015160a084015260e081015160c084015261010081015160e08401526101208101516101008401526135116101408201613401565b6101208401526135246101608201613401565b6101408401526135376101808201613401565b6101608401526101a08101516101808401526101c08101516101a08401526135626101e08201613401565b6101c08401526102008101519167ffffffffffffffff83116101e357612b3b92602080920192010161340c565b906101e0820161372561371b6125bd6101c06135d56135c06135bb6135b5885151612557565b60051c90565b612565565b90604051828193825260010160051b01604052565b602080820152966135fc6135f36135ed835160ff1690565b60ff1690565b60408a01528890565b5061361d6136146121d1602084015163ffffffff1690565b60608a01528890565b5061363e6136356121d1604084015163ffffffff1690565b60808a01528890565b50606081015160a0890152608081015160c089015260a081015160e089015260c081015161010089015260e08101516101208901526101008101516101408901526136a561369b6125bd61012084015167ffffffffffffffff1690565b6101608a01528890565b506136cc6136c26125bd61014084015167ffffffffffffffff1690565b6101808a01528890565b506136f36136e96125bd61016084015167ffffffffffffffff1690565b6101a08a01528890565b506101808101516101c08901526101a08101516101e0890152015167ffffffffffffffff1690565b6102008501528390565b506102006102208401528051516102408401525180518061376a575b5050815160051b602083012061029590928051604051828260010160051b011490151060061b52565b602082016020826102608701940101905b8181106137ea575050601f169081156137415760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80613741565b805184526020938401930161377b565b9161380c825f525f60205260405f2090565b805490600260a083901c60ff16613822816107cb565b1461394157916138a6917f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a9361385787613b50565b9161388261386860808a0151613c3b565b9173ffffffffffffffffffffffffffffffffffffffff1690565b6101008901518086101561393b575084905b61389e8287611de9565b968794612165565b73ffffffffffffffffffffffffffffffffffffffff811699868b15613908575050906138d391858a613c87565b816138f6575b5050505b6040805191825260208201929092529081908101612698565b6139009287613c87565b5f82816138d9565b925092905061393694935061392f6101c06101e085015194015167ffffffffffffffff1690565b9389612580565b6138dd565b90613894565b7fb196a44a000000000000000000000000000000000000000000000000000000005f52600484905260245ffd5b5f9081936040519383389360208451940192f1923d60243d116139a0575b806020918452805f8386013e830101604052565b5060019250602461398c565b805190602001fd5b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b959290949167ffffffffffffffff8091169516908181105f14613b375750925b5f93858110613a7d575b505050613a6c9250612573565b81811115613a78575090565b905090565b9091929350840393841161151157801580159081613ad2575b5093613aba9291613a6c955f14613acb575050805b808211613ac3575b5084614192565b905f8080613a5f565b90505f613ab3565b0290613aab565b9050919091613b0a57907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8190048411613aba613a96565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b905092613a55565b8060401c9067ffffffffffffffff16f35b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a0816024817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa8015610d05576060613bec9160c0935f91610dbb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b91015173ffffffffffffffffffffffffffffffffffffffff82168103613c10575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8060a01c613c5c5773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b908315613dbb57604051925f80602086017fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84169687602482015288604482015260448152613cee606482611ac9565b519082865af1613cfc61251a565b81613d8c575b50613d855781613d6a7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9361270773ffffffffffffffffffffffffffffffffffffffff9473ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b613d75878254612573565b90556040519586521693602090a4565b5050505050565b8051801592508215613da1575b50505f613d02565b613db49250602080918301019101611d42565b5f80613d99565b50505050565b905f916024815114613dd05750565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff0000000000000000000000000000000000000000000000000000000060246020840151930151921614613e265750565b73ffffffffffffffffffffffffffffffffffffffff169150565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815263ffffffff8216600482015260a0816024817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa908115610d05575f91613f31575b50613ecd610c1b8251151590565b908115613f0e575b50613edd5750565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6060015173ffffffffffffffffffffffffffffffffffffffff161590505f613ed5565b613f4a915060a03d60a011610cfe57610cf08183611ac9565b5f613ebf565b99979593919896949290986040519960208b019b7f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b8d5260ff1660408c015263ffffffff1660608b015260808a015260a089015260c088015267ffffffffffffffff1660e087015261010086015261012085015261014084015261016083015267ffffffffffffffff166101808201526101808152613ff16101a082611ac9565b51902090565b9593906140a193614082611a78999794604073ffffffffffffffffffffffffffffffffffffffff9461404a8c82516020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b6020818101518d84015291015160608c0152815173ffffffffffffffffffffffffffffffffffffffff1660808c0152015160a08a0152565b1660c087015260e0860152610140610100860152610140850190611516565b92610120818503910152611d57565b95949198939098979296976140e7604051976140cd60408a611ac9565b73ffffffffffffffffffffffffffffffffffffffff168852565b60208701526140f4611b29565b95865260208601526040850152614109611b1a565b3081529460208601526e22d473030f116ddee9f6b43ac78ba33b156101e3575f956141629360405198899788977f137c29fe00000000000000000000000000000000000000000000000000000000895260048901613ff7565b0381836e22d473030f116ddee9f6b43ac78ba35af18015610d05576141845750565b806119c35f61029593611ac9565b81810291670de0b6b3a7640000818385041483151702156141bd575050670de0b6b3a7640000900490565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff670de0b6b3a76400009284098481108501900392099080670de0b6b3a7640000111561423857828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690290565b63ae47f7025f526004601cfd5b908160209103126101e35751611a78816105e9565b90602073ffffffffffffffffffffffffffffffffffffffff926004604051809581937f2c121921000000000000000000000000000000000000000000000000000000008352165afa918215610d055773ffffffffffffffffffffffffffffffffffffffff926020915f91614387575b506004604051809581937f8d3638f4000000000000000000000000000000000000000000000000000000008352165afa918215610d05575f92614356575b5063ffffffff811663ffffffff83160361431f575050565b7fc9e030e8000000000000000000000000000000000000000000000000000000005f5263ffffffff9081166004521660245260445ffd5b61437991925060203d602011614380575b6143718183611ac9565b810190614245565b905f614307565b503d614367565b61439e9150823d8411610db457610da58183611ac9565b5f6142c9565b611a786101e0916144bc6040519384926020808501526143ca60408501825160ff169052565b602081015163ffffffff166060850152604081015163ffffffff166080850152606081015160a0850152608081015160c085015260a081015160e085015260c081015161010085015260e081015161012085015261010081015161014085015261444761012082015161016086019067ffffffffffffffff169052565b61014081015167ffffffffffffffff1661018085015261016081015167ffffffffffffffff166101a08501526101808101516101c08501526101a0810151828501526144a66101c082015161020086019067ffffffffffffffff169052565b0151610200610220840152610240830190611516565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282611ac9565b949063ffffffff94611a7899989486610100999573ffffffffffffffffffffffffffffffffffffffff948a521660208901526040880152166060860152608085015260a08401521660c08201528160e08201520190611516565b95909192939580601452836034526f095ea7b30000000000000000000000005f5260205f6044601082875af18060015f511416156145f1575b505f60345273ffffffffffffffffffffffffffffffffffffffff1692833b156101e3576145e05f96928793604051998a98899788967f779b432d0000000000000000000000000000000000000000000000000000000088523092309160048a016144e8565b03925af18015610d05576141845750565b3d843b15171015614603575b5f61457b565b5f6034526f095ea7b30000000000000000000000005f525f386044601083875af1508360345260205f6044601082875af18060015f51141615614647575b506145fd565b3d843b15171015614658575f614641565b633e3f8f735f526004601cfdfea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA04a\x01\x04W`\x1FaG\xA38\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x08W\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\x01\x04Wa\0G\x81a\x01\x1CV[\x90`@a\0V` \x83\x01a\x01\x1CV[\x91\x01Q\x90g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11a\0\xF1W`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3`\x02U`\x01`\x01`\xA0\x1B\x03\x16`\x80R`@QaFr\x90\x81a\x011\x829`\x80Q\x81\x81\x81a\nF\x01R\x81\x81a\x13\x1A\x01R\x81\x81a\x1E\xA2\x01R\x81\x81a)]\x01R\x81\x81a0\x9F\x01R\x81\x81a;\x84\x01Ra>z\x01R\xF3[Pc\xADk\xB6\xD1`\xE0\x1B_R`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\x04WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x16\xC3\x8B<\x14a\x01\xD4W\x80c\x1E\x83@\x9A\x14a\x01\xCFW\x80c%i)b\x14a\x01\xCAW\x80c1\xEE\xE4M\x14a\x01\xC5W\x80c4#j9\x14a\x01\xC0W\x80c9\xC32\x15\x14a\x01\xBBW\x80cT\xD1\xF1=\x14a\x01\xB6W\x80cWxG*\x14a\x01\xB1W\x80c\\\x97Z\xBB\x14a\x01\xACW\x80c_\xDC|\x12\x14a\x01\xA7W\x80ca|S|\x14a\x01\xA2W\x80cj\xFD\xD8P\x14a\x01\x9DW\x80cqP\x18\xA6\x14a\x01\x98W\x80cwo\xF3\xC7\x14a\x01\x93W\x80cw\x83\x9A\x9E\x14a\x01\x8EW\x80cyP,U\x14a\x01\x89W\x80c\x85\xC1x0\x14a\x01\x84W\x80c\x8C\xDA\x96\xDE\x14a\x01\x7FW\x80c\x8D\xA5\xCB[\x14a\x01zW\x80c\x97\xC3k\xAE\x14a\x01uW\x80c\xAC\x96P\xD8\x14a\x01pW\x80c\xCCn\xECp\x14a\x01kW\x80c\xD4W\x0C\x1C\x14a\x01fW\x80c\xE3\xBB\x93\xE8\x14a\x01aW\x80c\xF0N(>\x14a\x01\\W\x80c\xF2\xFD\xE3\x8B\x14a\x01WW\x80c\xF3\x99\\g\x14a\x01RWc\xFE\xE8\x1C\xF4\x14a\x01MW_\x80\xFD[a\x19\xC9V[a\x18\xE0V[a\x18mV[a\x17\xFCV[a\x17jV[a\x16\xD1V[a\x16\x97V[a\x15YV[a\x14bV[a\x13\xF2V[a\x13yV[a\x13>V[a\x12\xD0V[a\x12\x96V[a\x10\x1BV[a\x0F~V[a\x0F7V[a\r\xDAV[a\thV[a\x08\xF7V[a\x08HV[a\x07iV[a\x07.V[a\x06bV[a\x04\x85V[a\x04\x0FV[a\x02\x97V[a\x01\xE7V[\x80\x15\x15\x03a\x01\xE3WV[_\x80\xFD[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x02\"\x81a\x01\xD9V[a\x02*a\x1FOV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92\x15\x15`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xE3WV[a\x01$5\x90a\x02\x95\x82a\x02iV[V[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x02\xD2\x81a\x02iV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R`\x01` Ra\x03!\x81`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T\x80\x15a\x03\xD0Wa\x03\xCC\x913_R`\x01` R_a\x03`\x82`@\x83 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[Ua\x03l\x823\x83a\x1F\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83` 3\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[_\x91\x03\x12a\x01\xE3WV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[\x90\x81a\x02\0\x91\x03\x12a\x01\xE3W\x90V[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x04\xD4\x906\x90`\x04\x01a\x04vV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xC1Wa\x01\xC0a\x05xa\x05q\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x05\xA2a\x03\xCC\x95a\x0583\x82a!\xAEV[\x98\x93\x99\x92\x91\x8A\x98\x82\x96a\x05M\x8403\x88a#\xCFV[a\x05[a\x01\xE0\x82\x01\x82a\x1A\x1DV[\x96\x90\x91\x015\x95a\x05j\x87a\x06\x1CV[6\x91a\x1BrV[\x92\x89a%\x80V[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xE3WV[`\x045\x90a\x02\x95\x82a\x05\xE9V[`\x845\x90a\x02\x95\x82a\x05\xE9V[5\x90a\x02\x95\x82a\x05\xE9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xE3WV[`\xA45\x90a\x02\x95\x82a\x06\x1CV[5\x90a\x02\x95\x82a\x06\x1CV[\x90\x81`@\x91\x03\x12a\x01\xE3W\x90V[\x90\x81``\x91\x03\x12a\x01\xE3W\x90V[4a\x01\xE3Wa\x01`\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wa\x06\x9Aa\x05\xF7V[`$5\x90`D5`d5a\x06\xACa\x06\x04V[a\x06\xB4a\x06.V[`\xC45\x90`\xE45\x92a\x01\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x06\xDD\x906\x90`\x04\x01a\x06FV[\x94a\x06\xE6a\x02\x87V[\x96a\x01D5\x99g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x11a\x01\xE3Wa\x07\x0Ca\x07\x12\x9B6\x90`\x04\x01a\x06TV[\x99a\x1B\xA8V[`@\x80Q\x92\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R\x90\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@QbLK@\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[`\x03\x11\x15a\x07\xD5WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x91`@d\xFF\xFF\xFF\xFF\xFF\x81``\x84\x01\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R` \x81\x01Qa\x08;\x81a\x07\xCBV[` \x86\x01R\x01Q\x16\x91\x01RV[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045_`@\x80Qa\x08\x88\x81a\x1A\xA8V[\x82\x81R\x82` \x82\x01R\x01R_R_` Ra\x03\xCC`@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x08\xB3\x83a\x1A\xA8V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16a\x08\xDD\x81a\x07\xCBV[` \x84\x01R`\xA8\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82a\x08\x02V[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xE3W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xE3W` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xE3WV[4a\x01\xE3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\t\xB7\x906\x90`\x04\x01a\t:V[\x90`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\t\xD8\x906\x90`\x04\x01a\t:V[\x92\x90h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xC1W`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94`\xA0\x82`$\x81\x89Z\xFA\x91\x82\x15a\r\x05Wa\n\xC4a\n\xABa\n\xAB`\x80`\x04\x96` \x95_\x91a\r\xBBW[P\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x93\x84\x80\x92\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\r\x05Wa\x0BR\x93` \x93_\x92a\r\x8CW[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x80\x97\x81\x96\x82\x95\x7FW\xEC\xFD(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x8B\x8D`\x04\x86\x01a\x1D\x95V[\x03\x93\x16Z\xF1\x90\x81\x15a\r\x05W_\x91a\r]W[P\x15a\r5Wa\x0Bt\x91a30V[\x91\x96\x95\x92\x94\x93\x92\x91a\x0B\x850a\n\xABV[\x93\x84\x81\x03a\r\nWPa\x0B\xA7a\x0B\xA2a\x0B\xFB\x93`\xA0\x936\x91a\x1BrV[a4RV[\x96a\x0B\xB9` \x89\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x95\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\r\x05W_\x91a\x0C\xD6W[Pa\x0C\x1Fa\x0C\x1B\x82Q\x15\x15\x90V[\x15\x90V[\x90\x81\x15a\x0C\xBEW[Pa\x0C\x8DWP\x81\x03a\x0C_WPa\x0CQ\x92a\x0CA\x91a\x1D\xE9V[\x90a\x0CK\x81a5\x8FV[\x90a7\xFAV[_h\x80\0\0\0\0\xAB\x14<\x06]\0[\x7F\xC2\x1F\xA2\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[_\xFD[\x7Fj\x96e\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[` \x01Qc\xFF\xFF\xFF\xFF\x83\x81\x16\x91\x16\x14\x15\x90P_a\x0C'V[a\x0C\xF8\x91P`\xA0=`\xA0\x11a\x0C\xFEW[a\x0C\xF0\x81\x83a\x1A\xC9V[\x81\x01\x90a\x1C\x9CV[_a\x0C\rV[P=a\x0C\xE6V[a\x1D\"V[\x7F\xC7(n\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FQM\x84\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a\r\x7F\x91P` =` \x11a\r\x85W[a\rw\x81\x83a\x1A\xC9V[\x81\x01\x90a\x1DBV[_a\x0BeV[P=a\rmV[a\r\xAD\x91\x92P\x84=\x86\x11a\r\xB4W[a\r\xA5\x81\x83a\x1A\xC9V[\x81\x01\x90a\x1D-V[\x90_a\x0B\x04V[P=a\r\x9BV[a\r\xD4\x91P`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[_a\n\x8FV[4a\x01\xE3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x0E\x15\x81a\x02iV[`$5\x90a\x0E\"\x82a\x02iV[`D5`d5\x92`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x0EI\x906\x90`\x04\x01a\t:V[\x90\x94`\xA45\x9303\x03a\x0F\x0FWa\x0F\0\x96a\x0E\xFB\x93\x82a\x0Ema\x0E\xCF\x94\x88\x8Ba\x1F\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x99\x8A\x96\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x89\x01R`$\x88\x01R\x16`D\x86\x01R`d\x85\x01R`\x80`\x84\x85\x01R`\xA4\x84\x01\x91a\x1DWV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x85R\x84a\x1A\xC9V[a9nV[\x91\x90P\x15a\x0F\nW\0[a9\xACV[\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wa\x0F\xAFa\x1FOV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\0[4a\x01\xE3W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x10j\x906\x90`\x04\x01a\x04vV[`$5a\x10v\x81a\x02iV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x10\x96\x906\x90`\x04\x01a\x06TV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xC1Wa\x10\xC9\x82\x84a!\xAEV[\x95\x92\x91\x84\x97\x91\x94\x95\x85a\x01\0\x84\x015\x825\x92` \x81\x015`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\x11 ``\x82a\x1A\xC9V[Q\x90 \x90`@Q\x92a\x113`\xA0\x85a\x1A\xC9V[`j\x84R` \x84\x01\x7FFillAuthorization witness)FillAu\x90R`@\x84\x01\x7Fthorization(bytes32 orderId)Toke\x90R``\x84\x01\x7FnPermissions(address token,uint2\x90R`\x80\x84\x01\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90R`@\x81\x01a\x11\xE0\x91a\x1A\x1DV[\x95a\x11\xEE\x97\x91\x95\x8C\x8Ca@\xB0V[\x84a\x11\xFDa\x01\xE0\x84\x01\x84a\x1A\x1DV[\x93a\x01\xC0\x01a\x12\x0B\x90a\x1AnV[\x936\x90a\x12\x17\x92a\x1BrV[\x91a\x12\"\x94\x88a%\x80V[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qa\x07\xD0\x81R\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `\x02T`@Q\x90\x81R\xF3[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x13\xB3a\x1FOV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x13\xC7W`\x02U\0[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x01\xE3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x14\xB1\x906\x90`\x04\x01a\x04vV[a\x14\xF5`$5a\x01\0\x83\x015\x92a\x01@\x81\x015\x91a\x14\xCE\x83a\x06\x1CV[a\x01`\x82\x015\x90a\x14\xDE\x82a\x06\x1CV[a\x01\x80\x83\x015\x91a\x01\xA0`\x02T\x94\x015\x94\x87a:5V[\x90\x81\x81\x03\x90\x81\x11a\x15\x11W`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[a\x1D\xBCV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3W6`#\x82\x01\x12\x15a\x01\xE3W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3W\x80`\x05\x1B6`$\x82\x85\x01\x01\x11a\x01\xE3W4a\x01\xE3W`@Q\x92\x83\x92` \x84R\x80\x84` \x01R`@\x84\x81\x01\x93\x80`$\x85\x01\x867\x85\x01\x01\x92\x83\x91a\x16\x02W[PPP\x80`@R\x03`@\x1B\x17a;?V[\x91\x93P\x91[\x80\x91_`D\x82Q\x87\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x16\x8EW` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?_\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8A\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x93=\x01\x01R\x84\x83\x82\x10\x15a\x16\x81WP\x90a\x16\x07V[\x93PP\x90P_\x80\x80a\x15\xF1V[\x85=_\x82>=\x90\xFD[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qa\x03\xE8\x81R\xF3[4a\x01\xE3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` a\x17a`\x045a\x17\x11\x81a\x02iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91a\x173\x83a\x02iV[\x16_R`\x01\x83R`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x01\xE3Wa\x01 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x17\xA6\x81a\x05\xE9V[`$5\x90`D5`d5`\x845a\x17\xBC\x81a\x05\xE9V[`\xA45\x90a\x17\xC9\x82a\x06\x1CV[`\xC45\x92`\xE45\x94a\x01\x045\x97g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x11a\x01\xE3Wa\x17\xF6a\x07\x12\x996\x90`\x04\x01a\x06FV[\x97a\x1D\xF6V[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x182\x81a\x02iV[a\x18:a\x1FOV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x90\x81TB\x11a\x18`W_a\x18^\x92Ua9\xB4V[\0[co^\x88\x18_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x18\xA3\x81a\x02iV[a\x18\xABa\x1FOV[\x80``\x1B\x15a\x18\xBDWa\x18^\x90a9\xB4V[ctH\xFB\xAE_R`\x04`\x1C\xFD[`\xFF\x81\x16\x03a\x01\xE3WV[5\x90a\x02\x95\x82a\x18\xCAV[4a\x01\xE3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x19\x1B\x81a\x02iV[`$5`D5`d5\x92a\x19.\x84a\x18\xCAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91`\xA45\x90`\x845\x84;\x15a\x01\xE3W_\x94`\xE4\x93\x86\x92`\xFF`@Q\x99\x8A\x98\x89\x97\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R3`\x04\x8A\x01R0`$\x8A\x01R`D\x89\x01R`d\x88\x01R\x16`\x84\x86\x01R`\xA4\x85\x01R`\xC4\x84\x01RZ\xF1a\x19\xB5W\0[\x80a\x19\xC3_a\x18^\x93a\x1A\xC9V[\x80a\x04\x05V[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x1A\x04\x81a\x02iV[c8\x9Au\xE1`\x0CR_R` \x80`\x0C T`@Q\x90\x81R\xF3[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xE3W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xE3W` \x01\x91\x816\x03\x83\x13a\x01\xE3WV[5a\x1Ax\x81a\x06\x1CV[\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1A\xC4W`@RV[a\x1A{V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1A\xC4W`@RV[`@Q\x90a\x02\x95a\x02\0\x83a\x1A\xC9V[`@Q\x90a\x02\x95`@\x83a\x1A\xC9V[`@Q\x90a\x02\x95``\x83a\x1A\xC9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC4W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a\x1B~\x82a\x1B8V[\x91a\x1B\x8C`@Q\x93\x84a\x1A\xC9V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xE3W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x92\x96\x91\x9A\x99\x94\x98\x95\x90\x93\x97\x98`\xFF`\x03T`@\x1C\x16a\x05\xC1W\x8B\x88\x10\x15a\x1CaW\x87a\x1B\xD2a(CV[\x9Ca\x1B\xDF\x96\x8E\x96\x8Aa)\x0BV[\x94\x805a\x1B\xEB\x81a\x06\x1CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x87\x01R` \x81\x01a\x1C\x08\x91a\x1A\x1DV[6\x90a\x1C\x13\x92a\x1BrV[a\x01\xE0\x86\x01Ra\x1C\"\x85a+\xBFV[`@\x80Q` \x81\x01\x85\x81Rc\xFF\xFF\xFF\xFF\x87\x16\x82\x84\x01R\x91\x81Ra\x1CF``\x82a\x1A\xC9V[Q\x90 \x90a\x1CT\x92\x86a/tV[a\x1C]\x92a0LV[\x91\x90V[\x8B\x88\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[Q\x90a\x02\x95\x82a\x05\xE9V[\x90\x81`\xA0\x91\x03\x12a\x01\xE3W`@Q\x90`\xA0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x1A\xC4W`\x80\x91`@R\x80Qa\x1C\xD4\x81a\x01\xD9V[\x83R` \x81\x01Qa\x1C\xE4\x81a\x05\xE9V[` \x84\x01R`@\x81\x01Qa\x1C\xF7\x81a\x05\xE9V[`@\x84\x01R``\x81\x01Qa\x1D\n\x81a\x02iV[``\x84\x01R\x01Qa\x1D\x1A\x81a\x02iV[`\x80\x82\x01R\x90V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x01\xE3WQa\x1Ax\x81a\x02iV[\x90\x81` \x91\x03\x12a\x01\xE3WQa\x1Ax\x81a\x01\xD9V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x92\x90a\x1D\xAE\x90a\x1Ax\x95\x93`@\x86R`@\x86\x01\x91a\x1DWV[\x92` \x81\x85\x03\x91\x01Ra\x1DWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x03\x91\x82\x11a\x15\x11WV[\x92\x94\x98\x97\x93\x95\x90\x98\x96\x91\x96`\xFF`\x03T`@\x1C\x16a\x05\xC1W\x87\x86\x10\x15a\x1F\x1DW\x92a\x05j\x92a\x1E6\x92a\x1E_\x95\x88\x8Ba\x1E-a(CV[\x9E\x8F\x963a)\x0BV[\x92a\x1EUa\x1EC\x82a\x1AnV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x86\x01RV[` \x81\x01\x90a\x1A\x1DV[a\x01\xE0\x82\x01Ra\x1En\x81a+\xBFV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x94\x85\x15a\r\x05Wa\x1F\x0F``a\x1C]\x97a\x1F\x18\x94_\x91a\r\xBBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[0\x903\x90a#\xCFV[a0LV[\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x86\x90R`$\x88\x90R`D_\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a\x1FyWV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a\x1F\xC2W[PP_`4RV[;\x15=\x17\x10\x15a\x1F\xD3W_\x80a\x1F\xBAV[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5a\x1Ax\x81a\x18\xCAV[5a\x1Ax\x81a\x05\xE9V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xE3W\x81` a\x1Ax\x935\x91\x01a\x1BrV[\x91\x90\x91a\x02\0\x81\x84\x03\x12a\x01\xE3Wa %a\x1B\nV[\x92a /\x82a\x18\xD5V[\x84Ra =` \x83\x01a\x06\x11V[` \x85\x01Ra N`@\x83\x01a\x06\x11V[`@\x85\x01R``\x82\x015``\x85\x01R`\x80\x82\x015`\x80\x85\x01R`\xA0\x82\x015`\xA0\x85\x01R`\xC0\x82\x015`\xC0\x85\x01R`\xE0\x82\x015`\xE0\x85\x01Ra\x01\0\x82\x015a\x01\0\x85\x01Ra \x9Ea\x01 \x83\x01a\x06;V[a\x01 \x85\x01Ra \xB1a\x01@\x83\x01a\x06;V[a\x01@\x85\x01Ra \xC4a\x01`\x83\x01a\x06;V[a\x01`\x85\x01Ra\x01\x80\x82\x015a\x01\x80\x85\x01Ra\x01\xA0\x82\x015a\x01\xA0\x85\x01Ra \xEFa\x01\xC0\x83\x01a\x06;V[a\x01\xC0\x85\x01Ra\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa!\x14\x92\x01a\x1F\xF4V[a\x01\xE0\x83\x01RV[a!&`\x01a\x07\xCBV[t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[a!o`\x02a\x07\xCBV[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[\x91\x90\x91`\xFFa!\xBC\x82a\x1F\xE0V[\x16a#\x90W`@\x81\x01a!\xDAa!\xD1\x82a\x1F\xEAV[c\xFF\xFF\xFF\xFF\x16\x90V[F\x03a#SWPa!\xF3a!\xEE6\x83a \x0FV[a5\x8FV[\x92a\"\x05\x84_R_` R`@_ \x90V[\x91a\"\x15\x83T`\xFF\x90`\xA0\x1C\x16\x90V[a\"\x1E\x81a\x07\xCBV[a#&Wa\"4a\"/6\x83a \x0FV[a;PV[\x93a\x02\x95a\"E`\x80\x84\x015a<;V[\x94a\"\xD4a\"\x92a\"\x8Ba\x01\0\x87\x015\x96a\"ca\x01@\x82\x01a\x1AnV[\x90a\"qa\x01`\x82\x01a\x1AnV[\x91a\x01\x80\x82\x015\x90a\x01\xA0`\x02T\x93\x015\x93B\x91\x8Ca:5V[\x80\x96a\x1D\xE9V[\x95\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a\"\xDD\x81a!\x1CV[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B`\xA8\x1By\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x90UV[\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$_\xFD[a#_a\x0C\x8A\x91a\x1F\xEAV[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$\x90V[a#\x9Ca\x0C\x8A\x91a\x1F\xE0V[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x90\x81R`\x04R`\xFF\x16`$R`D\x90V[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a$\x18W[PP_``R`@RV[;\x15=\x17\x10\x15a$)W_\x80a$\rV[cy9\xF4$_R`\x04`\x1C\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\xC3P\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x15\x11WV[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x15\x11WV[\x94\x90\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa$\xCA\x94\x81`\xA0\x98\x9B\x9A\x9B\x16\x89R\x16` \x88\x01R`@\x87\x01R``\x86\x01R`\xC0`\x80\x86\x01R`\xC0\x85\x01\x90a\x15\x16V[\x94\x16\x91\x01RV[\x90_` \x83\x01\x92a$\xE1\x82a\x07\xCBV[RV[\x90`\x02` \x83\x01\x92a$\xE1\x82a\x07\xCBV[\x90`\x01` \x83\x01\x92a$\xE1\x82a\x07\xCBV[`@Q\x90a%\x15` \x83a\x1A\xC9V[_\x82RV[=\x15a%DW=\x90a%+\x82a\x1B8V[\x91a%9`@Q\x93\x84a\x1A\xC9V[\x82R=_` \x84\x01>V[``\x90V[\x90` \x82\x01\x80\x92\x11a\x15\x11WV[\x90`\x1F\x82\x01\x80\x92\x11a\x15\x11WV[`\x12\x01\x90\x81`\x12\x11a\x15\x11WV[\x91\x90\x82\x01\x80\x92\x11a\x15\x11WV[\x93\x94\x90\x91\x94\x83\x15a(;W\x80Q\x15\x80\x15a(2W[a(%WZa%\xCAa%\xBDa%\xB8`\x05\x86\x90\x1Cg\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a$WV[a$6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x11a'\xE8W0;\x15a\x01\xE3Wa&\x14\x91_\x91`@Q\x93\x84\x92\x83\x92\x7Fa|S|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x89\x89\x8C\x8A`\x04\x88\x01a$yV[\x03\x81\x830Z\xF1\x90\x81a'\xD4W[Pa'\x8CWa&6a&1a%\x1AV[a=\xC1V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x94\x85\x15\x15\x80a'\x82W[\x15a&\x9DWP\x90a&j\x92\x91\x84a<\x87V[\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a&\x98\x81a$\xF5V[\x03\x90\xA3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x96P\x82\x93P\x81a'*\x87\x94a'\x07\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[a'5\x87\x82Ta%sV[\x90U\x16\x95\x86\x93a'Q`@Q\x92\x83\x92\x16\x96\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA4\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a&\x98\x81a$\xE4V[P0\x86\x14\x15a&XV[PP\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x93\x16\x92\x80a&\x98\x81a$\xD1V[\x80a\x19\xC3_a'\xE2\x93a\x1A\xC9V[_a&!V[a\x0C\x8A\x82Z\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$R`D\x90V[PPa\x02\x95\x93\x91\x92a<\x87V[P\x85;\x15a%\x95V[PPPPPPV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x15\x11Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[a(\xA2a\x1B\nV[\x90_\x82R_` \x83\x01R_`@\x83\x01R_``\x83\x01R_`\x80\x83\x01R_`\xA0\x83\x01R_`\xC0\x83\x01R_`\xE0\x83\x01R_a\x01\0\x83\x01R_a\x01 \x83\x01R_a\x01@\x83\x01R_a\x01`\x83\x01R_a\x01\x80\x83\x01R_a\x01\xA0\x83\x01R_a\x01\xC0\x83\x01R``a\x01\xE0\x83\x01RV[\x94\x93\x97\x92\x90\x97a)\x19a(\x9AV[P`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x96\x90`\xA0\x81`$\x81\x8BZ\xFA\x80\x15a\r\x05W\x8Ba)\xBD```\xA0\x93a)\xFF\x95_\x91a+\xA8WP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x99`@Q\x80\x80\x95\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\r\x05Wa*3\x91``\x91_\x91a\r\xBBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x15a+vWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a+DWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x97\x91\x81\x16\x92\x91\x16\x90a*\x96\x90\x84a\x1D\xE9V[\x93Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96a*\xAD\x90\x88a$WV[\x97a*\xB6a\x1B\nV[_\x81Rc\xFF\xFF\xFF\xFFF\x16` \x82\x01R\x9Bc\xFF\xFF\xFF\xFF\x16`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01R_a\x01\xC0\x82\x01Ra+;a%\x06V[a\x01\xE0\x82\x01R\x90V[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04R`$_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_RFc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[a\r\xD4\x91P\x85=\x87\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16F\x81\x03a-\x8BWPa+\xEBa+\xE6`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[a>@V[a\x01`\x81\x01Qa\x01@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x80\x82\x11\x15a-PWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a-CW[a-\rWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a,\xDFWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa,Y`\x80\x83\x01Qa<;V[\x16\x15a,\xB7Wa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a,{WPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x91\x90\x91R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a,\x1FV[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x15\x11WV[`@Q\x90a-\xEEa\x01`\x83a\x1A\xC9V[a\x01)\x82R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x83\x7FOrderIntent witness)OrderIntent(` \x82\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x82\x01R\x7FId,bytes32 recipient,uint256 inp``\x82\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x82\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x82\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x82\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x82\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x82\x01R\x7FPermissions(address token,uint25a\x01 \x82\x01R\x01RV[\x91a\x02\x95\x93\x91\x92a0#`\xA0a0\x1Ba/\x8E\x84Q`\xFF\x16\x90V[\x84a/\xA0`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x97`\x80\x82\x01Q\x98`\xE0\x83\x01\x99\x8AQa\x01\0\x85\x01Qa/\xE3a/\xCDa\x01`\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01@\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a-\xBCV[\x91a\x01\x80\x87\x01Q\x93a\x01\xA0\x88\x01Q\x95a0\x15a\x01\xC0a\x01\xE0\x8B\x01Q` \x81Q\x91\x01 \x9A\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x99a?PV[\x92\x01Qa<;V[\x92Q\x825` \x84\x015\x91\x80a0Da09a-\xDEV[\x96`@\x81\x01\x90a\x1A\x1DV[\x98\x90\x97a@\xB0V[\x90\x92\x91\x92a0Y\x82a5\x8FV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x90\x94\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\xA0\x82`$\x81\x84Z\xFA\x91\x82\x15a\r\x05W_\x92a2\xBAW[P\x81\x92`\x80a1j\x93\x01\x93a1\x13a1\x02\x86Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90aBZV[`\xA0`@\x88\x01\x93a1(\x85Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x98\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x91\x82\x15a\r\x05W\x89\x96\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x96a2\n``a1\xEEa1\xD3` a2\x1F\x99s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9C_\x91a2\x9BW[P\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x93Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x95\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\xE0\x8B\x01Q\x90a2\x19\x8CaC\xA4V[\x95aEBV[a&\x98a2=a22``\x88\x01Qa<;V[\x92Qc\xFF\xFF\xFF\xFF\x16\x90V[\x95`\xC0\x81\x01Q\x90a2`a\x01 a\x01\0\x83\x01Q\x92\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@\x80Q_\x81Rc\xFF\xFF\xFF\xFF\x90\x9A\x16` \x8B\x01R\x89\x01\x92\x90\x92R``\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01R\x91\x16\x93\x90\x81\x90`\xA0\x82\x01\x90V[a2\xB4\x91P`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[_a1\xC7V[a1j\x92Pa2\xD7\x90`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[\x91a0\xD6V[\x90\x93\x92\x93\x84\x83\x11a\x01\xE3W\x84\x11a\x01\xE3W\x81\x01\x92\x03\x90V[5\x90` \x81\x10a3\x03WP\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90` \x03`\x03\x1B\x1B\x16\x90V[a\x01x\x82\x10a3\xCAW\x81`\x08\x11a\x01\xE3W`\x04\x81\x015`\xE0\x1C\x92a3ia3ca3Z`\xB8a%IV[`\xB8\x86\x86a2\xDDV[\x90a2\xF5V[\x91a3\x83a3ca3z`\xD8a%IV[`\xD8\x87\x85a2\xDDV[\x91a3\xC6a3\xA0a3ca3\x97`\xF8a%IV[`\xF8\x89\x87a2\xDDV[\x95a3\xBBa3ca\x018a3\xB3\x81a%IV[\x90\x84\x88a2\xDDV[\x93\x81a\x01x\x91a2\xDDV[\x90\x91V[P\x7F\xA2\xAB\xF1\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[Q\x90a\x02\x95\x82a\x18\xCAV[Q\x90a\x02\x95\x82a\x06\x1CV[\x81`\x1F\x82\x01\x12\x15a\x01\xE3W\x80Q\x90a4#\x82a\x1B8V[\x92a41`@Q\x94\x85a\x1A\xC9V[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xE3W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[a4Za(\x9AV[P\x80Q\x81\x01\x90` \x81\x83\x03\x12a\x01\xE3W` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xE3W\x01\x90a\x02\0\x82\x82\x03\x12a\x01\xE3Wa4\x94a\x1B\nV[\x91a4\xA1` \x82\x01a3\xF6V[\x83Ra4\xAF`@\x82\x01a\x1C\x91V[` \x84\x01Ra4\xC0``\x82\x01a\x1C\x91V[`@\x84\x01R`\x80\x81\x01Q``\x84\x01R`\xA0\x81\x01Q`\x80\x84\x01R`\xC0\x81\x01Q`\xA0\x84\x01R`\xE0\x81\x01Q`\xC0\x84\x01Ra\x01\0\x81\x01Q`\xE0\x84\x01Ra\x01 \x81\x01Qa\x01\0\x84\x01Ra5\x11a\x01@\x82\x01a4\x01V[a\x01 \x84\x01Ra5$a\x01`\x82\x01a4\x01V[a\x01@\x84\x01Ra57a\x01\x80\x82\x01a4\x01V[a\x01`\x84\x01Ra\x01\xA0\x81\x01Qa\x01\x80\x84\x01Ra\x01\xC0\x81\x01Qa\x01\xA0\x84\x01Ra5ba\x01\xE0\x82\x01a4\x01V[a\x01\xC0\x84\x01Ra\x02\0\x81\x01Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xE3Wa+;\x92` \x80\x92\x01\x92\x01\x01a4\x0CV[\x90a\x01\xE0\x82\x01a7%a7\x1Ba%\xBDa\x01\xC0a5\xD5a5\xC0a5\xBBa5\xB5\x88QQa%WV[`\x05\x1C\x90V[a%eV[\x90`@Q\x82\x81\x93\x82R`\x01\x01`\x05\x1B\x01`@RV[` \x80\x82\x01R\x96a5\xFCa5\xF3a5\xED\x83Q`\xFF\x16\x90V[`\xFF\x16\x90V[`@\x8A\x01R\x88\x90V[Pa6\x1Da6\x14a!\xD1` \x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[``\x8A\x01R\x88\x90V[Pa6>a65a!\xD1`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[`\x80\x8A\x01R\x88\x90V[P``\x81\x01Q`\xA0\x89\x01R`\x80\x81\x01Q`\xC0\x89\x01R`\xA0\x81\x01Q`\xE0\x89\x01R`\xC0\x81\x01Qa\x01\0\x89\x01R`\xE0\x81\x01Qa\x01 \x89\x01Ra\x01\0\x81\x01Qa\x01@\x89\x01Ra6\xA5a6\x9Ba%\xBDa\x01 \x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01`\x8A\x01R\x88\x90V[Pa6\xCCa6\xC2a%\xBDa\x01@\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\x80\x8A\x01R\x88\x90V[Pa6\xF3a6\xE9a%\xBDa\x01`\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\xA0\x8A\x01R\x88\x90V[Pa\x01\x80\x81\x01Qa\x01\xC0\x89\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x89\x01R\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\0\x85\x01R\x83\x90V[Pa\x02\0a\x02 \x84\x01R\x80QQa\x02@\x84\x01RQ\x80Q\x80a7jW[PP\x81Q`\x05\x1B` \x83\x01 a\x02\x95\x90\x92\x80Q`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x87\x01\x94\x01\x01\x90[\x81\x81\x10a7\xEAWPP`\x1F\x16\x90\x81\x15a7AW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a7AV[\x80Q\x84R` \x93\x84\x01\x93\x01a7{V[\x91a8\x0C\x82_R_` R`@_ \x90V[\x80T\x90`\x02`\xA0\x83\x90\x1C`\xFF\x16a8\"\x81a\x07\xCBV[\x14a9AW\x91a8\xA6\x91\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x93a8W\x87a;PV[\x91a8\x82a8h`\x80\x8A\x01Qa<;V[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\0\x89\x01Q\x80\x86\x10\x15a9;WP\x84\x90[a8\x9E\x82\x87a\x1D\xE9V[\x96\x87\x94a!eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x99\x86\x8B\x15a9\x08WPP\x90a8\xD3\x91\x85\x8Aa<\x87V[\x81a8\xF6W[PPP[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01a&\x98V[a9\0\x92\x87a<\x87V[_\x82\x81a8\xD9V[\x92P\x92\x90Pa96\x94\x93Pa9/a\x01\xC0a\x01\xE0\x85\x01Q\x94\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x93\x89a%\x80V[a8\xDDV[\x90a8\x94V[\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x84\x90R`$_\xFD[_\x90\x81\x93`@Q\x93\x838\x93` \x84Q\x94\x01\x92\xF1\x92=`$=\x11a9\xA0W[\x80` \x91\x84R\x80_\x83\x86\x01>\x83\x01\x01`@RV[P`\x01\x92P`$a9\x8CV[\x80Q\x90` \x01\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x95\x92\x90\x94\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x95\x16\x90\x81\x81\x10_\x14a;7WP\x92[_\x93\x85\x81\x10a:}W[PPPa:l\x92Pa%sV[\x81\x81\x11\x15a:xWP\x90V[\x90P\x90V[\x90\x91\x92\x93P\x84\x03\x93\x84\x11a\x15\x11W\x80\x15\x80\x15\x90\x81a:\xD2W[P\x93a:\xBA\x92\x91a:l\x95_\x14a:\xCBWPP\x80[\x80\x82\x11a:\xC3W[P\x84aA\x92V[\x90_\x80\x80a:_V[\x90P_a:\xB3V[\x02\x90a:\xABV[\x90P\x91\x90\x91a;\nW\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x90\x04\x84\x11a:\xBAa:\x96V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90P\x92a:UV[\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x80\x15a\r\x05W``a;\xEC\x91`\xC0\x93_\x91a\r\xBBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x03a<\x10WP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80`\xA0\x1Ca<\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x83\x15a=\xBBW`@Q\x92_\x80` \x86\x01\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x96\x87`$\x82\x01R\x88`D\x82\x01R`D\x81Ra<\xEE`d\x82a\x1A\xC9V[Q\x90\x82\x86Z\xF1a<\xFCa%\x1AV[\x81a=\x8CW[Pa=\x85W\x81a=j\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x93a'\x07s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[a=u\x87\x82Ta%sV[\x90U`@Q\x95\x86R\x16\x93` \x90\xA4V[PPPPPV[\x80Q\x80\x15\x92P\x82\x15a=\xA1W[PP_a=\x02V[a=\xB4\x92P` \x80\x91\x83\x01\x01\x91\x01a\x1DBV[_\x80a=\x99V[PPPPV[\x90_\x91`$\x81Q\x14a=\xD0WPV[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x14a>&WPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`\xA0\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x90\x81\x15a\r\x05W_\x91a?1W[Pa>\xCDa\x0C\x1B\x82Q\x15\x15\x90V[\x90\x81\x15a?\x0EW[Pa>\xDDWPV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90P_a>\xD5V[a?J\x91P`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[_a>\xBFV[\x99\x97\x95\x93\x91\x98\x96\x94\x92\x90\x98`@Q\x99` \x8B\x01\x9B\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x8DR`\xFF\x16`@\x8C\x01Rc\xFF\xFF\xFF\xFF\x16``\x8B\x01R`\x80\x8A\x01R`\xA0\x89\x01R`\xC0\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x82\x01Ra\x01\x80\x81Ra?\xF1a\x01\xA0\x82a\x1A\xC9V[Q\x90 \x90V[\x95\x93\x90a@\xA1\x93a@\x82a\x1Ax\x99\x97\x94`@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94a@J\x8C\x82Q` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[` \x81\x81\x01Q\x8D\x84\x01R\x91\x01Q``\x8C\x01R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8C\x01R\x01Q`\xA0\x8A\x01RV[\x16`\xC0\x87\x01R`\xE0\x86\x01Ra\x01@a\x01\0\x86\x01Ra\x01@\x85\x01\x90a\x15\x16V[\x92a\x01 \x81\x85\x03\x91\x01Ra\x1DWV[\x95\x94\x91\x98\x93\x90\x98\x97\x92\x96\x97a@\xE7`@Q\x97a@\xCD`@\x8Aa\x1A\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88RV[` \x87\x01Ra@\xF4a\x1B)V[\x95\x86R` \x86\x01R`@\x85\x01RaA\ta\x1B\x1AV[0\x81R\x94` \x86\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x01\xE3W_\x95aAb\x93`@Q\x98\x89\x97\x88\x97\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\x04\x89\x01a?\xF7V[\x03\x81\x83n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3Z\xF1\x80\x15a\r\x05WaA\x84WPV[\x80a\x19\xC3_a\x02\x95\x93a\x1A\xC9V[\x81\x81\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x81\x83\x85\x04\x14\x83\x15\x17\x02\x15aA\xBDWPPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFg\r\xE0\xB6\xB3\xA7d\0\0\x92\x84\t\x84\x81\x10\x85\x01\x90\x03\x92\t\x90\x80g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15aB8W\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x90V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x90\x81` \x91\x03\x12a\x01\xE3WQa\x1Ax\x81a\x05\xE9V[\x90` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\x04`@Q\x80\x95\x81\x93\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x91\x82\x15a\r\x05Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92` \x91_\x91aC\x87W[P`\x04`@Q\x80\x95\x81\x93\x7F\x8D68\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x91\x82\x15a\r\x05W_\x92aCVW[Pc\xFF\xFF\xFF\xFF\x81\x16c\xFF\xFF\xFF\xFF\x83\x16\x03aC\x1FWPPV[\x7F\xC9\xE00\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[aCy\x91\x92P` =` \x11aC\x80W[aCq\x81\x83a\x1A\xC9V[\x81\x01\x90aBEV[\x90_aC\x07V[P=aCgV[aC\x9E\x91P\x82=\x84\x11a\r\xB4Wa\r\xA5\x81\x83a\x1A\xC9V[_aB\xC9V[a\x1Axa\x01\xE0\x91aD\xBC`@Q\x93\x84\x92` \x80\x85\x01RaC\xCA`@\x85\x01\x82Q`\xFF\x16\x90RV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16``\x85\x01R`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\x80\x85\x01R``\x81\x01Q`\xA0\x85\x01R`\x80\x81\x01Q`\xC0\x85\x01R`\xA0\x81\x01Q`\xE0\x85\x01R`\xC0\x81\x01Qa\x01\0\x85\x01R`\xE0\x81\x01Qa\x01 \x85\x01Ra\x01\0\x81\x01Qa\x01@\x85\x01RaDGa\x01 \x82\x01Qa\x01`\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a\x01@\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x85\x01Ra\x01`\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01RaD\xA6a\x01\xC0\x82\x01Qa\x02\0\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a\x15\x16V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x1A\xC9V[\x94\x90c\xFF\xFF\xFF\xFF\x94a\x1Ax\x99\x98\x94\x86a\x01\0\x99\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x8AR\x16` \x89\x01R`@\x88\x01R\x16``\x86\x01R`\x80\x85\x01R`\xA0\x84\x01R\x16`\xC0\x82\x01R\x81`\xE0\x82\x01R\x01\x90a\x15\x16V[\x95\x90\x91\x92\x93\x95\x80`\x14R\x83`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x87Z\xF1\x80`\x01_Q\x14\x16\x15aE\xF1W[P_`4Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x83;\x15a\x01\xE3WaE\xE0_\x96\x92\x87\x93`@Q\x99\x8A\x98\x89\x97\x88\x96\x7Fw\x9BC-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R0\x920\x91`\x04\x8A\x01aD\xE8V[\x03\x92Z\xF1\x80\x15a\r\x05WaA\x84WPV[=\x84;\x15\x17\x10\x15aF\x03W[_aE{V[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x87Z\xF1P\x83`4R` _`D`\x10\x82\x87Z\xF1\x80`\x01_Q\x14\x16\x15aFGW[PaE\xFDV[=\x84;\x15\x17\x10\x15aFXW_aFAV[c>?\x8Fs_R`\x04`\x1C\xFD\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806316c38b3c146101d45780631e83409a146101cf57806325692962146101ca57806331eee44d146101c557806334236a39146101c057806339c33215146101bb57806354d1f13d146101b65780635778472a146101b15780635c975abb146101ac5780635fdc7c12146101a7578063617c537c146101a25780636afdd8501461019d578063715018a614610198578063776ff3c71461019357806377839a9e1461018e57806379502c551461018957806385c17830146101845780638cda96de1461017f5780638da5cb5b1461017a57806397c36bae14610175578063ac9650d814610170578063cc6eec701461016b578063d4570c1c14610166578063e3bb93e814610161578063f04e283e1461015c578063f2fde38b14610157578063f3995c67146101525763fee81cf41461014d575f80fd5b6119c9565b6118e0565b61186d565b6117fc565b61176a565b6116d1565b611697565b611559565b611462565b6113f2565b611379565b61133e565b6112d0565b611296565b61101b565b610f7e565b610f37565b610dda565b610968565b6108f7565b610848565b610769565b61072e565b610662565b610485565b61040f565b610297565b6101e7565b801515036101e357565b5f80fd5b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357600435610222816101d9565b61022a611f4f565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff000000000000000060035492151560401b169116176003555f80f35b73ffffffffffffffffffffffffffffffffffffffff8116036101e357565b610124359061029582610269565b565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356102d281610269565b688000000000ab143c065c6103f8576001688000000000ab143c065d335f5260016020526103218160405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5480156103d0576103cc91335f5260016020525f61036082604083209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5561036c823383611f86565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd399268360203392a35f688000000000ab143c065d6040519081529081906020820190565b0390f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b5f9103126101e357565b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b90816102009103126101e35790565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e3576104d4903690600401610476565b688000000000ab143c065c6103f8576001688000000000ab143c065d60ff60035460401c166105c1576101c06105786105717fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d6105a26103cc9561053833826121ae565b98939992918a98829661054d843033886123cf565b61055b6101e0820182611a1d565b96909101359561056a8761061c565b3691611b72565b9289612580565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d6040519081529081906020820190565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ffffffff8116036101e357565b60043590610295826105e9565b60843590610295826105e9565b3590610295826105e9565b67ffffffffffffffff8116036101e357565b60a435906102958261061c565b35906102958261061c565b908160409103126101e35790565b908160609103126101e35790565b346101e3576101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35761069a6105f7565b602435906044356064356106ac610604565b6106b461062e565b60c4359060e435926101043567ffffffffffffffff81116101e3576106dd903690600401610646565b946106e6610287565b96610144359967ffffffffffffffff8b116101e35761070c6107129b3690600401610654565b99611ba8565b6040805192835267ffffffffffffffff91909116602083015290f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576020604051624c4b408152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b600311156107d557565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b919091604064ffffffffff81606084019573ffffffffffffffffffffffffffffffffffffffff8151168552602081015161083b816107cb565b6020860152015116910152565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004355f6040805161088881611aa8565b82815282602082015201525f525f6020526103cc60405f2064ffffffffff604051916108b383611aa8565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c166108dd816107cb565b602084015260a81c16604082015260405191829182610802565b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357602060ff60035460401c166040519015158152f35b9181601f840112156101e35782359167ffffffffffffffff83116101e357602083818601950101116101e357565b346101e35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e3576109b790369060040161093a565b9060243567ffffffffffffffff81116101e3576109d890369060040161093a565b9290688000000000ab143c065c6103f8576001688000000000ab143c065d60ff60035460401c166105c1576040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169460a082602481895afa918215610d0557610ac4610aab610aab60806004966020955f91610dbb575b50015173ffffffffffffffffffffffffffffffffffffffff1690565b73ffffffffffffffffffffffffffffffffffffffff1690565b604051938480927f2c1219210000000000000000000000000000000000000000000000000000000082525afa8015610d0557610b52936020935f92610d8c575b505f73ffffffffffffffffffffffffffffffffffffffff6040518097819682957f57ecfd280000000000000000000000000000000000000000000000000000000084528b8d60048601611d95565b0393165af1908115610d05575f91610d5d575b5015610d3557610b7491613330565b9196959294939291610b8530610aab565b93848103610d0a5750610ba7610ba2610bfb9360a0933691611b72565b613452565b96610bb9602089015163ffffffff1690565b9060405180809581947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa908115610d05575f91610cd6575b50610c1f610c1b8251151590565b1590565b908115610cbe575b50610c8d57508103610c5f5750610c5192610c4191611de9565b90610c4b8161358f565b906137fa565b5f688000000000ab143c065d005b7fc21fa2e5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5ffd5b7f6a96659e000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6020015163ffffffff8381169116141590505f610c27565b610cf8915060a03d60a011610cfe575b610cf08183611ac9565b810190611c9c565b5f610c0d565b503d610ce6565b611d22565b7fc7286ea1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f514d840a000000000000000000000000000000000000000000000000000000005f5260045ffd5b610d7f915060203d602011610d85575b610d778183611ac9565b810190611d42565b5f610b65565b503d610d6d565b610dad919250843d8611610db4575b610da58183611ac9565b810190611d2d565b905f610b04565b503d610d9b565b610dd4915060a03d60a011610cfe57610cf08183611ac9565b5f610a8f565b346101e35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357600435610e1581610269565b60243590610e2282610269565b6044356064359260843567ffffffffffffffff81116101e357610e4990369060040161093a565b909460a43593303303610f0f57610f0096610efb9382610e6d610ecf94888b611f86565b73ffffffffffffffffffffffffffffffffffffffff604051998a967f3866c1dc000000000000000000000000000000000000000000000000000000006020890152602488015216604486015260648501526080608485015260a4840191611d57565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101855284611ac9565b61396e565b91905015610f0a57005b6139ac565b7f14d4a4e8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760206040516e22d473030f116ddee9f6b43ac78ba38152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357610faf611f4f565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a35f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755005b346101e35760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e35761106a903690600401610476565b60243561107681610269565b60443567ffffffffffffffff81116101e357611096903690600401610654565b688000000000ab143c065c6103f8576001688000000000ab143c065d60ff60035460401c166105c1576110c982846121ae565b959291849791949585610100840135823592602081013560405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152611120606082611ac9565b519020906040519261113360a085611ac9565b606a8452602084017f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c41759052604084017f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b659052606084017f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e74329052608084017f353620616d6f756e7429000000000000000000000000000000000000000000009052604081016111e091611a1d565b956111ee9791958c8c6140b0565b846111fd6101e0840184611a1d565b936101c00161120b90611a6e565b93369061121792611b72565b916112229488612580565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a35f688000000000ab143c065d604051908152602090f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760206040516107d08152f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576020600254604051908152f35b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356113b3611f4f565b670de0b6b3a764000081116113c757600255005b7fad6bb6d1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b346101e35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e3576114b1903690600401610476565b6114f560243561010083013592610140810135916114ce8361061c565b610160820135906114de8261061c565b610180830135916101a06002549401359487613a35565b9081810390811161151157604080519182526020820192909252f35b611dbc565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043567ffffffffffffffff81116101e357366023820112156101e357806004013567ffffffffffffffff81116101e3578060051b36602482850101116101e357346101e3576040519283926020845280846020015260408481019380602485018637850101928391611602575b505050806040520360401b17613b3f565b919350915b80915f6044825187016024810135918291018537389084305af41561168e57602067ffffffffffffffe0603f5f937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08a87030181528301943d81523d858583013e3d010116933d0101528483821015611681575090611607565b93505090505f80806115f1565b853d5f823e3d90fd5b346101e3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760206040516103e88152f35b346101e35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357602061176160043561171181610269565b73ffffffffffffffffffffffffffffffffffffffff6024359161173383610269565b165f526001835260405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b54604051908152f35b346101e3576101207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356117a6816105e9565b602435906044356064356084356117bc816105e9565b60a435906117c98261061c565b60c4359260e43594610104359767ffffffffffffffff89116101e3576117f6610712993690600401610646565b97611df6565b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043561183281610269565b61183a611f4f565b63389a75e1600c52805f526020600c209081544211611860575f61185e92556139b4565b005b636f5e88185f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e3576004356118a381610269565b6118ab611f4f565b8060601b156118bd5761185e906139b4565b637448fbae5f526004601cfd5b60ff8116036101e357565b3590610295826118ca565b346101e35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e35760043561191b81610269565b6024356044356064359261192e846118ca565b73ffffffffffffffffffffffffffffffffffffffff169160a43590608435843b156101e3575f9460e493869260ff604051998a9889977fd505accf0000000000000000000000000000000000000000000000000000000089523360048a01523060248a01526044890152606488015216608486015260a485015260c48401525af16119b557005b806119c35f61185e93611ac9565b80610405565b346101e35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e357600435611a0481610269565b63389a75e1600c525f52602080600c2054604051908152f35b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101e3570180359067ffffffffffffffff82116101e3576020019181360383136101e357565b35611a788161061c565b90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6060810190811067ffffffffffffffff821117611ac457604052565b611a7b565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117611ac457604052565b6040519061029561020083611ac9565b60405190610295604083611ac9565b60405190610295606083611ac9565b67ffffffffffffffff8111611ac457601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b929192611b7e82611b38565b91611b8c6040519384611ac9565b8294818452818301116101e3578281602093845f960137010152565b9296919a999498959093979860ff60035460401c166105c1578b881015611c615787611bd2612843565b9c611bdf968e968a61290b565b948035611beb8161061c565b67ffffffffffffffff166101c087015260208101611c0891611a1d565b3690611c1392611b72565b6101e0860152611c2285612bbf565b604080516020810185815263ffffffff871682840152918152611c46606082611ac9565b51902090611c549286612f74565b611c5d9261304c565b9190565b8b887fed2bc1ea000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5190610295826105e9565b908160a09103126101e3576040519060a082019082821067ffffffffffffffff831117611ac4576080916040528051611cd4816101d9565b83526020810151611ce4816105e9565b60208401526040810151611cf7816105e9565b60408401526060810151611d0a81610269565b60608401520151611d1a81610269565b608082015290565b6040513d5f823e3d90fd5b908160209103126101e35751611a7881610269565b908160209103126101e35751611a78816101d9565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b9290611dae90611a789593604086526040860191611d57565b926020818503910152611d57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9190820391821161151157565b929498979395909896919660ff60035460401c166105c15787861015611f1d579261056a92611e3692611e5f95888b611e2d612843565b9e8f963361290b565b92611e55611e4382611a6e565b67ffffffffffffffff166101c0860152565b6020810190611a1d565b6101e0820152611e6e81612bbf565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a0816024817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa948515610d0557611f0f6060611c5d97611f18945f91610dbb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b309033906123cf565b61304c565b7fed2bc1ea000000000000000000000000000000000000000000000000000000005f526004869052602488905260445ffd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927543303611f7957565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f51141615611fc2575b50505f603452565b3b153d171015611fd3575f80611fba565b6390b8ec185f526004601cfd5b35611a78816118ca565b35611a78816105e9565b9080601f830112156101e357816020611a7893359101611b72565b919091610200818403126101e357612025611b0a565b9261202f826118d5565b845261203d60208301610611565b602085015261204e60408301610611565b6040850152606082013560608501526080820135608085015260a082013560a085015260c082013560c085015260e082013560e085015261010082013561010085015261209e610120830161063b565b6101208501526120b1610140830161063b565b6101408501526120c4610160830161063b565b6101608501526101808201356101808501526101a08201356101a08501526120ef6101c0830161063b565b6101c08501526101e082013567ffffffffffffffff81116101e3576121149201611ff4565b6101e0830152565b61212660016107cb565b740100000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b61216f60026107cb565b740200000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b91909160ff6121bc82611fe0565b1661239057604081016121da6121d182611fea565b63ffffffff1690565b460361235357506121f36121ee368361200f565b61358f565b92612205845f525f60205260405f2090565b91612215835460ff9060a01c1690565b61221e816107cb565b6123265761223461222f368361200f565b613b50565b936102956122456080840135613c3b565b946122d461229261228b610100870135966122636101408201611a6e565b906122716101608201611a6e565b91610180820135906101a06002549301359342918c613a35565b8096611de9565b95829073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b6122dd8161211c565b80547fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff164260a81b79ffffffffff00000000000000000000000000000000000000000016179055565b7f343e211e000000000000000000000000000000000000000000000000000000005f52600485905260245ffd5b61235f610c8a91611fea565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5263ffffffff16600452602490565b61239c610c8a91611fe0565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f90815260045260ff16602452604490565b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f51141615612418575b50505f606052604052565b3b153d171015612429575f8061240d565b637939f4245f526004601cfd5b67ffffffffffffffff61c3509116019067ffffffffffffffff821161151157565b9067ffffffffffffffff8091169116019067ffffffffffffffff821161151157565b94909367ffffffffffffffff9373ffffffffffffffffffffffffffffffffffffffff6124ca948160a0989b9a9b1689521660208801526040870152606086015260c0608086015260c0850190611516565b9416910152565b905f60208301926124e1826107cb565b52565b90600260208301926124e1826107cb565b90600160208301926124e1826107cb565b60405190612515602083611ac9565b5f8252565b3d15612544573d9061252b82611b38565b916125396040519384611ac9565b82523d5f602084013e565b606090565b906020820180921161151157565b90601f820180921161151157565b601201908160121161151157565b9190820180921161151157565b9394909194831561283b578051158015612832575b612825575a6125ca6125bd6125b8600586901c6707ffffffffffffff1686612457565b612436565b67ffffffffffffffff1690565b116127e857303b156101e357612614915f9160405193849283927f617c537c00000000000000000000000000000000000000000000000000000000845289898c8a60048801612479565b038183305af190816127d4575b5061278c5761263661263161251a565b613dc1565b9073ffffffffffffffffffffffffffffffffffffffff82169485151580612782575b1561269d57509061266a929184613c87565b7f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a960405180612698816124f5565b0390a3565b73ffffffffffffffffffffffffffffffffffffffff8096508293508161272a87946127077f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9573ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b9073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b612735878254612573565b9055169586936127516040519283921696829190602083019252565b0390a47f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a960405180612698816124e4565b5030861415612658565b50507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a973ffffffffffffffffffffffffffffffffffffffff60405193169280612698816124d1565b806119c35f6127e293611ac9565b5f612621565b610c8a825a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045267ffffffffffffffff16602452604490565b5050610295939192613c87565b50853b15612595565b505050505050565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff81116115115767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b6128a2611b0a565b905f82525f60208301525f60408301525f60608301525f60808301525f60a08301525f60c08301525f60e08301525f6101008301525f6101208301525f6101408301525f6101608301525f6101808301525f6101a08301525f6101c083015260606101e0830152565b94939792909761291961289a565b506040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016969060a0816024818b5afa8015610d05578b6129bd606060a0936129ff955f91612ba85750015173ffffffffffffffffffffffffffffffffffffffff1690565b9960405180809581947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa908115610d0557612a33916060915f91610dbb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b9073ffffffffffffffffffffffffffffffffffffffff881615612b765773ffffffffffffffffffffffffffffffffffffffff821615612b445773ffffffffffffffffffffffffffffffffffffffff9081169791811692911690612a969084611de9565b934267ffffffffffffffff1696612aad9088612457565b97612ab6611b0a565b5f815263ffffffff461660208201529b63ffffffff1660408d015260608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015267ffffffffffffffff1661014085015267ffffffffffffffff166101608401526101808301526101a08201525f6101c0820152612b3b612506565b6101e082015290565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff8b1660045260245ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f524663ffffffff1660045260245ffd5b610dd49150853d8711610cfe57610cf08183611ac9565b602081015163ffffffff16468103612d8b5750612beb612be6604083015163ffffffff1690565b613e40565b61016081015161014082015167ffffffffffffffff918216911680821115612d505750506101008101805180158015612d43575b612d0d57506101a082015190519081811015612cdf57505073ffffffffffffffffffffffffffffffffffffffff612c596080830151613c3b565b1615612cb7576101c0015167ffffffffffffffff16624c4b408111612c7b5750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff16600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e08301517f8dd09d91000000000000000000000000000000000000000000000000000000005f5260049190915260245260445ffd5b5060e08301518111612c1f565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff9081166004521660245260445ffd5b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b9067ffffffffffffffff8091169116039067ffffffffffffffff821161151157565b60405190612dee61016083611ac9565b61012982527f3620616d6f756e74290000000000000000000000000000000000000000000000610140837f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208201527f75696e743820627269646765547970652c75696e74333220647374436861696e60408201527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608201527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808201527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08201527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08201527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08201527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008201527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208201520152565b9161029593919261302360a061301b612f8e845160ff1690565b84612fa0604082015163ffffffff1690565b9760808201519860e08301998a51610100850151612fe3612fcd61016088015167ffffffffffffffff1690565b61014088015167ffffffffffffffff1690612dbc565b91610180870151936101a0880151956130156101c06101e08b0151602081519101209a015167ffffffffffffffff1690565b99613f50565b920151613c3b565b9251823560208401359180613044613039612dde565b966040810190611a1d565b9890976140b0565b909291926130598261358f565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015290949073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001660a082602481845afa918215610d05575f926132ba575b508192608061316a930193613113613102865173ffffffffffffffffffffffffffffffffffffffff1690565b602083015163ffffffff169061425a565b60a06040880193613128855163ffffffff1690565b9060405180809881947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa918215610d055789967f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a9661320a60606131ee6131d3602061321f9973ffffffffffffffffffffffffffffffffffffffff9c5f9161329b575b50015163ffffffff1690565b935173ffffffffffffffffffffffffffffffffffffffff1690565b95015173ffffffffffffffffffffffffffffffffffffffff1690565b60e08b0151906132198c6143a4565b95614542565b61269861323d6132326060880151613c3b565b925163ffffffff1690565b9560c08101519061326061012061010083015192015167ffffffffffffffff1690565b604080515f815263ffffffff909a1660208b0152890192909252606088015267ffffffffffffffff16608087015291169390819060a0820190565b6132b4915060a03d60a011610cfe57610cf08183611ac9565b5f6131c7565b61316a92506132d79060a03d60a011610cfe57610cf08183611ac9565b916130d6565b909392938483116101e35784116101e3578101920390565b359060208110613303575090565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9060200360031b1b1690565b61017882106133ca57816008116101e357600481013560e01c9261336961336361335a60b8612549565b60b886866132dd565b906132f5565b9161338361336361337a60d8612549565b60d887856132dd565b916133c66133a061336361339760f8612549565b60f889876132dd565b956133bb6133636101386133b381612549565b9084886132dd565b9381610178916132dd565b9091565b507fa2abf1b6000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b5190610295826118ca565b51906102958261061c565b81601f820112156101e35780519061342382611b38565b926134316040519485611ac9565b828452602083830101116101e357815f9260208093018386015e8301015290565b61345a61289a565b5080518101906020818303126101e35760208101519067ffffffffffffffff82116101e3570190610200828203126101e357613494611b0a565b916134a1602082016133f6565b83526134af60408201611c91565b60208401526134c060608201611c91565b60408401526080810151606084015260a0810151608084015260c081015160a084015260e081015160c084015261010081015160e08401526101208101516101008401526135116101408201613401565b6101208401526135246101608201613401565b6101408401526135376101808201613401565b6101608401526101a08101516101808401526101c08101516101a08401526135626101e08201613401565b6101c08401526102008101519167ffffffffffffffff83116101e357612b3b92602080920192010161340c565b906101e0820161372561371b6125bd6101c06135d56135c06135bb6135b5885151612557565b60051c90565b612565565b90604051828193825260010160051b01604052565b602080820152966135fc6135f36135ed835160ff1690565b60ff1690565b60408a01528890565b5061361d6136146121d1602084015163ffffffff1690565b60608a01528890565b5061363e6136356121d1604084015163ffffffff1690565b60808a01528890565b50606081015160a0890152608081015160c089015260a081015160e089015260c081015161010089015260e08101516101208901526101008101516101408901526136a561369b6125bd61012084015167ffffffffffffffff1690565b6101608a01528890565b506136cc6136c26125bd61014084015167ffffffffffffffff1690565b6101808a01528890565b506136f36136e96125bd61016084015167ffffffffffffffff1690565b6101a08a01528890565b506101808101516101c08901526101a08101516101e0890152015167ffffffffffffffff1690565b6102008501528390565b506102006102208401528051516102408401525180518061376a575b5050815160051b602083012061029590928051604051828260010160051b011490151060061b52565b602082016020826102608701940101905b8181106137ea575050601f169081156137415760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80613741565b805184526020938401930161377b565b9161380c825f525f60205260405f2090565b805490600260a083901c60ff16613822816107cb565b1461394157916138a6917f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a9361385787613b50565b9161388261386860808a0151613c3b565b9173ffffffffffffffffffffffffffffffffffffffff1690565b6101008901518086101561393b575084905b61389e8287611de9565b968794612165565b73ffffffffffffffffffffffffffffffffffffffff811699868b15613908575050906138d391858a613c87565b816138f6575b5050505b6040805191825260208201929092529081908101612698565b6139009287613c87565b5f82816138d9565b925092905061393694935061392f6101c06101e085015194015167ffffffffffffffff1690565b9389612580565b6138dd565b90613894565b7fb196a44a000000000000000000000000000000000000000000000000000000005f52600484905260245ffd5b5f9081936040519383389360208451940192f1923d60243d116139a0575b806020918452805f8386013e830101604052565b5060019250602461398c565b805190602001fd5b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b959290949167ffffffffffffffff8091169516908181105f14613b375750925b5f93858110613a7d575b505050613a6c9250612573565b81811115613a78575090565b905090565b9091929350840393841161151157801580159081613ad2575b5093613aba9291613a6c955f14613acb575050805b808211613ac3575b5084614192565b905f8080613a5f565b90505f613ab3565b0290613aab565b9050919091613b0a57907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8190048411613aba613a96565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b905092613a55565b8060401c9067ffffffffffffffff16f35b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a0816024817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa8015610d05576060613bec9160c0935f91610dbb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b91015173ffffffffffffffffffffffffffffffffffffffff82168103613c10575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8060a01c613c5c5773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b908315613dbb57604051925f80602086017fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84169687602482015288604482015260448152613cee606482611ac9565b519082865af1613cfc61251a565b81613d8c575b50613d855781613d6a7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9361270773ffffffffffffffffffffffffffffffffffffffff9473ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b613d75878254612573565b90556040519586521693602090a4565b5050505050565b8051801592508215613da1575b50505f613d02565b613db49250602080918301019101611d42565b5f80613d99565b50505050565b905f916024815114613dd05750565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff0000000000000000000000000000000000000000000000000000000060246020840151930151921614613e265750565b73ffffffffffffffffffffffffffffffffffffffff169150565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815263ffffffff8216600482015260a0816024817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa908115610d05575f91613f31575b50613ecd610c1b8251151590565b908115613f0e575b50613edd5750565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6060015173ffffffffffffffffffffffffffffffffffffffff161590505f613ed5565b613f4a915060a03d60a011610cfe57610cf08183611ac9565b5f613ebf565b99979593919896949290986040519960208b019b7f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b8d5260ff1660408c015263ffffffff1660608b015260808a015260a089015260c088015267ffffffffffffffff1660e087015261010086015261012085015261014084015261016083015267ffffffffffffffff166101808201526101808152613ff16101a082611ac9565b51902090565b9593906140a193614082611a78999794604073ffffffffffffffffffffffffffffffffffffffff9461404a8c82516020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b6020818101518d84015291015160608c0152815173ffffffffffffffffffffffffffffffffffffffff1660808c0152015160a08a0152565b1660c087015260e0860152610140610100860152610140850190611516565b92610120818503910152611d57565b95949198939098979296976140e7604051976140cd60408a611ac9565b73ffffffffffffffffffffffffffffffffffffffff168852565b60208701526140f4611b29565b95865260208601526040850152614109611b1a565b3081529460208601526e22d473030f116ddee9f6b43ac78ba33b156101e3575f956141629360405198899788977f137c29fe00000000000000000000000000000000000000000000000000000000895260048901613ff7565b0381836e22d473030f116ddee9f6b43ac78ba35af18015610d05576141845750565b806119c35f61029593611ac9565b81810291670de0b6b3a7640000818385041483151702156141bd575050670de0b6b3a7640000900490565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff670de0b6b3a76400009284098481108501900392099080670de0b6b3a7640000111561423857828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690290565b63ae47f7025f526004601cfd5b908160209103126101e35751611a78816105e9565b90602073ffffffffffffffffffffffffffffffffffffffff926004604051809581937f2c121921000000000000000000000000000000000000000000000000000000008352165afa918215610d055773ffffffffffffffffffffffffffffffffffffffff926020915f91614387575b506004604051809581937f8d3638f4000000000000000000000000000000000000000000000000000000008352165afa918215610d05575f92614356575b5063ffffffff811663ffffffff83160361431f575050565b7fc9e030e8000000000000000000000000000000000000000000000000000000005f5263ffffffff9081166004521660245260445ffd5b61437991925060203d602011614380575b6143718183611ac9565b810190614245565b905f614307565b503d614367565b61439e9150823d8411610db457610da58183611ac9565b5f6142c9565b611a786101e0916144bc6040519384926020808501526143ca60408501825160ff169052565b602081015163ffffffff166060850152604081015163ffffffff166080850152606081015160a0850152608081015160c085015260a081015160e085015260c081015161010085015260e081015161012085015261010081015161014085015261444761012082015161016086019067ffffffffffffffff169052565b61014081015167ffffffffffffffff1661018085015261016081015167ffffffffffffffff166101a08501526101808101516101c08501526101a0810151828501526144a66101c082015161020086019067ffffffffffffffff169052565b0151610200610220840152610240830190611516565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282611ac9565b949063ffffffff94611a7899989486610100999573ffffffffffffffffffffffffffffffffffffffff948a521660208901526040880152166060860152608085015260a08401521660c08201528160e08201520190611516565b95909192939580601452836034526f095ea7b30000000000000000000000005f5260205f6044601082875af18060015f511416156145f1575b505f60345273ffffffffffffffffffffffffffffffffffffffff1692833b156101e3576145e05f96928793604051998a98899788967f779b432d0000000000000000000000000000000000000000000000000000000088523092309160048a016144e8565b03925af18015610d05576141845750565b3d843b15171015614603575b5f61457b565b5f6034526f095ea7b30000000000000000000000005f525f386044601083875af1508360345260205f6044601082875af18060015f51141615614647575b506145fd565b3d843b15171015614658575f614641565b633e3f8f735f526004601cfdfea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x16\xC3\x8B<\x14a\x01\xD4W\x80c\x1E\x83@\x9A\x14a\x01\xCFW\x80c%i)b\x14a\x01\xCAW\x80c1\xEE\xE4M\x14a\x01\xC5W\x80c4#j9\x14a\x01\xC0W\x80c9\xC32\x15\x14a\x01\xBBW\x80cT\xD1\xF1=\x14a\x01\xB6W\x80cWxG*\x14a\x01\xB1W\x80c\\\x97Z\xBB\x14a\x01\xACW\x80c_\xDC|\x12\x14a\x01\xA7W\x80ca|S|\x14a\x01\xA2W\x80cj\xFD\xD8P\x14a\x01\x9DW\x80cqP\x18\xA6\x14a\x01\x98W\x80cwo\xF3\xC7\x14a\x01\x93W\x80cw\x83\x9A\x9E\x14a\x01\x8EW\x80cyP,U\x14a\x01\x89W\x80c\x85\xC1x0\x14a\x01\x84W\x80c\x8C\xDA\x96\xDE\x14a\x01\x7FW\x80c\x8D\xA5\xCB[\x14a\x01zW\x80c\x97\xC3k\xAE\x14a\x01uW\x80c\xAC\x96P\xD8\x14a\x01pW\x80c\xCCn\xECp\x14a\x01kW\x80c\xD4W\x0C\x1C\x14a\x01fW\x80c\xE3\xBB\x93\xE8\x14a\x01aW\x80c\xF0N(>\x14a\x01\\W\x80c\xF2\xFD\xE3\x8B\x14a\x01WW\x80c\xF3\x99\\g\x14a\x01RWc\xFE\xE8\x1C\xF4\x14a\x01MW_\x80\xFD[a\x19\xC9V[a\x18\xE0V[a\x18mV[a\x17\xFCV[a\x17jV[a\x16\xD1V[a\x16\x97V[a\x15YV[a\x14bV[a\x13\xF2V[a\x13yV[a\x13>V[a\x12\xD0V[a\x12\x96V[a\x10\x1BV[a\x0F~V[a\x0F7V[a\r\xDAV[a\thV[a\x08\xF7V[a\x08HV[a\x07iV[a\x07.V[a\x06bV[a\x04\x85V[a\x04\x0FV[a\x02\x97V[a\x01\xE7V[\x80\x15\x15\x03a\x01\xE3WV[_\x80\xFD[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x02\"\x81a\x01\xD9V[a\x02*a\x1FOV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92\x15\x15`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xE3WV[a\x01$5\x90a\x02\x95\x82a\x02iV[V[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x02\xD2\x81a\x02iV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R`\x01` Ra\x03!\x81`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T\x80\x15a\x03\xD0Wa\x03\xCC\x913_R`\x01` R_a\x03`\x82`@\x83 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[Ua\x03l\x823\x83a\x1F\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83` 3\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[_\x91\x03\x12a\x01\xE3WV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[\x90\x81a\x02\0\x91\x03\x12a\x01\xE3W\x90V[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x04\xD4\x906\x90`\x04\x01a\x04vV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xC1Wa\x01\xC0a\x05xa\x05q\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x05\xA2a\x03\xCC\x95a\x0583\x82a!\xAEV[\x98\x93\x99\x92\x91\x8A\x98\x82\x96a\x05M\x8403\x88a#\xCFV[a\x05[a\x01\xE0\x82\x01\x82a\x1A\x1DV[\x96\x90\x91\x015\x95a\x05j\x87a\x06\x1CV[6\x91a\x1BrV[\x92\x89a%\x80V[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xE3WV[`\x045\x90a\x02\x95\x82a\x05\xE9V[`\x845\x90a\x02\x95\x82a\x05\xE9V[5\x90a\x02\x95\x82a\x05\xE9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xE3WV[`\xA45\x90a\x02\x95\x82a\x06\x1CV[5\x90a\x02\x95\x82a\x06\x1CV[\x90\x81`@\x91\x03\x12a\x01\xE3W\x90V[\x90\x81``\x91\x03\x12a\x01\xE3W\x90V[4a\x01\xE3Wa\x01`\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wa\x06\x9Aa\x05\xF7V[`$5\x90`D5`d5a\x06\xACa\x06\x04V[a\x06\xB4a\x06.V[`\xC45\x90`\xE45\x92a\x01\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x06\xDD\x906\x90`\x04\x01a\x06FV[\x94a\x06\xE6a\x02\x87V[\x96a\x01D5\x99g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x11a\x01\xE3Wa\x07\x0Ca\x07\x12\x9B6\x90`\x04\x01a\x06TV[\x99a\x1B\xA8V[`@\x80Q\x92\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R\x90\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@QbLK@\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[`\x03\x11\x15a\x07\xD5WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x91`@d\xFF\xFF\xFF\xFF\xFF\x81``\x84\x01\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R` \x81\x01Qa\x08;\x81a\x07\xCBV[` \x86\x01R\x01Q\x16\x91\x01RV[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045_`@\x80Qa\x08\x88\x81a\x1A\xA8V[\x82\x81R\x82` \x82\x01R\x01R_R_` Ra\x03\xCC`@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x08\xB3\x83a\x1A\xA8V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16a\x08\xDD\x81a\x07\xCBV[` \x84\x01R`\xA8\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82a\x08\x02V[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xE3W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xE3W` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xE3WV[4a\x01\xE3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\t\xB7\x906\x90`\x04\x01a\t:V[\x90`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\t\xD8\x906\x90`\x04\x01a\t:V[\x92\x90h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xC1W`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94`\xA0\x82`$\x81\x89Z\xFA\x91\x82\x15a\r\x05Wa\n\xC4a\n\xABa\n\xAB`\x80`\x04\x96` \x95_\x91a\r\xBBW[P\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x93\x84\x80\x92\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82RZ\xFA\x80\x15a\r\x05Wa\x0BR\x93` \x93_\x92a\r\x8CW[P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x80\x97\x81\x96\x82\x95\x7FW\xEC\xFD(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x8B\x8D`\x04\x86\x01a\x1D\x95V[\x03\x93\x16Z\xF1\x90\x81\x15a\r\x05W_\x91a\r]W[P\x15a\r5Wa\x0Bt\x91a30V[\x91\x96\x95\x92\x94\x93\x92\x91a\x0B\x850a\n\xABV[\x93\x84\x81\x03a\r\nWPa\x0B\xA7a\x0B\xA2a\x0B\xFB\x93`\xA0\x936\x91a\x1BrV[a4RV[\x96a\x0B\xB9` \x89\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x95\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\r\x05W_\x91a\x0C\xD6W[Pa\x0C\x1Fa\x0C\x1B\x82Q\x15\x15\x90V[\x15\x90V[\x90\x81\x15a\x0C\xBEW[Pa\x0C\x8DWP\x81\x03a\x0C_WPa\x0CQ\x92a\x0CA\x91a\x1D\xE9V[\x90a\x0CK\x81a5\x8FV[\x90a7\xFAV[_h\x80\0\0\0\0\xAB\x14<\x06]\0[\x7F\xC2\x1F\xA2\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[_\xFD[\x7Fj\x96e\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[` \x01Qc\xFF\xFF\xFF\xFF\x83\x81\x16\x91\x16\x14\x15\x90P_a\x0C'V[a\x0C\xF8\x91P`\xA0=`\xA0\x11a\x0C\xFEW[a\x0C\xF0\x81\x83a\x1A\xC9V[\x81\x01\x90a\x1C\x9CV[_a\x0C\rV[P=a\x0C\xE6V[a\x1D\"V[\x7F\xC7(n\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FQM\x84\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[a\r\x7F\x91P` =` \x11a\r\x85W[a\rw\x81\x83a\x1A\xC9V[\x81\x01\x90a\x1DBV[_a\x0BeV[P=a\rmV[a\r\xAD\x91\x92P\x84=\x86\x11a\r\xB4W[a\r\xA5\x81\x83a\x1A\xC9V[\x81\x01\x90a\x1D-V[\x90_a\x0B\x04V[P=a\r\x9BV[a\r\xD4\x91P`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[_a\n\x8FV[4a\x01\xE3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x0E\x15\x81a\x02iV[`$5\x90a\x0E\"\x82a\x02iV[`D5`d5\x92`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x0EI\x906\x90`\x04\x01a\t:V[\x90\x94`\xA45\x9303\x03a\x0F\x0FWa\x0F\0\x96a\x0E\xFB\x93\x82a\x0Ema\x0E\xCF\x94\x88\x8Ba\x1F\x86V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x99\x8A\x96\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x89\x01R`$\x88\x01R\x16`D\x86\x01R`d\x85\x01R`\x80`\x84\x85\x01R`\xA4\x84\x01\x91a\x1DWV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x85R\x84a\x1A\xC9V[a9nV[\x91\x90P\x15a\x0F\nW\0[a9\xACV[\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3Wa\x0F\xAFa\x1FOV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\0[4a\x01\xE3W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x10j\x906\x90`\x04\x01a\x04vV[`$5a\x10v\x81a\x02iV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x10\x96\x906\x90`\x04\x01a\x06TV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xF8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xC1Wa\x10\xC9\x82\x84a!\xAEV[\x95\x92\x91\x84\x97\x91\x94\x95\x85a\x01\0\x84\x015\x825\x92` \x81\x015`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\x11 ``\x82a\x1A\xC9V[Q\x90 \x90`@Q\x92a\x113`\xA0\x85a\x1A\xC9V[`j\x84R` \x84\x01\x7FFillAuthorization witness)FillAu\x90R`@\x84\x01\x7Fthorization(bytes32 orderId)Toke\x90R``\x84\x01\x7FnPermissions(address token,uint2\x90R`\x80\x84\x01\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90R`@\x81\x01a\x11\xE0\x91a\x1A\x1DV[\x95a\x11\xEE\x97\x91\x95\x8C\x8Ca@\xB0V[\x84a\x11\xFDa\x01\xE0\x84\x01\x84a\x1A\x1DV[\x93a\x01\xC0\x01a\x12\x0B\x90a\x1AnV[\x936\x90a\x12\x17\x92a\x1BrV[\x91a\x12\"\x94\x88a%\x80V[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qa\x07\xD0\x81R\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `\x02T`@Q\x90\x81R\xF3[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x13\xB3a\x1FOV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x13\xC7W`\x02U\0[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x01\xE3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa\x14\xB1\x906\x90`\x04\x01a\x04vV[a\x14\xF5`$5a\x01\0\x83\x015\x92a\x01@\x81\x015\x91a\x14\xCE\x83a\x06\x1CV[a\x01`\x82\x015\x90a\x14\xDE\x82a\x06\x1CV[a\x01\x80\x83\x015\x91a\x01\xA0`\x02T\x94\x015\x94\x87a:5V[\x90\x81\x81\x03\x90\x81\x11a\x15\x11W`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[a\x1D\xBCV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3W6`#\x82\x01\x12\x15a\x01\xE3W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3W\x80`\x05\x1B6`$\x82\x85\x01\x01\x11a\x01\xE3W4a\x01\xE3W`@Q\x92\x83\x92` \x84R\x80\x84` \x01R`@\x84\x81\x01\x93\x80`$\x85\x01\x867\x85\x01\x01\x92\x83\x91a\x16\x02W[PPP\x80`@R\x03`@\x1B\x17a;?V[\x91\x93P\x91[\x80\x91_`D\x82Q\x87\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x16\x8EW` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?_\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8A\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x93=\x01\x01R\x84\x83\x82\x10\x15a\x16\x81WP\x90a\x16\x07V[\x93PP\x90P_\x80\x80a\x15\xF1V[\x85=_\x82>=\x90\xFD[4a\x01\xE3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` `@Qa\x03\xE8\x81R\xF3[4a\x01\xE3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W` a\x17a`\x045a\x17\x11\x81a\x02iV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91a\x173\x83a\x02iV[\x16_R`\x01\x83R`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x01\xE3Wa\x01 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x17\xA6\x81a\x05\xE9V[`$5\x90`D5`d5`\x845a\x17\xBC\x81a\x05\xE9V[`\xA45\x90a\x17\xC9\x82a\x06\x1CV[`\xC45\x92`\xE45\x94a\x01\x045\x97g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x11a\x01\xE3Wa\x17\xF6a\x07\x12\x996\x90`\x04\x01a\x06FV[\x97a\x1D\xF6V[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x182\x81a\x02iV[a\x18:a\x1FOV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x90\x81TB\x11a\x18`W_a\x18^\x92Ua9\xB4V[\0[co^\x88\x18_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x18\xA3\x81a\x02iV[a\x18\xABa\x1FOV[\x80``\x1B\x15a\x18\xBDWa\x18^\x90a9\xB4V[ctH\xFB\xAE_R`\x04`\x1C\xFD[`\xFF\x81\x16\x03a\x01\xE3WV[5\x90a\x02\x95\x82a\x18\xCAV[4a\x01\xE3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x19\x1B\x81a\x02iV[`$5`D5`d5\x92a\x19.\x84a\x18\xCAV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91`\xA45\x90`\x845\x84;\x15a\x01\xE3W_\x94`\xE4\x93\x86\x92`\xFF`@Q\x99\x8A\x98\x89\x97\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R3`\x04\x8A\x01R0`$\x8A\x01R`D\x89\x01R`d\x88\x01R\x16`\x84\x86\x01R`\xA4\x85\x01R`\xC4\x84\x01RZ\xF1a\x19\xB5W\0[\x80a\x19\xC3_a\x18^\x93a\x1A\xC9V[\x80a\x04\x05V[4a\x01\xE3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xE3W`\x045a\x1A\x04\x81a\x02iV[c8\x9Au\xE1`\x0CR_R` \x80`\x0C T`@Q\x90\x81R\xF3[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xE3W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xE3W` \x01\x91\x816\x03\x83\x13a\x01\xE3WV[5a\x1Ax\x81a\x06\x1CV[\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1A\xC4W`@RV[a\x1A{V[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1A\xC4W`@RV[`@Q\x90a\x02\x95a\x02\0\x83a\x1A\xC9V[`@Q\x90a\x02\x95`@\x83a\x1A\xC9V[`@Q\x90a\x02\x95``\x83a\x1A\xC9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1A\xC4W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a\x1B~\x82a\x1B8V[\x91a\x1B\x8C`@Q\x93\x84a\x1A\xC9V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xE3W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x92\x96\x91\x9A\x99\x94\x98\x95\x90\x93\x97\x98`\xFF`\x03T`@\x1C\x16a\x05\xC1W\x8B\x88\x10\x15a\x1CaW\x87a\x1B\xD2a(CV[\x9Ca\x1B\xDF\x96\x8E\x96\x8Aa)\x0BV[\x94\x805a\x1B\xEB\x81a\x06\x1CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x87\x01R` \x81\x01a\x1C\x08\x91a\x1A\x1DV[6\x90a\x1C\x13\x92a\x1BrV[a\x01\xE0\x86\x01Ra\x1C\"\x85a+\xBFV[`@\x80Q` \x81\x01\x85\x81Rc\xFF\xFF\xFF\xFF\x87\x16\x82\x84\x01R\x91\x81Ra\x1CF``\x82a\x1A\xC9V[Q\x90 \x90a\x1CT\x92\x86a/tV[a\x1C]\x92a0LV[\x91\x90V[\x8B\x88\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[Q\x90a\x02\x95\x82a\x05\xE9V[\x90\x81`\xA0\x91\x03\x12a\x01\xE3W`@Q\x90`\xA0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x1A\xC4W`\x80\x91`@R\x80Qa\x1C\xD4\x81a\x01\xD9V[\x83R` \x81\x01Qa\x1C\xE4\x81a\x05\xE9V[` \x84\x01R`@\x81\x01Qa\x1C\xF7\x81a\x05\xE9V[`@\x84\x01R``\x81\x01Qa\x1D\n\x81a\x02iV[``\x84\x01R\x01Qa\x1D\x1A\x81a\x02iV[`\x80\x82\x01R\x90V[`@Q=_\x82>=\x90\xFD[\x90\x81` \x91\x03\x12a\x01\xE3WQa\x1Ax\x81a\x02iV[\x90\x81` \x91\x03\x12a\x01\xE3WQa\x1Ax\x81a\x01\xD9V[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x92\x90a\x1D\xAE\x90a\x1Ax\x95\x93`@\x86R`@\x86\x01\x91a\x1DWV[\x92` \x81\x85\x03\x91\x01Ra\x1DWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x03\x91\x82\x11a\x15\x11WV[\x92\x94\x98\x97\x93\x95\x90\x98\x96\x91\x96`\xFF`\x03T`@\x1C\x16a\x05\xC1W\x87\x86\x10\x15a\x1F\x1DW\x92a\x05j\x92a\x1E6\x92a\x1E_\x95\x88\x8Ba\x1E-a(CV[\x9E\x8F\x963a)\x0BV[\x92a\x1EUa\x1EC\x82a\x1AnV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x86\x01RV[` \x81\x01\x90a\x1A\x1DV[a\x01\xE0\x82\x01Ra\x1En\x81a+\xBFV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x94\x85\x15a\r\x05Wa\x1F\x0F``a\x1C]\x97a\x1F\x18\x94_\x91a\r\xBBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[0\x903\x90a#\xCFV[a0LV[\x7F\xED+\xC1\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x86\x90R`$\x88\x90R`D_\xFD[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a\x1FyWV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a\x1F\xC2W[PP_`4RV[;\x15=\x17\x10\x15a\x1F\xD3W_\x80a\x1F\xBAV[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5a\x1Ax\x81a\x18\xCAV[5a\x1Ax\x81a\x05\xE9V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xE3W\x81` a\x1Ax\x935\x91\x01a\x1BrV[\x91\x90\x91a\x02\0\x81\x84\x03\x12a\x01\xE3Wa %a\x1B\nV[\x92a /\x82a\x18\xD5V[\x84Ra =` \x83\x01a\x06\x11V[` \x85\x01Ra N`@\x83\x01a\x06\x11V[`@\x85\x01R``\x82\x015``\x85\x01R`\x80\x82\x015`\x80\x85\x01R`\xA0\x82\x015`\xA0\x85\x01R`\xC0\x82\x015`\xC0\x85\x01R`\xE0\x82\x015`\xE0\x85\x01Ra\x01\0\x82\x015a\x01\0\x85\x01Ra \x9Ea\x01 \x83\x01a\x06;V[a\x01 \x85\x01Ra \xB1a\x01@\x83\x01a\x06;V[a\x01@\x85\x01Ra \xC4a\x01`\x83\x01a\x06;V[a\x01`\x85\x01Ra\x01\x80\x82\x015a\x01\x80\x85\x01Ra\x01\xA0\x82\x015a\x01\xA0\x85\x01Ra \xEFa\x01\xC0\x83\x01a\x06;V[a\x01\xC0\x85\x01Ra\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xE3Wa!\x14\x92\x01a\x1F\xF4V[a\x01\xE0\x83\x01RV[a!&`\x01a\x07\xCBV[t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[a!o`\x02a\x07\xCBV[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[\x91\x90\x91`\xFFa!\xBC\x82a\x1F\xE0V[\x16a#\x90W`@\x81\x01a!\xDAa!\xD1\x82a\x1F\xEAV[c\xFF\xFF\xFF\xFF\x16\x90V[F\x03a#SWPa!\xF3a!\xEE6\x83a \x0FV[a5\x8FV[\x92a\"\x05\x84_R_` R`@_ \x90V[\x91a\"\x15\x83T`\xFF\x90`\xA0\x1C\x16\x90V[a\"\x1E\x81a\x07\xCBV[a#&Wa\"4a\"/6\x83a \x0FV[a;PV[\x93a\x02\x95a\"E`\x80\x84\x015a<;V[\x94a\"\xD4a\"\x92a\"\x8Ba\x01\0\x87\x015\x96a\"ca\x01@\x82\x01a\x1AnV[\x90a\"qa\x01`\x82\x01a\x1AnV[\x91a\x01\x80\x82\x015\x90a\x01\xA0`\x02T\x93\x015\x93B\x91\x8Ca:5V[\x80\x96a\x1D\xE9V[\x95\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a\"\xDD\x81a!\x1CV[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B`\xA8\x1By\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x90UV[\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$_\xFD[a#_a\x0C\x8A\x91a\x1F\xEAV[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$\x90V[a#\x9Ca\x0C\x8A\x91a\x1F\xE0V[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x90\x81R`\x04R`\xFF\x16`$R`D\x90V[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a$\x18W[PP_``R`@RV[;\x15=\x17\x10\x15a$)W_\x80a$\rV[cy9\xF4$_R`\x04`\x1C\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\xC3P\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x15\x11WV[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x15\x11WV[\x94\x90\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa$\xCA\x94\x81`\xA0\x98\x9B\x9A\x9B\x16\x89R\x16` \x88\x01R`@\x87\x01R``\x86\x01R`\xC0`\x80\x86\x01R`\xC0\x85\x01\x90a\x15\x16V[\x94\x16\x91\x01RV[\x90_` \x83\x01\x92a$\xE1\x82a\x07\xCBV[RV[\x90`\x02` \x83\x01\x92a$\xE1\x82a\x07\xCBV[\x90`\x01` \x83\x01\x92a$\xE1\x82a\x07\xCBV[`@Q\x90a%\x15` \x83a\x1A\xC9V[_\x82RV[=\x15a%DW=\x90a%+\x82a\x1B8V[\x91a%9`@Q\x93\x84a\x1A\xC9V[\x82R=_` \x84\x01>V[``\x90V[\x90` \x82\x01\x80\x92\x11a\x15\x11WV[\x90`\x1F\x82\x01\x80\x92\x11a\x15\x11WV[`\x12\x01\x90\x81`\x12\x11a\x15\x11WV[\x91\x90\x82\x01\x80\x92\x11a\x15\x11WV[\x93\x94\x90\x91\x94\x83\x15a(;W\x80Q\x15\x80\x15a(2W[a(%WZa%\xCAa%\xBDa%\xB8`\x05\x86\x90\x1Cg\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a$WV[a$6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x11a'\xE8W0;\x15a\x01\xE3Wa&\x14\x91_\x91`@Q\x93\x84\x92\x83\x92\x7Fa|S|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x89\x89\x8C\x8A`\x04\x88\x01a$yV[\x03\x81\x830Z\xF1\x90\x81a'\xD4W[Pa'\x8CWa&6a&1a%\x1AV[a=\xC1V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x94\x85\x15\x15\x80a'\x82W[\x15a&\x9DWP\x90a&j\x92\x91\x84a<\x87V[\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a&\x98\x81a$\xF5V[\x03\x90\xA3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x96P\x82\x93P\x81a'*\x87\x94a'\x07\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[a'5\x87\x82Ta%sV[\x90U\x16\x95\x86\x93a'Q`@Q\x92\x83\x92\x16\x96\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA4\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a&\x98\x81a$\xE4V[P0\x86\x14\x15a&XV[PP\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x93\x16\x92\x80a&\x98\x81a$\xD1V[\x80a\x19\xC3_a'\xE2\x93a\x1A\xC9V[_a&!V[a\x0C\x8A\x82Z\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$R`D\x90V[PPa\x02\x95\x93\x91\x92a<\x87V[P\x85;\x15a%\x95V[PPPPPPV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x15\x11Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[a(\xA2a\x1B\nV[\x90_\x82R_` \x83\x01R_`@\x83\x01R_``\x83\x01R_`\x80\x83\x01R_`\xA0\x83\x01R_`\xC0\x83\x01R_`\xE0\x83\x01R_a\x01\0\x83\x01R_a\x01 \x83\x01R_a\x01@\x83\x01R_a\x01`\x83\x01R_a\x01\x80\x83\x01R_a\x01\xA0\x83\x01R_a\x01\xC0\x83\x01R``a\x01\xE0\x83\x01RV[\x94\x93\x97\x92\x90\x97a)\x19a(\x9AV[P`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x96\x90`\xA0\x81`$\x81\x8BZ\xFA\x80\x15a\r\x05W\x8Ba)\xBD```\xA0\x93a)\xFF\x95_\x91a+\xA8WP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x99`@Q\x80\x80\x95\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\r\x05Wa*3\x91``\x91_\x91a\r\xBBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x15a+vWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a+DWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x97\x91\x81\x16\x92\x91\x16\x90a*\x96\x90\x84a\x1D\xE9V[\x93Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96a*\xAD\x90\x88a$WV[\x97a*\xB6a\x1B\nV[_\x81Rc\xFF\xFF\xFF\xFFF\x16` \x82\x01R\x9Bc\xFF\xFF\xFF\xFF\x16`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01R_a\x01\xC0\x82\x01Ra+;a%\x06V[a\x01\xE0\x82\x01R\x90V[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04R`$_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_RFc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[a\r\xD4\x91P\x85=\x87\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16F\x81\x03a-\x8BWPa+\xEBa+\xE6`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[a>@V[a\x01`\x81\x01Qa\x01@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x80\x82\x11\x15a-PWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a-CW[a-\rWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a,\xDFWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa,Y`\x80\x83\x01Qa<;V[\x16\x15a,\xB7Wa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a,{WPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x91\x90\x91R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a,\x1FV[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x15\x11WV[`@Q\x90a-\xEEa\x01`\x83a\x1A\xC9V[a\x01)\x82R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x83\x7FOrderIntent witness)OrderIntent(` \x82\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x82\x01R\x7FId,bytes32 recipient,uint256 inp``\x82\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x82\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x82\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x82\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x82\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x82\x01R\x7FPermissions(address token,uint25a\x01 \x82\x01R\x01RV[\x91a\x02\x95\x93\x91\x92a0#`\xA0a0\x1Ba/\x8E\x84Q`\xFF\x16\x90V[\x84a/\xA0`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x97`\x80\x82\x01Q\x98`\xE0\x83\x01\x99\x8AQa\x01\0\x85\x01Qa/\xE3a/\xCDa\x01`\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01@\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a-\xBCV[\x91a\x01\x80\x87\x01Q\x93a\x01\xA0\x88\x01Q\x95a0\x15a\x01\xC0a\x01\xE0\x8B\x01Q` \x81Q\x91\x01 \x9A\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x99a?PV[\x92\x01Qa<;V[\x92Q\x825` \x84\x015\x91\x80a0Da09a-\xDEV[\x96`@\x81\x01\x90a\x1A\x1DV[\x98\x90\x97a@\xB0V[\x90\x92\x91\x92a0Y\x82a5\x8FV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x90\x94\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\xA0\x82`$\x81\x84Z\xFA\x91\x82\x15a\r\x05W_\x92a2\xBAW[P\x81\x92`\x80a1j\x93\x01\x93a1\x13a1\x02\x86Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90aBZV[`\xA0`@\x88\x01\x93a1(\x85Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x98\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x91\x82\x15a\r\x05W\x89\x96\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x96a2\n``a1\xEEa1\xD3` a2\x1F\x99s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x9C_\x91a2\x9BW[P\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x93Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x95\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`\xE0\x8B\x01Q\x90a2\x19\x8CaC\xA4V[\x95aEBV[a&\x98a2=a22``\x88\x01Qa<;V[\x92Qc\xFF\xFF\xFF\xFF\x16\x90V[\x95`\xC0\x81\x01Q\x90a2`a\x01 a\x01\0\x83\x01Q\x92\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@\x80Q_\x81Rc\xFF\xFF\xFF\xFF\x90\x9A\x16` \x8B\x01R\x89\x01\x92\x90\x92R``\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01R\x91\x16\x93\x90\x81\x90`\xA0\x82\x01\x90V[a2\xB4\x91P`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[_a1\xC7V[a1j\x92Pa2\xD7\x90`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[\x91a0\xD6V[\x90\x93\x92\x93\x84\x83\x11a\x01\xE3W\x84\x11a\x01\xE3W\x81\x01\x92\x03\x90V[5\x90` \x81\x10a3\x03WP\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90` \x03`\x03\x1B\x1B\x16\x90V[a\x01x\x82\x10a3\xCAW\x81`\x08\x11a\x01\xE3W`\x04\x81\x015`\xE0\x1C\x92a3ia3ca3Z`\xB8a%IV[`\xB8\x86\x86a2\xDDV[\x90a2\xF5V[\x91a3\x83a3ca3z`\xD8a%IV[`\xD8\x87\x85a2\xDDV[\x91a3\xC6a3\xA0a3ca3\x97`\xF8a%IV[`\xF8\x89\x87a2\xDDV[\x95a3\xBBa3ca\x018a3\xB3\x81a%IV[\x90\x84\x88a2\xDDV[\x93\x81a\x01x\x91a2\xDDV[\x90\x91V[P\x7F\xA2\xAB\xF1\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[Q\x90a\x02\x95\x82a\x18\xCAV[Q\x90a\x02\x95\x82a\x06\x1CV[\x81`\x1F\x82\x01\x12\x15a\x01\xE3W\x80Q\x90a4#\x82a\x1B8V[\x92a41`@Q\x94\x85a\x1A\xC9V[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xE3W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[a4Za(\x9AV[P\x80Q\x81\x01\x90` \x81\x83\x03\x12a\x01\xE3W` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xE3W\x01\x90a\x02\0\x82\x82\x03\x12a\x01\xE3Wa4\x94a\x1B\nV[\x91a4\xA1` \x82\x01a3\xF6V[\x83Ra4\xAF`@\x82\x01a\x1C\x91V[` \x84\x01Ra4\xC0``\x82\x01a\x1C\x91V[`@\x84\x01R`\x80\x81\x01Q``\x84\x01R`\xA0\x81\x01Q`\x80\x84\x01R`\xC0\x81\x01Q`\xA0\x84\x01R`\xE0\x81\x01Q`\xC0\x84\x01Ra\x01\0\x81\x01Q`\xE0\x84\x01Ra\x01 \x81\x01Qa\x01\0\x84\x01Ra5\x11a\x01@\x82\x01a4\x01V[a\x01 \x84\x01Ra5$a\x01`\x82\x01a4\x01V[a\x01@\x84\x01Ra57a\x01\x80\x82\x01a4\x01V[a\x01`\x84\x01Ra\x01\xA0\x81\x01Qa\x01\x80\x84\x01Ra\x01\xC0\x81\x01Qa\x01\xA0\x84\x01Ra5ba\x01\xE0\x82\x01a4\x01V[a\x01\xC0\x84\x01Ra\x02\0\x81\x01Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xE3Wa+;\x92` \x80\x92\x01\x92\x01\x01a4\x0CV[\x90a\x01\xE0\x82\x01a7%a7\x1Ba%\xBDa\x01\xC0a5\xD5a5\xC0a5\xBBa5\xB5\x88QQa%WV[`\x05\x1C\x90V[a%eV[\x90`@Q\x82\x81\x93\x82R`\x01\x01`\x05\x1B\x01`@RV[` \x80\x82\x01R\x96a5\xFCa5\xF3a5\xED\x83Q`\xFF\x16\x90V[`\xFF\x16\x90V[`@\x8A\x01R\x88\x90V[Pa6\x1Da6\x14a!\xD1` \x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[``\x8A\x01R\x88\x90V[Pa6>a65a!\xD1`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[`\x80\x8A\x01R\x88\x90V[P``\x81\x01Q`\xA0\x89\x01R`\x80\x81\x01Q`\xC0\x89\x01R`\xA0\x81\x01Q`\xE0\x89\x01R`\xC0\x81\x01Qa\x01\0\x89\x01R`\xE0\x81\x01Qa\x01 \x89\x01Ra\x01\0\x81\x01Qa\x01@\x89\x01Ra6\xA5a6\x9Ba%\xBDa\x01 \x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01`\x8A\x01R\x88\x90V[Pa6\xCCa6\xC2a%\xBDa\x01@\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\x80\x8A\x01R\x88\x90V[Pa6\xF3a6\xE9a%\xBDa\x01`\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\xA0\x8A\x01R\x88\x90V[Pa\x01\x80\x81\x01Qa\x01\xC0\x89\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x89\x01R\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\0\x85\x01R\x83\x90V[Pa\x02\0a\x02 \x84\x01R\x80QQa\x02@\x84\x01RQ\x80Q\x80a7jW[PP\x81Q`\x05\x1B` \x83\x01 a\x02\x95\x90\x92\x80Q`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x87\x01\x94\x01\x01\x90[\x81\x81\x10a7\xEAWPP`\x1F\x16\x90\x81\x15a7AW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a7AV[\x80Q\x84R` \x93\x84\x01\x93\x01a7{V[\x91a8\x0C\x82_R_` R`@_ \x90V[\x80T\x90`\x02`\xA0\x83\x90\x1C`\xFF\x16a8\"\x81a\x07\xCBV[\x14a9AW\x91a8\xA6\x91\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x93a8W\x87a;PV[\x91a8\x82a8h`\x80\x8A\x01Qa<;V[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\0\x89\x01Q\x80\x86\x10\x15a9;WP\x84\x90[a8\x9E\x82\x87a\x1D\xE9V[\x96\x87\x94a!eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x99\x86\x8B\x15a9\x08WPP\x90a8\xD3\x91\x85\x8Aa<\x87V[\x81a8\xF6W[PPP[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01a&\x98V[a9\0\x92\x87a<\x87V[_\x82\x81a8\xD9V[\x92P\x92\x90Pa96\x94\x93Pa9/a\x01\xC0a\x01\xE0\x85\x01Q\x94\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x93\x89a%\x80V[a8\xDDV[\x90a8\x94V[\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x84\x90R`$_\xFD[_\x90\x81\x93`@Q\x93\x838\x93` \x84Q\x94\x01\x92\xF1\x92=`$=\x11a9\xA0W[\x80` \x91\x84R\x80_\x83\x86\x01>\x83\x01\x01`@RV[P`\x01\x92P`$a9\x8CV[\x80Q\x90` \x01\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x95\x92\x90\x94\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x95\x16\x90\x81\x81\x10_\x14a;7WP\x92[_\x93\x85\x81\x10a:}W[PPPa:l\x92Pa%sV[\x81\x81\x11\x15a:xWP\x90V[\x90P\x90V[\x90\x91\x92\x93P\x84\x03\x93\x84\x11a\x15\x11W\x80\x15\x80\x15\x90\x81a:\xD2W[P\x93a:\xBA\x92\x91a:l\x95_\x14a:\xCBWPP\x80[\x80\x82\x11a:\xC3W[P\x84aA\x92V[\x90_\x80\x80a:_V[\x90P_a:\xB3V[\x02\x90a:\xABV[\x90P\x91\x90\x91a;\nW\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x90\x04\x84\x11a:\xBAa:\x96V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90P\x92a:UV[\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x80\x15a\r\x05W``a;\xEC\x91`\xC0\x93_\x91a\r\xBBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x03a<\x10WP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80`\xA0\x1Ca<\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x83\x15a=\xBBW`@Q\x92_\x80` \x86\x01\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x96\x87`$\x82\x01R\x88`D\x82\x01R`D\x81Ra<\xEE`d\x82a\x1A\xC9V[Q\x90\x82\x86Z\xF1a<\xFCa%\x1AV[\x81a=\x8CW[Pa=\x85W\x81a=j\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x93a'\x07s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[a=u\x87\x82Ta%sV[\x90U`@Q\x95\x86R\x16\x93` \x90\xA4V[PPPPPV[\x80Q\x80\x15\x92P\x82\x15a=\xA1W[PP_a=\x02V[a=\xB4\x92P` \x80\x91\x83\x01\x01\x91\x01a\x1DBV[_\x80a=\x99V[PPPPV[\x90_\x91`$\x81Q\x14a=\xD0WPV[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x14a>&WPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`\xA0\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x90\x81\x15a\r\x05W_\x91a?1W[Pa>\xCDa\x0C\x1B\x82Q\x15\x15\x90V[\x90\x81\x15a?\x0EW[Pa>\xDDWPV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90P_a>\xD5V[a?J\x91P`\xA0=`\xA0\x11a\x0C\xFEWa\x0C\xF0\x81\x83a\x1A\xC9V[_a>\xBFV[\x99\x97\x95\x93\x91\x98\x96\x94\x92\x90\x98`@Q\x99` \x8B\x01\x9B\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x8DR`\xFF\x16`@\x8C\x01Rc\xFF\xFF\xFF\xFF\x16``\x8B\x01R`\x80\x8A\x01R`\xA0\x89\x01R`\xC0\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x82\x01Ra\x01\x80\x81Ra?\xF1a\x01\xA0\x82a\x1A\xC9V[Q\x90 \x90V[\x95\x93\x90a@\xA1\x93a@\x82a\x1Ax\x99\x97\x94`@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94a@J\x8C\x82Q` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[` \x81\x81\x01Q\x8D\x84\x01R\x91\x01Q``\x8C\x01R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8C\x01R\x01Q`\xA0\x8A\x01RV[\x16`\xC0\x87\x01R`\xE0\x86\x01Ra\x01@a\x01\0\x86\x01Ra\x01@\x85\x01\x90a\x15\x16V[\x92a\x01 \x81\x85\x03\x91\x01Ra\x1DWV[\x95\x94\x91\x98\x93\x90\x98\x97\x92\x96\x97a@\xE7`@Q\x97a@\xCD`@\x8Aa\x1A\xC9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88RV[` \x87\x01Ra@\xF4a\x1B)V[\x95\x86R` \x86\x01R`@\x85\x01RaA\ta\x1B\x1AV[0\x81R\x94` \x86\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x01\xE3W_\x95aAb\x93`@Q\x98\x89\x97\x88\x97\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\x04\x89\x01a?\xF7V[\x03\x81\x83n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3Z\xF1\x80\x15a\r\x05WaA\x84WPV[\x80a\x19\xC3_a\x02\x95\x93a\x1A\xC9V[\x81\x81\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x81\x83\x85\x04\x14\x83\x15\x17\x02\x15aA\xBDWPPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFg\r\xE0\xB6\xB3\xA7d\0\0\x92\x84\t\x84\x81\x10\x85\x01\x90\x03\x92\t\x90\x80g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15aB8W\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x90V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x90\x81` \x91\x03\x12a\x01\xE3WQa\x1Ax\x81a\x05\xE9V[\x90` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\x04`@Q\x80\x95\x81\x93\x7F,\x12\x19!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x91\x82\x15a\r\x05Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92` \x91_\x91aC\x87W[P`\x04`@Q\x80\x95\x81\x93\x7F\x8D68\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x91\x82\x15a\r\x05W_\x92aCVW[Pc\xFF\xFF\xFF\xFF\x81\x16c\xFF\xFF\xFF\xFF\x83\x16\x03aC\x1FWPPV[\x7F\xC9\xE00\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[aCy\x91\x92P` =` \x11aC\x80W[aCq\x81\x83a\x1A\xC9V[\x81\x01\x90aBEV[\x90_aC\x07V[P=aCgV[aC\x9E\x91P\x82=\x84\x11a\r\xB4Wa\r\xA5\x81\x83a\x1A\xC9V[_aB\xC9V[a\x1Axa\x01\xE0\x91aD\xBC`@Q\x93\x84\x92` \x80\x85\x01RaC\xCA`@\x85\x01\x82Q`\xFF\x16\x90RV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16``\x85\x01R`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\x80\x85\x01R``\x81\x01Q`\xA0\x85\x01R`\x80\x81\x01Q`\xC0\x85\x01R`\xA0\x81\x01Q`\xE0\x85\x01R`\xC0\x81\x01Qa\x01\0\x85\x01R`\xE0\x81\x01Qa\x01 \x85\x01Ra\x01\0\x81\x01Qa\x01@\x85\x01RaDGa\x01 \x82\x01Qa\x01`\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a\x01@\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x85\x01Ra\x01`\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01RaD\xA6a\x01\xC0\x82\x01Qa\x02\0\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a\x15\x16V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x1A\xC9V[\x94\x90c\xFF\xFF\xFF\xFF\x94a\x1Ax\x99\x98\x94\x86a\x01\0\x99\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x8AR\x16` \x89\x01R`@\x88\x01R\x16``\x86\x01R`\x80\x85\x01R`\xA0\x84\x01R\x16`\xC0\x82\x01R\x81`\xE0\x82\x01R\x01\x90a\x15\x16V[\x95\x90\x91\x92\x93\x95\x80`\x14R\x83`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x87Z\xF1\x80`\x01_Q\x14\x16\x15aE\xF1W[P_`4Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x83;\x15a\x01\xE3WaE\xE0_\x96\x92\x87\x93`@Q\x99\x8A\x98\x89\x97\x88\x96\x7Fw\x9BC-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88R0\x920\x91`\x04\x8A\x01aD\xE8V[\x03\x92Z\xF1\x80\x15a\r\x05WaA\x84WPV[=\x84;\x15\x17\x10\x15aF\x03W[_aE{V[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x87Z\xF1P\x83`4R` _`D`\x10\x82\x87Z\xF1\x80`\x01_Q\x14\x16\x15aFGW[PaE\xFDV[=\x84;\x15\x17\x10\x15aFXW_aFAV[c>?\x8Fs_R`\x04`\x1C\xFD\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
event DestinationCallback(bytes32 indexed orderId, address indexed fundsTo, CallbackResult result);
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
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
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
                    orderId: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.orderId.clone(), self.fundsTo.clone())
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
event PayoutDeferred(bytes32 indexed orderId, address indexed to, address indexed token, uint256 amount);
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
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
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
                    orderId: topics.1,
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
                    self.orderId.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
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
constructor(address config_, address owner_, uint256 maxFeeRate_);
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.config_, value.owner_, value.maxFeeRate_)
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
    /**Function with signature `_executeDelivery(address,address,uint256,bytes32,bytes,uint256)` and selector `0x617c537c`.
```solidity
function _executeDelivery(address token, address recipient, uint256 amount, bytes32 orderId, bytes memory hookData, uint256 gasLimit) external;
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
        pub orderId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub hookData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub gasLimit: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`_executeDelivery(address,address,uint256,bytes32,bytes,uint256)`](_executeDeliveryCall) function.
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<_executeDeliveryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: _executeDeliveryCall) -> Self {
                    (
                        value.token,
                        value.recipient,
                        value.amount,
                        value.orderId,
                        value.hookData,
                        value.gasLimit,
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
                        orderId: tuple.3,
                        hookData: tuple.4,
                        gasLimit: tuple.5,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = _executeDeliveryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_executeDelivery(address,address,uint256,bytes32,bytes,uint256)";
            const SELECTOR: [u8; 4] = [97u8, 124u8, 83u8, 124u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasLimit),
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
    /**Function with signature `initiateCCTP(uint32,bytes32,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))` and selector `0xe3bb93e8`.
```solidity
function initiateCCTP(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec) external returns (bytes32 orderId, uint64 nonce);
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
    ///Container type for the return parameters of the [`initiateCCTP(uint32,bytes32,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))`](initiateCCTPCall) function.
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
                        minFinalityThreshold: tuple.4,
                        deliveryWindow: tuple.5,
                        discountRate: tuple.6,
                        baseFee: tuple.7,
                        exec: tuple.8,
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
            const SIGNATURE: &'static str = "initiateCCTP(uint32,bytes32,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes))";
            const SELECTOR: [u8; 4] = [227u8, 187u8, 147u8, 232u8];
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
    /**Function with signature `initiateCCTPFor(uint32,bytes32,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))` and selector `0x34236a39`.
```solidity
function initiateCCTPFor(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 maxFee, uint32 minFinalityThreshold, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec, address from, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId, uint64 nonce);
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
    ///Container type for the return parameters of the [`initiateCCTPFor(uint32,bytes32,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))`](initiateCCTPForCall) function.
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
                        minFinalityThreshold: tuple.4,
                        deliveryWindow: tuple.5,
                        discountRate: tuple.6,
                        baseFee: tuple.7,
                        exec: tuple.8,
                        from: tuple.9,
                        permit: tuple.10,
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
            const SIGNATURE: &'static str = "initiateCCTPFor(uint32,bytes32,uint256,uint256,uint32,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))";
            const SELECTOR: [u8; 4] = [52u8, 35u8, 106u8, 57u8];
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
            [52u8, 35u8, 106u8, 57u8],
            [57u8, 195u8, 50u8, 21u8],
            [84u8, 209u8, 241u8, 61u8],
            [87u8, 120u8, 71u8, 42u8],
            [92u8, 151u8, 90u8, 187u8],
            [95u8, 220u8, 124u8, 18u8],
            [97u8, 124u8, 83u8, 124u8],
            [106u8, 253u8, 216u8, 80u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 111u8, 243u8, 199u8],
            [119u8, 131u8, 154u8, 158u8],
            [121u8, 80u8, 44u8, 85u8],
            [133u8, 193u8, 120u8, 48u8],
            [140u8, 218u8, 150u8, 222u8],
            [141u8, 165u8, 203u8, 91u8],
            [151u8, 195u8, 107u8, 174u8],
            [172u8, 150u8, 80u8, 216u8],
            [204u8, 110u8, 236u8, 112u8],
            [212u8, 87u8, 12u8, 28u8],
            [227u8, 187u8, 147u8, 232u8],
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
            ::core::stringify!(initiateCCTPFor),
            ::core::stringify!(MAX_CALLBACK_GAS_LIMIT),
            ::core::stringify!(cancelOwnershipHandover),
            ::core::stringify!(getOrder),
            ::core::stringify!(paused),
            ::core::stringify!(settle),
            ::core::stringify!(_executeDelivery),
            ::core::stringify!(PERMIT2),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(fillFor),
            ::core::stringify!(FINALITY_FINALIZED),
            ::core::stringify!(config),
            ::core::stringify!(maxFeeRate),
            ::core::stringify!(setMaxFeeRate),
            ::core::stringify!(owner),
            ::core::stringify!(quoteFill),
            ::core::stringify!(multicall),
            ::core::stringify!(FINALITY_FAST),
            ::core::stringify!(claimable),
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
            <initiateCCTPForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <settleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_executeDeliveryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PERMIT2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fillForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <FINALITY_FINALIZEDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <configCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setMaxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <quoteFillCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multicallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <FINALITY_FASTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimableCall as alloy_sol_types::SolCall>::SIGNATURE,
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
        const COUNT: usize = 28usize;
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
                    fn quoteFill(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CctpAdapterCalls> {
                        <quoteFillCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CctpAdapterCalls::quoteFill)
                    }
                    quoteFill
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
        Reentrancy(Reentrancy),
        #[allow(missing_docs)]
        Unauthorized(Unauthorized),
        #[allow(missing_docs)]
        UnsupportedChain(UnsupportedChain),
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
            [37u8, 173u8, 133u8, 148u8],
            [40u8, 2u8, 221u8, 158u8],
            [43u8, 249u8, 80u8, 101u8],
            [46u8, 119u8, 92u8, 124u8],
            [52u8, 62u8, 33u8, 30u8],
            [81u8, 77u8, 132u8, 10u8],
            [88u8, 135u8, 0u8, 199u8],
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
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(OnlySelf),
            ::core::stringify!(NotSourceChain),
            ::core::stringify!(InvalidCallbackGasLimit),
            ::core::stringify!(InvalidWindow),
            ::core::stringify!(InvalidAddress),
            ::core::stringify!(WrongOutputToken),
            ::core::stringify!(OrderAlreadyActive),
            ::core::stringify!(ReceiveMessageFailed),
            ::core::stringify!(InsufficientCallbackGas),
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
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <OnlySelf as alloy_sol_types::SolError>::SIGNATURE,
            <NotSourceChain as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidCallbackGasLimit as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidWindow as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidAddress as alloy_sol_types::SolError>::SIGNATURE,
            <WrongOutputToken as alloy_sol_types::SolError>::SIGNATURE,
            <OrderAlreadyActive as alloy_sol_types::SolError>::SIGNATURE,
            <ReceiveMessageFailed as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientCallbackGas as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 30usize;
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
                Self::Reentrancy(_) => {
                    <Reentrancy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedChain(_) => {
                    <UnsupportedChain as alloy_sol_types::SolError>::SELECTOR
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<CctpAdapterInstance<P, N>>,
    > {
        CctpAdapterInstance::<P, N>::deploy(__provider, config_, owner_, maxFeeRate_)
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
    ) -> alloy_contract::RawCallBuilder<P, N> {
        CctpAdapterInstance::<
            P,
            N,
        >::deploy_builder(__provider, config_, owner_, maxFeeRate_)
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
        ) -> alloy_contract::Result<CctpAdapterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                config_,
                owner_,
                maxFeeRate_,
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
            orderId: alloy::sol_types::private::FixedBytes<32>,
            hookData: alloy::sol_types::private::Bytes,
            gasLimit: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, _executeDeliveryCall, N> {
            self.call_builder(
                &_executeDeliveryCall {
                    token,
                    recipient,
                    amount,
                    orderId,
                    hookData,
                    gasLimit,
                },
            )
        }
        ///Creates a new call builder for the [`cancelOwnershipHandover`] function.
        pub fn cancelOwnershipHandover(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, cancelOwnershipHandoverCall, N> {
            self.call_builder(&cancelOwnershipHandoverCall)
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
