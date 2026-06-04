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

interface OftAdapter {
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
    error EidMismatch(uint32 configured, uint32 onchain);
    error InsufficientCallbackGas(uint256 available, uint256 callbackGasLimit);
    error InvalidAddress(bytes32 b);
    error InvalidBaseFee(uint256 baseFee, uint256 outputAmount);
    error InvalidCallbackGasLimit(uint64 callbackGasLimit, uint64 maxCallbackGasLimit);
    error InvalidMaxFeeRate(uint256 maxFeeRate);
    error InvalidOutputAmount(uint256 outputAmount, uint256 inputAmount);
    error InvalidWindow(uint64 startTime, uint64 expectedDeliveryTime);
    error NewOwnerIsZeroAddress();
    error NoHandoverRequest();
    error NotEndpoint(address caller);
    error NotSourceChain(uint32 srcChainId);
    error NothingToClaim();
    error OnlySelf();
    error OrderAlreadyActive(bytes32 orderId);
    error Paused();
    error Reentrancy();
    error TokenMismatch(address onchain, address configured);
    error Unauthorized();
    error UnsupportedChain(uint32 chainId);
    error UntrustedLocalOFT(address from);
    error UntrustedPeer(bytes32 composeFrom);
    error UntrustedSourceEid(uint32 srcEid);
    error WrongBridgeType(uint8 expected, uint8 got);
    error WrongDestinationChain(uint32 expected);
    error WrongOutputToken(bytes32 outputToken);
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

    constructor(address config_, address owner_, uint256 maxFeeRate_, uint8 oftId_);

    function MAX_CALLBACK_GAS_LIMIT() external view returns (uint64);
    function PERMIT2() external view returns (address);
    function _executeDelivery(address token, address recipient, uint256 amount, uint256 gasLimit, bytes memory callbackData) external;
    function cancelOwnershipHandover() external payable;
    function claim(address token) external returns (uint256 amount);
    function claimable(address account, address token) external view returns (uint256);
    function completeOwnershipHandover(address pendingOwner) external payable;
    function config() external view returns (address);
    function fill(Order memory order) external returns (bytes32 orderId);
    function fillFor(Order memory order, address filler, PermitLib.Permit2Data memory permit) external returns (bytes32 orderId);
    function getOrder(bytes32 orderId) external view returns (OrderRecord memory);
    function initiateOFT(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 minAmountLD, bytes memory extraOptions, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec) external payable returns (bytes32 orderId, uint64 nonce);
    function initiateOFTFor(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 minAmountLD, bytes memory extraOptions, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec, address from, PermitLib.Permit2Data memory permit) external payable returns (bytes32 orderId, uint64 nonce);
    function lzCompose(address from, bytes32, bytes memory message, address, bytes memory) external payable;
    function maxFeeRate() external view returns (uint256);
    function multicall(bytes[] memory data) external payable returns (bytes[] memory);
    function oftId() external view returns (uint8);
    function owner() external view returns (address result);
    function ownershipHandoverExpiresAt(address pendingOwner) external view returns (uint256 result);
    function paused() external view returns (bool);
    function quoteFill(Order memory order, uint256 fillTime) external view returns (uint256 payoutToRecipient, uint256 feeToFiller);
    function renounceOwnership() external payable;
    function requestOwnershipHandover() external payable;
    function selfPermit(address token, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function setMaxFeeRate(uint256 newMaxFeeRate) external;
    function setPaused(bool newPaused) external;
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
        "name": "oftId_",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "initiateOFT",
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
        "name": "minAmountLD",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "extraOptions",
        "type": "bytes",
        "internalType": "bytes"
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
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "initiateOFTFor",
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
        "name": "minAmountLD",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "extraOptions",
        "type": "bytes",
        "internalType": "bytes"
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
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "lzCompose",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "message",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
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
    "name": "oftId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
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
    "name": "EidMismatch",
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
    "name": "NotEndpoint",
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
    "name": "Reentrancy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TokenMismatch",
    "inputs": [
      {
        "name": "onchain",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "configured",
        "type": "address",
        "internalType": "address"
      }
    ]
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
    "name": "UntrustedLocalOFT",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "UntrustedPeer",
    "inputs": [
      {
        "name": "composeFrom",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "UntrustedSourceEid",
    "inputs": [
      {
        "name": "srcEid",
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
pub mod OftAdapter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c03461014957601f61467438819003918201601f19168301916001600160401b0383118484101761014d578084926080946040528339810103126101495761004781610161565b61005360208301610161565b91606060408201519101519260ff8416840361014957670de0b6b3a76400008211610136576001600160a01b0316638b78c6d8198190555f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a36002556001600160a01b031660805260a0526040516144fe908161017682396080518181816104dd01528181611011015281816115dc01528181612a7e01528181612ccd0152818161309d0152613d82015260a05181818161049f01528181611324015281816115ae01528181612a3f01528181612d660152818161305d0152613d540152f35b5063ad6bb6d160e01b5f5260045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101495756fe60806040526004361015610011575f80fd5b5f803560e01c806316c38b3c14611f885780631e83409a14611e4b5780632569296214611de457806331eee44d14611c8d57806339c3321514611c525780634a6be468146117cb57806354d1f13d146117695780635778472a1461168c5780635975e7901461146f5780635c975abb1461142c5780636afdd850146113e5578063715018a6146113485780637376f1c0146112ed578063776ff3c71461103557806379502c5514610fc757806385c1783014610f8c5780638cda96de14610f135780638da5cb5b14610ea357806397c36bae14610ddc578063ac4fca8214610ceb578063ac9650d814610ba1578063d0a10260146103c8578063d4570c1c1461033d578063f04e283e146102d2578063f2fde38b14610279578063f3995c67146101965763fee81cf414610143575f80fd5b346101935760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101935761017a612009565b9063389a75e1600c5252602080600c2054604051908152f35b80fd5b50346102755760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576101ce612009565b6064359060ff82168092036102755773ffffffffffffffffffffffffffffffffffffffff1690813b15610275575f9160e4839260405194859384927fd505accf00000000000000000000000000000000000000000000000000000000845233600485015230602485015260243560448501526044356064850152608484015260843560a484015260a43560c48401525af1610267575080f35b61027391505f90612252565b005b5f80fd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576102ab612009565b6102b3612401565b8060601b156102c5576102739061390f565b637448fbae5f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557610304612009565b61030c612401565b63389a75e1600c52805f526020600c209081544211610330575f610273925561390f565b636f5e88185f526004601cfd5b346102755760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557610374612009565b73ffffffffffffffffffffffffffffffffffffffff61039161202c565b91165f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060405f2054604051908152f35b60a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576103fa612009565b60443567ffffffffffffffff81116102755761041a903690600401612096565b919061042461204f565b5060843567ffffffffffffffff811161027557610445903690600401612096565b5050688000000000ab143c065c610b94576001688000000000ab143c065d60ff60035460401c16610b6c57604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016939181604481875afa8015610a5c5773ffffffffffffffffffffffffffffffffffffffff915f91610b3d575b505116908115610b0b576040517f5e280f11000000000000000000000000000000000000000000000000000000008152602081600481865afa8015610a5c575f90610abe575b73ffffffffffffffffffffffffffffffffffffffff9150163303610a925773ffffffffffffffffffffffffffffffffffffffff16908103610a67575082600c1161027557600881013560e01c9183604c1161027557602c8201359380602c116102755761060990600c84013593604c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb436930191016122cd565b9261061261298c565b508351840193602085019060208187031261027557602081015167ffffffffffffffff81116102755701946102009086900312610275576040519161065683612219565b602086015160ff8116810361027557835261067360408701612374565b966020840197885261068760608801612374565b91604085019283526080880151606086015260a0880151956080860196875260c089015160a087015260e089015160c087015261010089015160e08701526101208901519961010087019a8b526106e16101408b01613b45565b6101208801526106f46101608b01613b45565b6101408801526107076101808b01613b45565b6101608801526101a08a01516101808801526101c08a01516101a08801526107326101e08b01613b45565b996101c088019a8b5261020081015167ffffffffffffffff811161027557602091010186601f820112156102755780519161076c83612293565b9761077a604051998a612252565b83895260208484010111610275575f60208460a0958263ffffffff9601838d015e8a0101526101e0890197885251166024604051809481937f0a70b05600000000000000000000000000000000000000000000000000000000835260048301525afa908115610a5c575f91610a2d575b5081815115918215610a18575b50506109ed57503081036109c2575063ffffffff905116804603610997575061081f82613b5a565b94855f52600160205260405f2090815493600260ff8660a01c16610842816120f0565b1461096b5792604095928795927f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a986108a561089e7fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff98613d25565b9651613e3d565b9373ffffffffffffffffffffffffffffffffffffffff86169c518083105f14610965575081935b740200000000000000000000000000000000000000006108ec868561235a565b998a98161790558c15155f146109405750505061090b908a858b6143ce565b8161092e575b5050505b82519182526020820152a35f688000000000ab143c065d005b61093892886143ce565b868181610911565b61096096955067ffffffffffffffff91935093919351935116938a612849565b610915565b936108cc565b877fb196a44a000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f25ea23d9000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fe73d23c3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6040015163ffffffff1614159050818b6107f7565b610a4f915060a03d60a011610a55575b610a478183612252565b810190612385565b8a6107ea565b503d610a3d565b6040513d5f823e3d90fd5b7f89233a72000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f39e7e94b000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b506020813d602011610b03575b81610ad860209383612252565b8101031261027557610afe73ffffffffffffffffffffffffffffffffffffffff91612303565b61056f565b3d9150610acb565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff461660045260245ffd5b610b5f915060403d604011610b65575b610b578183612252565b810190612324565b86610529565b503d610b4d565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff8111610275573660238201121561027557806004013567ffffffffffffffff8111610275578060051b36602482850101116102755734610275576040519283926020845280846020015260408481019380602485018637850101928391610c56575b505050806040520360401b178060401c9067ffffffffffffffff16f35b919350915b80915f6044825187016024810135918291018537389084305af415610ce257602067ffffffffffffffe0603f5f937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08a87030181528301943d81523d858583013e3d010116933d0101528483821015610cd5575090610c5b565b9350509050838080610c39565b853d5f823e3d90fd5b346102755760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557610d22612009565b610d2a61202c565b9060843567ffffffffffffffff811161027557610d4b903690600401612096565b9190303303610db4575f92610d67610d6e938660443591612438565b36916122cd565b916040519238918360208351930191606435f13d60243d11610dac575b806020918452805f8386013e83010160405215610da457005b805190602001fd5b506024610d8b565b7f14d4a4e8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102755760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff8111610275576102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc82360301126102755780610e96610e8f610104604094013592610e6761014482016121bb565b610e7461016483016121bb565b600254916101846101a485013594013591602435918861399d565b809261235a565b9082519182526020820152f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557600435610f4d612401565b670de0b6b3a76400008111610f6157600255005b7fad6bb6d1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576020600254604051908152f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346102755760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff811161027557806004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8336030112610275576110af61202c565b916044359067ffffffffffffffff821161027557816004019360607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc843603011261027557688000000000ab143c065c610b94576001688000000000ab143c065d60ff60035460401c16610b6c5761112781856125d9565b9783859a839895969960405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152611170606082612252565b5190206040519061118260a083612252565b606a82527f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c417560208301527f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b6560408301527f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e743260608301527f353620616d6f756e742900000000000000000000000000000000000000000000608083015261122b604484018561216a565b949093602401359035888d6101048d013590611246996141cd565b6101e484016112549161216a565b936101c401611262906121bb565b93369061126e926122cd565b916112799488612849565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a35f688000000000ab143c065d604051908152602090f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557602060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557611379612401565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a35f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755005b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760206040516e22d473030f116ddee9f6b43ac78ba38152f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557602060ff60035460401c166040519015158152f35b6101207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576114a2612072565b60443560843567ffffffffffffffff8111610275576114c5903690600401612096565b906114ce6120c4565b93610104359067ffffffffffffffff821161027557816004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc84360301126102755760ff60035460401c16610b6c5761157091602461154f610d6793611536612935565b9a8b9160e4359160c435918d6064359188359033612a00565b9467ffffffffffffffff611562846121bb565b166101c0870152019061216a565b6101e082015261157f81612c98565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa948515610a5c5773ffffffffffffffffffffffffffffffffffffffff60206116529761164d945f9161166d575b50015116309033906127e2565b613023565b6040805191825267ffffffffffffffff929092166020820152f35b611686915060403d604011610b6557610b578183612252565b8a611640565b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275575f604080516116c9816121d0565b82815282602082015201526004355f526001602052606060405f2064ffffffffff604051916116f7836121d0565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c166020840190611726816120f0565b815282604085019260a81c16825273ffffffffffffffffffffffffffffffffffffffff60405194511684525161175b816120f0565b602084015251166040820152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b6101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576117fe612072565b60843567ffffffffffffffff81116102755761181e903690600401612096565b6118299291926120c4565b92610104359267ffffffffffffffff841161027557836004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc863603011261027557610124359073ffffffffffffffffffffffffffffffffffffffff82168203610275576101443567ffffffffffffffff811161027557806004019260607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc83360301126102755760ff60035460401c16610b6c57611937611911611946946118f4612935565b9b8c9160e4359160c435916064359060443590602435908a612a00565b95602461191d826121bb565b9a67ffffffffffffffff6101c08a019c168c52019061216a565b93906101e087019436916122cd565b835261195185612c98565b61195c3688886122cd565b6020815191012060ff8651169463ffffffff6040880151169160808801519660e089019384519c6101008b0151938b67ffffffffffffffff61014081610160840151169201511690039967ffffffffffffffff8b11611c25576116529f67ffffffffffffffff60249c8f929861164d9f997f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b9a846101a0610180880151970151975160208151910120995116996040519b60208d019d8e5260408d015260608c015260808b015260a08a015260c08901521660e08701526101008601526101208501526101408401526101608301526101808201526101808152611a626101a082612252565b51902091611a7360a0890151613e3d565b90518061016095611a876040519788612252565b61012987527f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208801527f75696e743820627269646765547970652c75696e74333220647374436861696e60408801527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608801527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808801527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08801527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08801527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08801527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008801527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208801527f3620616d6f756e74290000000000000000000000000000000000000000000000610140880152611c18604489018661216a565b99909801359435936141cd565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576020604051624c4b408152f35b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff811161027557806004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc833603011261027557688000000000ab143c065c610b94576001688000000000ab143c065d60ff60035460401c16610b6c57611d94611d8b917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d611dcd611d9c611da36101c4602098611d6933886125d9565b9b849d829993949692959c611d808730338b6127e2565b6101e484019061216a565b979092016121bb565b9536916122cd565b9289612849565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d604051908152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557611e82612009565b688000000000ab143c065c610b94576001688000000000ab143c065d335f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5260205260405f20548015611f6057602091335f525f835260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5283525f6040812055611f09823383612438565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd3992683843392a35f688000000000ab143c065d604051908152f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043580151580910361027557611fcc612401565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff00000000000000006003549260401b169116176003555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b6064359073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b6004359063ffffffff8216820361027557565b359063ffffffff8216820361027557565b9181601f840112156102755782359167ffffffffffffffff8311610275576020838186019501011161027557565b60a4359067ffffffffffffffff8216820361027557565b359067ffffffffffffffff8216820361027557565b600311156120fa57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610275570180359067ffffffffffffffff82116102755760200191813603831361027557565b3567ffffffffffffffff811681036102755790565b6060810190811067ffffffffffffffff8211176121ec57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610200810190811067ffffffffffffffff8211176121ec57604052565b6040810190811067ffffffffffffffff8211176121ec57604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176121ec57604052565b67ffffffffffffffff81116121ec57601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b9291926122d982612293565b916122e76040519384612252565b829481845281830111610275578281602093845f960137010152565b519073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b908160409103126102755761235260206040519261234184612236565b61234a81612303565b845201612303565b602082015290565b91908203918211611c2557565b5190811515820361027557565b519063ffffffff8216820361027557565b908160a0910312610275576040519060a0820182811067ffffffffffffffff8211176121ec576123f9916080916040526123be81612367565b84526123cc60208201612374565b60208501526123dd60408201612374565b60408501526123ee60608201612303565b606085015201612303565b608082015290565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392754330361242b57565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f51141615612474575b50505f603452565b3b153d171015612485575f8061246c565b6390b8ec185f526004601cfd5b3560ff811681036102755790565b3563ffffffff811681036102755790565b91906102008382031261027557604051906124cb82612219565b8193803560ff811681036102755783526124e760208201612085565b60208401526124f860408201612085565b6040840152606081013560608401526080810135608084015260a081013560a084015260c081013560c084015260e081013560e084015261010081013561010084015261254861012082016120db565b61012084015261255b61014082016120db565b61014084015261256e61016082016120db565b6101608401526101808101356101808401526101a08101356101a08401526125996101c082016120db565b6101c08401526101e08101359067ffffffffffffffff8211610275570181601f82011215610275576101e0918160206125d4933591016122cd565b910152565b90600160ff6125e784612492565b16036127a6576040820163ffffffff6125ff826124a0565b16460361276c575061261961261436846124b1565b613b5a565b92835f52600160205260405f209283549160ff8360a01c1661263a816120f0565b6127405761265061264b36846124b1565b613d25565b9461265e6080840135613e3d565b947fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff740100000000000000000000000000000000000000006126dd6126d6610100880135976126b061014082016121bb565b6126bd61016083016121bb565b600254916101806101a08501359401359142918d61399d565b809761235a565b967fffffffffffffffffffffff00000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff79ffffffffff0000000000000000000000000000000000000000004260a81b1695169116171716179055565b857f343e211e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b61277a63ffffffff916124a0565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b60ff6127b183612492565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f5260016004521660245260445ffd5b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f5114161561282b575b50505f606052604052565b3b153d17101561283c575f80612820565b637939f4245f526004601cfd5b9161287395939181958051155f14612875575060405161286a602082612252565b5f815293613eb8565b565b61290d6128e1916040519283917f3866c1dc00000000000000000000000000000000000000000000000000000000602084015288602484015273ffffffffffffffffffffffffffffffffffffffff861660448401528760648401526080608484015260a4830190612127565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282612252565b93613eb8565b9067ffffffffffffffff8091169116019067ffffffffffffffff8211611c2557565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff8111611c255767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b6040519061299982612219565b60606101e0835f81525f60208201525f60408201525f838201525f60808201525f60a08201525f60c08201525f60e08201525f6101008201525f6101208201525f6101408201525f6101608201525f6101808201525f6101a08201525f6101c08201520152565b94939197929097612a0f61298c565b50604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff8116602483015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169290919081604481865afa918215610a5c5773ffffffffffffffffffffffffffffffffffffffff6020612b25946040945f91612c7b575b50015116938d83518095819482937f320005c50000000000000000000000000000000000000000000000000000000084526004840190929160ff60209163ffffffff604085019616845216910152565b03915afa908115610a5c5773ffffffffffffffffffffffffffffffffffffffff916020915f91612c5c575b50015116918115610b0b578215612c2a574267ffffffffffffffff1696612b779088612913565b976040519b612b858d612219565b60018d528c4663ffffffff16906020015263ffffffff1660408d015273ffffffffffffffffffffffffffffffffffffffff1660608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015261014085015267ffffffffffffffff166101608401526101808301526101a08201526101c081015f9052604051612c1e602082612252565b5f81526101e082015290565b63ffffffff8b7fb825dd76000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b612c75915060403d604011610b6557610b578183612252565b5f612b50565b612c929150853d8711610b6557610b578183612252565b5f612ad5565b63ffffffff602082015116468103612ff8575063ffffffff60408201511673ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517f0a70b05600000000000000000000000000000000000000000000000000000000815282600482015260a081602481855afa908115610a5c575f91612fd9575b50604080517f320005c500000000000000000000000000000000000000000000000000000000815263ffffffff851660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015292839060449082905afa918215610a5c575f92612fb8575b505115908115612f98575b8115612f75575b50612f4a575067ffffffffffffffff6101608201511667ffffffffffffffff6101408301511680821115612f1c5750506101008101805180158015612f0f575b612edb57506101a082015190519081811015612ead57505073ffffffffffffffffffffffffffffffffffffffff612e316080830151613e3d565b1615612e85576101c0015167ffffffffffffffff16624c4b408111612e535750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f52600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e0830151907f8dd09d91000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5060e08301518111612df7565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff91506020015116155f612db7565b805173ffffffffffffffffffffffffffffffffffffffff16159150612db0565b612fd291925060403d604011610b6557610b578183612252565b905f612da5565b612ff2915060a03d60a011610a5557610a478183612252565b5f612d30565b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b929161302e84613b5a565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff166024820152919573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016929182604481865afa918215610a5c575f926138ee575b5073ffffffffffffffffffffffffffffffffffffffff82511693604051927ffc0c546a000000000000000000000000000000000000000000000000000000008452602084600481895afa938415610a5c575f9461389a575b50602073ffffffffffffffffffffffffffffffffffffffff9101511673ffffffffffffffffffffffffffffffffffffffff841681810361386c5750506040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a081602481885afa908115610a5c5763ffffffff916040915f9161384d575b500151166040517f5e280f110000000000000000000000000000000000000000000000000000000081526020816004818a5afa908115610a5c575f916137fb575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f416ecebf000000000000000000000000000000000000000000000000000000008352165afa8015610a5c575f906137be575b63ffffffff9150169080820361379057505060e0820192835186601452806034526f095ea7b30000000000000000000000005f5260205f6044601082865af18060015f51141615613724575b5050505f603452604082019360a063ffffffff8651166024604051809481937f0a70b05600000000000000000000000000000000000000000000000000000000835260048301525afa908115610a5c5763ffffffff916040915f91613705575b500151169583519561010084019283519860405160208082015260ff875116604082015263ffffffff602088015116606082015263ffffffff89511660808201526060870194855160a0830152608088015160c083015260a088015160e083015260c0880198895161010084015251610120830152865161014083015261342f826134036101e06101208c019b67ffffffffffffffff8d511661016085015267ffffffffffffffff6101408201511661018085015267ffffffffffffffff610160820151166101a08501526101808101516101c08501526101a08101518285015267ffffffffffffffff6101c0820151166102008501520151610200610220840152610240830190612127565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101845283612252565b6040519260e0840184811067ffffffffffffffff8211176121ec5760405283526020830193308552604084019b8c52606084019c8d523690613470926122cd565b9a608083019b8c5260a0830191825260209b8c92604051916134928584612252565b5f835260c08601928352604051946134a986612236565b3486528501935f85526040519e8f9889987fc7c7f5b3000000000000000000000000000000000000000000000000000000008a5260048a01608090525163ffffffff1660848a01525160a48901525160c48801525160e487015251610104860160e09052610164860161351b91612127565b9051908581037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7c0161012487015261355291612127565b9051908481037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7c0161014486015261358991612127565b915160248401525160448301523360648301520381345a9460c095f1958615610a5c578996613632575b509273ffffffffffffffffffffffffffffffffffffffff927f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a959267ffffffffffffffff63ffffffff61360860a09851613e3d565b95511693519151925116926040519960018b528a01526040890152606088015260808701521693a3565b93909491955060c03d60c0116136fe575b61364d8186612252565b84019284840360c081126102755760801361027557899667ffffffffffffffff63ffffffff6136087f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a996136ee73ffffffffffffffffffffffffffffffffffffffff9960808f9c60a09d604051906136c4826121d0565b825182526136d3818401613b45565b9082015260406136e5858285016143a6565b910152016143a6565b50959850505050929550926135b3565b503d613643565b61371e915060a03d60a011610a5557610a478183612252565b5f6132f6565b3d833b15171015613736575b80613296565b5f6034526f095ea7b30000000000000000000000005f525f386044601083865af15060345260205f6044601082855af1908160015f511416613730573b153d171015613783575f80613730565b633e3f8f735f526004601cfd5b7fb9190bb3000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b506020813d6020116137f3575b816137d860209383612252565b81010312610275576137ee63ffffffff91612374565b61324a565b3d91506137cb565b90506020813d602011613845575b8161381660209383612252565b8101031261027557602061383e73ffffffffffffffffffffffffffffffffffffffff92612303565b91506131f5565b3d9150613809565b613866915060a03d60a011610a5557610a478183612252565b5f6131b4565b7ff902523f000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9093506020813d6020116138e6575b816138b660209383612252565b810103126102755760206138de73ffffffffffffffffffffffffffffffffffffffff92612303565b94915061312c565b3d91506138a9565b61390891925060403d604011610b6557610b578183612252565b905f6130d4565b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b91908201809211611c2557565b959290939167ffffffffffffffff8091169416908181105f14613b3e57505b5f938082106139e4575b5050506139d39250613990565b818111156139df575090565b905090565b6139f09293945061235a565b90801580159081613ae0575b5015613ad9575050815b808311613ad1575b5081830291670de0b6b3a764000084158286860414170215613a435750670de0b6b3a76400006139d392045b905f80806139c6565b91670de0b6b3a76400007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8486098281108301900393850983670de0b6b3a76400001115613ac4576139d393828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066902613a3a565b63ae47f7025f526004601cfd5b91505f613a0e565b0291613a06565b9050613b1157807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0482115f6139fc565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b90506139bc565b519067ffffffffffffffff8216820361027557565b906101e08201805151601f8101809111611c255760051c908160120180601211611c2557601360405193849283520160051b0160405267ffffffffffffffff6101c060208401956020875260ff815116604086015263ffffffff602082015116606086015263ffffffff6040820151166080860152606081015160a0860152608081015160c086015260a081015160e086015260c081015161010086015260e08101516101208601526101008101516101408601528261012082015116610160860152826101408201511661018086015282610160820151166101a0860152610180810151828601526101a08101516101e086015201511661020083015261020061022083015280515161024083015251805180613c95575b50508051928360051b902092604051828260010160051b011490151060061b52565b602082016020826102608601940101905b818110613d15575050601f16908115613c735760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80613c73565b8051845260209384019301613ca6565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa8015610a5c57602073ffffffffffffffffffffffffffffffffffffffff9160c0935f91613e1e575b50015116910151818103613df3575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b613e37915060403d604011610b6557610b578183612252565b5f613de1565b8060a01c613e5e5773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3d15613eb3573d90613e9a82612293565b91613ea86040519384612252565b82523d5f602084013e565b606090565b945f9484156141c35780511580156141ba575b6141a9575a61c35067ffffffffffffffff613ef7818616956707ffffffffffffff8160051c1690612913565b160167ffffffffffffffff8111611c255767ffffffffffffffff161161417957303b15610275575f613f9091604051809381927fac4fca8200000000000000000000000000000000000000000000000000000000835273ffffffffffffffffffffffffffffffffffffffff808916998a600486015216968760248501528a6044850152606484015260a0608484015260a4830190612127565b038183305af19081614164575b506141325750613fab613e89565b91849260248151146140c1575b5073ffffffffffffffffffffffffffffffffffffffff831696871515806140b7575b1561401c5750506020927f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9949261401192876143ce565b5060405160018152a3565b602097507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a995935087949192507f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f73ffffffffffffffffffffffffffffffffffffffff88921698899384875286825260408720865f52825260405f206140a3828254613990565b9055604051908152a45060405160028152a3565b5030881415613fda565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff0000000000000000000000000000000000000000000000000000000060246020840151930151921603613fb85773ffffffffffffffffffffffffffffffffffffffff1692505f613fb8565b955050505060207f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a991604051908152a3565b6141719196505f90612252565b5f945f613f9d565b505a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5050919390925061287394506143ce565b50833b15613ecb565b5050505050505050565b9590929893969197949773ffffffffffffffffffffffffffffffffffffffff604051976141f989612236565b16875260208701526040519561420e876121d0565b865260208601968752604086019788526040519861422b8a612236565b308a5260208a01526e22d473030f116ddee9f6b43ac78ba33b15610275578392604051998a998a998a997f137c29fe000000000000000000000000000000000000000000000000000000008b5260048b019051906142a8916020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b5160448a0152516064890152805173ffffffffffffffffffffffffffffffffffffffff1660848901526020015160a488015273ffffffffffffffffffffffffffffffffffffffff1660c487015260e486015261010485016101409052610144850161431291612127565b928484037ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0161012486015281845260208401378082016020015f9052601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01601036020015a925f6e22d473030f116ddee9f6b43ac78ba38195f18015610a5c5761439c5750565b5f61287391612252565b9190826040910312610275576040516143be81612236565b6020808294805184520151910152565b9083156144b4575f806040519473ffffffffffffffffffffffffffffffffffffffff60208701917fa9059cbb000000000000000000000000000000000000000000000000000000008352169586602482015287604482015260448152614435606482612252565b519082855af1614443613e89565b816144ba575b506144b457602073ffffffffffffffffffffffffffffffffffffffff7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f92855f525f835260405f208282165f52835260405f206144a7888254613990565b90556040519687521694a4565b50505050565b80518015925082156144cf575b50505f614449565b81925090602091810103126102755760206144ea9101612367565b5f806144c756fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC04a\x01IW`\x1FaFt8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01MW\x80\x84\x92`\x80\x94`@R\x839\x81\x01\x03\x12a\x01IWa\0G\x81a\x01aV[a\0S` \x83\x01a\x01aV[\x91```@\x82\x01Q\x91\x01Q\x92`\xFF\x84\x16\x84\x03a\x01IWg\r\xE0\xB6\xB3\xA7d\0\0\x82\x11a\x016W`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3`\x02U`\x01`\x01`\xA0\x1B\x03\x16`\x80R`\xA0R`@QaD\xFE\x90\x81a\x01v\x829`\x80Q\x81\x81\x81a\x04\xDD\x01R\x81\x81a\x10\x11\x01R\x81\x81a\x15\xDC\x01R\x81\x81a*~\x01R\x81\x81a,\xCD\x01R\x81\x81a0\x9D\x01Ra=\x82\x01R`\xA0Q\x81\x81\x81a\x04\x9F\x01R\x81\x81a\x13$\x01R\x81\x81a\x15\xAE\x01R\x81\x81a*?\x01R\x81\x81a-f\x01R\x81\x81a0]\x01Ra=T\x01R\xF3[Pc\xADk\xB6\xD1`\xE0\x1B_R`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01IWV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_\x805`\xE0\x1C\x80c\x16\xC3\x8B<\x14a\x1F\x88W\x80c\x1E\x83@\x9A\x14a\x1EKW\x80c%i)b\x14a\x1D\xE4W\x80c1\xEE\xE4M\x14a\x1C\x8DW\x80c9\xC32\x15\x14a\x1CRW\x80cJk\xE4h\x14a\x17\xCBW\x80cT\xD1\xF1=\x14a\x17iW\x80cWxG*\x14a\x16\x8CW\x80cYu\xE7\x90\x14a\x14oW\x80c\\\x97Z\xBB\x14a\x14,W\x80cj\xFD\xD8P\x14a\x13\xE5W\x80cqP\x18\xA6\x14a\x13HW\x80csv\xF1\xC0\x14a\x12\xEDW\x80cwo\xF3\xC7\x14a\x105W\x80cyP,U\x14a\x0F\xC7W\x80c\x85\xC1x0\x14a\x0F\x8CW\x80c\x8C\xDA\x96\xDE\x14a\x0F\x13W\x80c\x8D\xA5\xCB[\x14a\x0E\xA3W\x80c\x97\xC3k\xAE\x14a\r\xDCW\x80c\xACO\xCA\x82\x14a\x0C\xEBW\x80c\xAC\x96P\xD8\x14a\x0B\xA1W\x80c\xD0\xA1\x02`\x14a\x03\xC8W\x80c\xD4W\x0C\x1C\x14a\x03=W\x80c\xF0N(>\x14a\x02\xD2W\x80c\xF2\xFD\xE3\x8B\x14a\x02yW\x80c\xF3\x99\\g\x14a\x01\x96Wc\xFE\xE8\x1C\xF4\x14a\x01CW_\x80\xFD[4a\x01\x93W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\x93Wa\x01za \tV[\x90c8\x9Au\xE1`\x0CRR` \x80`\x0C T`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02uW`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x01\xCEa \tV[`d5\x90`\xFF\x82\x16\x80\x92\x03a\x02uWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81;\x15a\x02uW_\x91`\xE4\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R3`\x04\x85\x01R0`$\x85\x01R`$5`D\x85\x01R`D5`d\x85\x01R`\x84\x84\x01R`\x845`\xA4\x84\x01R`\xA45`\xC4\x84\x01RZ\xF1a\x02gWP\x80\xF3[a\x02s\x91P_\x90a\"RV[\0[_\x80\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x02\xABa \tV[a\x02\xB3a$\x01V[\x80``\x1B\x15a\x02\xC5Wa\x02s\x90a9\x0FV[ctH\xFB\xAE_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x03\x04a \tV[a\x03\x0Ca$\x01V[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x90\x81TB\x11a\x030W_a\x02s\x92Ua9\x0FV[co^\x88\x18_R`\x04`\x1C\xFD[4a\x02uW`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x03ta \tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\x91a ,V[\x91\x16_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x03\xFAa \tV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x04\x1A\x906\x90`\x04\x01a \x96V[\x91\x90a\x04$a OV[P`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x04E\x906\x90`\x04\x01a \x96V[PPh\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x0BlW`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x91\x81`D\x81\x87Z\xFA\x80\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91_\x91a\x0B=W[PQ\x16\x90\x81\x15a\x0B\x0BW`@Q\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x80\x15a\n\\W_\x90a\n\xBEW[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P\x163\x03a\n\x92Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x03a\ngWP\x82`\x0C\x11a\x02uW`\x08\x81\x015`\xE0\x1C\x91\x83`L\x11a\x02uW`,\x82\x015\x93\x80`,\x11a\x02uWa\x06\t\x90`\x0C\x84\x015\x93`L\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xB46\x93\x01\x91\x01a\"\xCDV[\x92a\x06\x12a)\x8CV[P\x83Q\x84\x01\x93` \x85\x01\x90` \x81\x87\x03\x12a\x02uW` \x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x01\x94a\x02\0\x90\x86\x90\x03\x12a\x02uW`@Q\x91a\x06V\x83a\"\x19V[` \x86\x01Q`\xFF\x81\x16\x81\x03a\x02uW\x83Ra\x06s`@\x87\x01a#tV[\x96` \x84\x01\x97\x88Ra\x06\x87``\x88\x01a#tV[\x91`@\x85\x01\x92\x83R`\x80\x88\x01Q``\x86\x01R`\xA0\x88\x01Q\x95`\x80\x86\x01\x96\x87R`\xC0\x89\x01Q`\xA0\x87\x01R`\xE0\x89\x01Q`\xC0\x87\x01Ra\x01\0\x89\x01Q`\xE0\x87\x01Ra\x01 \x89\x01Q\x99a\x01\0\x87\x01\x9A\x8BRa\x06\xE1a\x01@\x8B\x01a;EV[a\x01 \x88\x01Ra\x06\xF4a\x01`\x8B\x01a;EV[a\x01@\x88\x01Ra\x07\x07a\x01\x80\x8B\x01a;EV[a\x01`\x88\x01Ra\x01\xA0\x8A\x01Qa\x01\x80\x88\x01Ra\x01\xC0\x8A\x01Qa\x01\xA0\x88\x01Ra\x072a\x01\xE0\x8B\x01a;EV[\x99a\x01\xC0\x88\x01\x9A\x8BRa\x02\0\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW` \x91\x01\x01\x86`\x1F\x82\x01\x12\x15a\x02uW\x80Q\x91a\x07l\x83a\"\x93V[\x97a\x07z`@Q\x99\x8Aa\"RV[\x83\x89R` \x84\x84\x01\x01\x11a\x02uW_` \x84`\xA0\x95\x82c\xFF\xFF\xFF\xFF\x96\x01\x83\x8D\x01^\x8A\x01\x01Ra\x01\xE0\x89\x01\x97\x88RQ\x16`$`@Q\x80\x94\x81\x93\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\n\\W_\x91a\n-W[P\x81\x81Q\x15\x91\x82\x15a\n\x18W[PPa\t\xEDWP0\x81\x03a\t\xC2WPc\xFF\xFF\xFF\xFF\x90Q\x16\x80F\x03a\t\x97WPa\x08\x1F\x82a;ZV[\x94\x85_R`\x01` R`@_ \x90\x81T\x93`\x02`\xFF\x86`\xA0\x1C\x16a\x08B\x81a \xF0V[\x14a\tkW\x92`@\x95\x92\x87\x95\x92\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x98a\x08\xA5a\x08\x9E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x98a=%V[\x96Qa>=V[\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x9CQ\x80\x83\x10_\x14a\teWP\x81\x93[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x08\xEC\x86\x85a#ZV[\x99\x8A\x98\x16\x17\x90U\x8C\x15\x15_\x14a\t@WPPPa\t\x0B\x90\x8A\x85\x8BaC\xCEV[\x81a\t.W[PPP[\x82Q\x91\x82R` \x82\x01R\xA3_h\x80\0\0\0\0\xAB\x14<\x06]\0[a\t8\x92\x88aC\xCEV[\x86\x81\x81a\t\x11V[a\t`\x96\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x93P\x93\x91\x93Q\x93Q\x16\x93\x8Aa(IV[a\t\x15V[\x93a\x08\xCCV[\x87\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F%\xEA#\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xE7=#\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`@\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x15\x90P\x81\x8Ba\x07\xF7V[a\nO\x91P`\xA0=`\xA0\x11a\nUW[a\nG\x81\x83a\"RV[\x81\x01\x90a#\x85V[\x8Aa\x07\xEAV[P=a\n=V[`@Q=_\x82>=\x90\xFD[\x7F\x89#:r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F9\xE7\xE9K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[P` \x81=` \x11a\x0B\x03W[\x81a\n\xD8` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uWa\n\xFEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a#\x03V[a\x05oV[=\x91Pa\n\xCBV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFFF\x16`\x04R`$_\xFD[a\x0B_\x91P`@=`@\x11a\x0BeW[a\x0BW\x81\x83a\"RV[\x81\x01\x90a#$V[\x86a\x05)V[P=a\x0BMV[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW6`#\x82\x01\x12\x15a\x02uW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x05\x1B6`$\x82\x85\x01\x01\x11a\x02uW4a\x02uW`@Q\x92\x83\x92` \x84R\x80\x84` \x01R`@\x84\x81\x01\x93\x80`$\x85\x01\x867\x85\x01\x01\x92\x83\x91a\x0CVW[PPP\x80`@R\x03`@\x1B\x17\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[\x91\x93P\x91[\x80\x91_`D\x82Q\x87\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x0C\xE2W` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?_\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8A\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x93=\x01\x01R\x84\x83\x82\x10\x15a\x0C\xD5WP\x90a\x0C[V[\x93PP\x90P\x83\x80\x80a\x0C9V[\x85=_\x82>=\x90\xFD[4a\x02uW`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\r\"a \tV[a\r*a ,V[\x90`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\rK\x906\x90`\x04\x01a \x96V[\x91\x9003\x03a\r\xB4W_\x92a\rga\rn\x93\x86`D5\x91a$8V[6\x91a\"\xCDV[\x91`@Q\x928\x91\x83` \x83Q\x93\x01\x91`d5\xF1=`$=\x11a\r\xACW[\x80` \x91\x84R\x80_\x83\x86\x01>\x83\x01\x01`@R\x15a\r\xA4W\0[\x80Q\x90` \x01\xFD[P`$a\r\x8BV[\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02uW`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x826\x03\x01\x12a\x02uW\x80a\x0E\x96a\x0E\x8Fa\x01\x04`@\x94\x015\x92a\x0Ega\x01D\x82\x01a!\xBBV[a\x0Eta\x01d\x83\x01a!\xBBV[`\x02T\x91a\x01\x84a\x01\xA4\x85\x015\x94\x015\x91`$5\x91\x88a9\x9DV[\x80\x92a#ZV[\x90\x82Q\x91\x82R` \x82\x01R\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045a\x0FMa$\x01V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x0FaW`\x02U\0[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `\x02T`@Q\x90\x81R\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x02uW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x02uWa\x10\xAFa ,V[\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW\x81`\x04\x01\x93``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x02uWh\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x0BlWa\x11'\x81\x85a%\xD9V[\x97\x83\x85\x9A\x83\x98\x95\x96\x99`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\x11p``\x82a\"RV[Q\x90 `@Q\x90a\x11\x82`\xA0\x83a\"RV[`j\x82R\x7FFillAuthorization witness)FillAu` \x83\x01R\x7Fthorization(bytes32 orderId)Toke`@\x83\x01R\x7FnPermissions(address token,uint2``\x83\x01R\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x83\x01Ra\x12+`D\x84\x01\x85a!jV[\x94\x90\x93`$\x015\x905\x88\x8Da\x01\x04\x8D\x015\x90a\x12F\x99aA\xCDV[a\x01\xE4\x84\x01a\x12T\x91a!jV[\x93a\x01\xC4\x01a\x12b\x90a!\xBBV[\x936\x90a\x12n\x92a\"\xCDV[\x91a\x12y\x94\x88a(IV[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x13ya$\x01V[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\0[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[a\x01 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x14\xA2a rV[`D5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x14\xC5\x906\x90`\x04\x01a \x96V[\x90a\x14\xCEa \xC4V[\x93a\x01\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW\x81`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x02uW`\xFF`\x03T`@\x1C\x16a\x0BlWa\x15p\x91`$a\x15Oa\rg\x93a\x156a)5V[\x9A\x8B\x91`\xE45\x91`\xC45\x91\x8D`d5\x91\x885\x903a*\0V[\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x15b\x84a!\xBBV[\x16a\x01\xC0\x87\x01R\x01\x90a!jV[a\x01\xE0\x82\x01Ra\x15\x7F\x81a,\x98V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x94\x85\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` a\x16R\x97a\x16M\x94_\x91a\x16mW[P\x01Q\x160\x903\x90a'\xE2V[a0#V[`@\x80Q\x91\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x82\x01R\xF3[a\x16\x86\x91P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[\x8Aa\x16@V[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW_`@\x80Qa\x16\xC9\x81a!\xD0V[\x82\x81R\x82` \x82\x01R\x01R`\x045_R`\x01` R```@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x16\xF7\x83a!\xD0V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16` \x84\x01\x90a\x17&\x81a \xF0V[\x81R\x82`@\x85\x01\x92`\xA8\x1C\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x94Q\x16\x84RQa\x17[\x81a \xF0V[` \x84\x01RQ\x16`@\x82\x01R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[a\x01`\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x17\xFEa rV[`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x18\x1E\x906\x90`\x04\x01a \x96V[a\x18)\x92\x91\x92a \xC4V[\x92a\x01\x045\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\x02uW\x83`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x866\x03\x01\x12a\x02uWa\x01$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWa\x01D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x04\x01\x92``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x02uW`\xFF`\x03T`@\x1C\x16a\x0BlWa\x197a\x19\x11a\x19F\x94a\x18\xF4a)5V[\x9B\x8C\x91`\xE45\x91`\xC45\x91`d5\x90`D5\x90`$5\x90\x8Aa*\0V[\x95`$a\x19\x1D\x82a!\xBBV[\x9Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x8A\x01\x9C\x16\x8CR\x01\x90a!jV[\x93\x90a\x01\xE0\x87\x01\x946\x91a\"\xCDV[\x83Ra\x19Q\x85a,\x98V[a\x19\\6\x88\x88a\"\xCDV[` \x81Q\x91\x01 `\xFF\x86Q\x16\x94c\xFF\xFF\xFF\xFF`@\x88\x01Q\x16\x91`\x80\x88\x01Q\x96`\xE0\x89\x01\x93\x84Q\x9Ca\x01\0\x8B\x01Q\x93\x8Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x81a\x01`\x84\x01Q\x16\x92\x01Q\x16\x90\x03\x99g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x11a\x1C%Wa\x16R\x9Fg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x9C\x8F\x92\x98a\x16M\x9F\x99\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x9A\x84a\x01\xA0a\x01\x80\x88\x01Q\x97\x01Q\x97Q` \x81Q\x91\x01 \x99Q\x16\x99`@Q\x9B` \x8D\x01\x9D\x8ER`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Ra\x01\x80\x82\x01Ra\x01\x80\x81Ra\x1Aba\x01\xA0\x82a\"RV[Q\x90 \x91a\x1As`\xA0\x89\x01Qa>=V[\x90Q\x80a\x01`\x95a\x1A\x87`@Q\x97\x88a\"RV[a\x01)\x87R\x7FOrderIntent witness)OrderIntent(` \x88\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x88\x01R\x7FId,bytes32 recipient,uint256 inp``\x88\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x88\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x88\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x88\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x88\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x88\x01R\x7FPermissions(address token,uint25a\x01 \x88\x01R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x88\x01Ra\x1C\x18`D\x89\x01\x86a!jV[\x99\x90\x98\x015\x945\x93aA\xCDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@QbLK@\x81R\xF3[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x02uWh\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x0BlWa\x1D\x94a\x1D\x8B\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x1D\xCDa\x1D\x9Ca\x1D\xA3a\x01\xC4` \x98a\x1Di3\x88a%\xD9V[\x9B\x84\x9D\x82\x99\x93\x94\x96\x92\x95\x9Ca\x1D\x80\x8703\x8Ba'\xE2V[a\x01\xE4\x84\x01\x90a!jV[\x97\x90\x92\x01a!\xBBV[\x956\x91a\"\xCDV[\x92\x89a(IV[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x1E\x82a \tV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R` R`@_ T\x80\x15a\x1F`W` \x913_R_\x83R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x83R_`@\x81 Ua\x1F\t\x823\x83a$8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83\x843\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045\x80\x15\x15\x80\x91\x03a\x02uWa\x1F\xCCa$\x01V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`d5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x91\x81`\x1F\x84\x01\x12\x15a\x02uW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02uW` \x83\x81\x86\x01\x95\x01\x01\x11a\x02uWV[`\xA45\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`\x03\x11\x15a \xFAWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x02uW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW` \x01\x91\x816\x03\x83\x13a\x02uWV[5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x02uW\x90V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x02\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\xECW`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a\"\xD9\x82a\"\x93V[\x91a\"\xE7`@Q\x93\x84a\"RV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02uW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x90\x81`@\x91\x03\x12a\x02uWa#R` `@Q\x92a#A\x84a\"6V[a#J\x81a#\x03V[\x84R\x01a#\x03V[` \x82\x01R\x90V[\x91\x90\x82\x03\x91\x82\x11a\x1C%WV[Q\x90\x81\x15\x15\x82\x03a\x02uWV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x90\x81`\xA0\x91\x03\x12a\x02uW`@Q\x90`\xA0\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECWa#\xF9\x91`\x80\x91`@Ra#\xBE\x81a#gV[\x84Ra#\xCC` \x82\x01a#tV[` \x85\x01Ra#\xDD`@\x82\x01a#tV[`@\x85\x01Ra#\xEE``\x82\x01a#\x03V[``\x85\x01R\x01a#\x03V[`\x80\x82\x01R\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a$+WV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a$tW[PP_`4RV[;\x15=\x17\x10\x15a$\x85W_\x80a$lV[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5`\xFF\x81\x16\x81\x03a\x02uW\x90V[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x02uW\x90V[\x91\x90a\x02\0\x83\x82\x03\x12a\x02uW`@Q\x90a$\xCB\x82a\"\x19V[\x81\x93\x805`\xFF\x81\x16\x81\x03a\x02uW\x83Ra$\xE7` \x82\x01a \x85V[` \x84\x01Ra$\xF8`@\x82\x01a \x85V[`@\x84\x01R``\x81\x015``\x84\x01R`\x80\x81\x015`\x80\x84\x01R`\xA0\x81\x015`\xA0\x84\x01R`\xC0\x81\x015`\xC0\x84\x01R`\xE0\x81\x015`\xE0\x84\x01Ra\x01\0\x81\x015a\x01\0\x84\x01Ra%Ha\x01 \x82\x01a \xDBV[a\x01 \x84\x01Ra%[a\x01@\x82\x01a \xDBV[a\x01@\x84\x01Ra%na\x01`\x82\x01a \xDBV[a\x01`\x84\x01Ra\x01\x80\x81\x015a\x01\x80\x84\x01Ra\x01\xA0\x81\x015a\x01\xA0\x84\x01Ra%\x99a\x01\xC0\x82\x01a \xDBV[a\x01\xC0\x84\x01Ra\x01\xE0\x81\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW\x01\x81`\x1F\x82\x01\x12\x15a\x02uWa\x01\xE0\x91\x81` a%\xD4\x935\x91\x01a\"\xCDV[\x91\x01RV[\x90`\x01`\xFFa%\xE7\x84a$\x92V[\x16\x03a'\xA6W`@\x82\x01c\xFF\xFF\xFF\xFFa%\xFF\x82a$\xA0V[\x16F\x03a'lWPa&\x19a&\x146\x84a$\xB1V[a;ZV[\x92\x83_R`\x01` R`@_ \x92\x83T\x91`\xFF\x83`\xA0\x1C\x16a&:\x81a \xF0V[a'@Wa&Pa&K6\x84a$\xB1V[a=%V[\x94a&^`\x80\x84\x015a>=V[\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&\xDDa&\xD6a\x01\0\x88\x015\x97a&\xB0a\x01@\x82\x01a!\xBBV[a&\xBDa\x01`\x83\x01a!\xBBV[`\x02T\x91a\x01\x80a\x01\xA0\x85\x015\x94\x015\x91B\x91\x8Da9\x9DV[\x80\x97a#ZV[\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFy\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0B`\xA8\x1B\x16\x95\x16\x91\x16\x17\x17\x16\x17\x90UV[\x85\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a'zc\xFF\xFF\xFF\xFF\x91a$\xA0V[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\xFFa'\xB1\x83a$\x92V[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R\x16`$R`D_\xFD[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a(+W[PP_``R`@RV[;\x15=\x17\x10\x15a(<W_\x80a( V[cy9\xF4$_R`\x04`\x1C\xFD[\x91a(s\x95\x93\x91\x81\x95\x80Q\x15_\x14a(uWP`@Qa(j` \x82a\"RV[_\x81R\x93a>\xB8V[V[a)\ra(\xE1\x91`@Q\x92\x83\x91\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x88`$\x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`D\x84\x01R\x87`d\x84\x01R`\x80`\x84\x84\x01R`\xA4\x83\x01\x90a!'V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\"RV[\x93a>\xB8V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1C%WV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1C%Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[`@Q\x90a)\x99\x82a\"\x19V[``a\x01\xE0\x83_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R_`\xA0\x82\x01R_`\xC0\x82\x01R_`\xE0\x82\x01R_a\x01\0\x82\x01R_a\x01 \x82\x01R_a\x01@\x82\x01R_a\x01`\x82\x01R_a\x01\x80\x82\x01R_a\x01\xA0\x82\x01R_a\x01\xC0\x82\x01R\x01RV[\x94\x93\x91\x97\x92\x90\x97a*\x0Fa)\x8CV[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x81\x16`$\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x90\x91\x90\x81`D\x81\x86Z\xFA\x91\x82\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` a+%\x94`@\x94_\x91a,{W[P\x01Q\x16\x93\x8D\x83Q\x80\x95\x81\x94\x82\x93\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\xFF` \x91c\xFF\xFF\xFF\xFF`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91` \x91_\x91a,\\W[P\x01Q\x16\x91\x81\x15a\x0B\x0BW\x82\x15a,*WBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96a+w\x90\x88a)\x13V[\x97`@Q\x9Ba+\x85\x8Da\"\x19V[`\x01\x8DR\x8CFc\xFF\xFF\xFF\xFF\x16\x90` \x01Rc\xFF\xFF\xFF\xFF\x16`@\x8D\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Ra\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01Ra\x01\xC0\x81\x01_\x90R`@Qa,\x1E` \x82a\"RV[_\x81Ra\x01\xE0\x82\x01R\x90V[c\xFF\xFF\xFF\xFF\x8B\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[a,u\x91P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[_a+PV[a,\x92\x91P\x85=\x87\x11a\x0BeWa\x0BW\x81\x83a\"RV[_a*\xD5V[c\xFF\xFF\xFF\xFF` \x82\x01Q\x16F\x81\x03a/\xF8WPc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R`\xA0\x81`$\x81\x85Z\xFA\x90\x81\x15a\n\\W_\x91a/\xD9W[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x92\x83\x90`D\x90\x82\x90Z\xFA\x91\x82\x15a\n\\W_\x92a/\xB8W[PQ\x15\x90\x81\x15a/\x98W[\x81\x15a/uW[Pa/JWPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x83\x01Q\x16\x80\x82\x11\x15a/\x1CWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a/\x0FW[a.\xDBWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a.\xADWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.1`\x80\x83\x01Qa>=V[\x16\x15a.\x85Wa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a.SWPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x90\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a-\xF7V[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P` \x01Q\x16\x15_a-\xB7V[\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x91Pa-\xB0V[a/\xD2\x91\x92P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[\x90_a-\xA5V[a/\xF2\x91P`\xA0=`\xA0\x11a\nUWa\nG\x81\x83a\"RV[_a-0V[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x92\x91a0.\x84a;ZV[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x91\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x91\x82`D\x81\x86Z\xFA\x91\x82\x15a\n\\W_\x92a8\xEEW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82Q\x16\x93`@Q\x92\x7F\xFC\x0CTj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R` \x84`\x04\x81\x89Z\xFA\x93\x84\x15a\n\\W_\x94a8\x9AW[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x01Q\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x81\x03a8lWPP`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x88Z\xFA\x90\x81\x15a\n\\Wc\xFF\xFF\xFF\xFF\x91`@\x91_\x91a8MW[P\x01Q\x16`@Q\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x8AZ\xFA\x90\x81\x15a\n\\W_\x91a7\xFBW[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7FAn\xCE\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x80\x15a\n\\W_\x90a7\xBEW[c\xFF\xFF\xFF\xFF\x91P\x16\x90\x80\x82\x03a7\x90WPP`\xE0\x82\x01\x92\x83Q\x86`\x14R\x80`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x86Z\xF1\x80`\x01_Q\x14\x16\x15a7$W[PPP_`4R`@\x82\x01\x93`\xA0c\xFF\xFF\xFF\xFF\x86Q\x16`$`@Q\x80\x94\x81\x93\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\n\\Wc\xFF\xFF\xFF\xFF\x91`@\x91_\x91a7\x05W[P\x01Q\x16\x95\x83Q\x95a\x01\0\x84\x01\x92\x83Q\x98`@Q` \x80\x82\x01R`\xFF\x87Q\x16`@\x82\x01Rc\xFF\xFF\xFF\xFF` \x88\x01Q\x16``\x82\x01Rc\xFF\xFF\xFF\xFF\x89Q\x16`\x80\x82\x01R``\x87\x01\x94\x85Q`\xA0\x83\x01R`\x80\x88\x01Q`\xC0\x83\x01R`\xA0\x88\x01Q`\xE0\x83\x01R`\xC0\x88\x01\x98\x89Qa\x01\0\x84\x01RQa\x01 \x83\x01R\x86Qa\x01@\x83\x01Ra4/\x82a4\x03a\x01\xE0a\x01 \x8C\x01\x9Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8DQ\x16a\x01`\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x82\x01Q\x16a\x01\x80\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x82\x01Q\x16a\x02\0\x85\x01R\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a!'V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x84R\x83a\"RV[`@Q\x92`\xE0\x84\x01\x84\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@R\x83R` \x83\x01\x930\x85R`@\x84\x01\x9B\x8CR``\x84\x01\x9C\x8DR6\x90a4p\x92a\"\xCDV[\x9A`\x80\x83\x01\x9B\x8CR`\xA0\x83\x01\x91\x82R` \x9B\x8C\x92`@Q\x91a4\x92\x85\x84a\"RV[_\x83R`\xC0\x86\x01\x92\x83R`@Q\x94a4\xA9\x86a\"6V[4\x86R\x85\x01\x93_\x85R`@Q\x9E\x8F\x98\x89\x98\x7F\xC7\xC7\xF5\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8AR`\x04\x8A\x01`\x80\x90RQc\xFF\xFF\xFF\xFF\x16`\x84\x8A\x01RQ`\xA4\x89\x01RQ`\xC4\x88\x01RQ`\xE4\x87\x01RQa\x01\x04\x86\x01`\xE0\x90Ra\x01d\x86\x01a5\x1B\x91a!'V[\x90Q\x90\x85\x81\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF|\x01a\x01$\x87\x01Ra5R\x91a!'V[\x90Q\x90\x84\x81\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF|\x01a\x01D\x86\x01Ra5\x89\x91a!'V[\x91Q`$\x84\x01RQ`D\x83\x01R3`d\x83\x01R\x03\x814Z\x94`\xC0\x95\xF1\x95\x86\x15a\n\\W\x89\x96a62W[P\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x95\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFc\xFF\xFF\xFF\xFFa6\x08`\xA0\x98Qa>=V[\x95Q\x16\x93Q\x91Q\x92Q\x16\x92`@Q\x99`\x01\x8BR\x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R\x16\x93\xA3V[\x93\x90\x94\x91\x95P`\xC0=`\xC0\x11a6\xFEW[a6M\x81\x86a\"RV[\x84\x01\x92\x84\x84\x03`\xC0\x81\x12a\x02uW`\x80\x13a\x02uW\x89\x96g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFc\xFF\xFF\xFF\xFFa6\x08\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x99a6\xEEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x99`\x80\x8F\x9C`\xA0\x9D`@Q\x90a6\xC4\x82a!\xD0V[\x82Q\x82Ra6\xD3\x81\x84\x01a;EV[\x90\x82\x01R`@a6\xE5\x85\x82\x85\x01aC\xA6V[\x91\x01R\x01aC\xA6V[P\x95\x98PPPP\x92\x95P\x92a5\xB3V[P=a6CV[a7\x1E\x91P`\xA0=`\xA0\x11a\nUWa\nG\x81\x83a\"RV[_a2\xF6V[=\x83;\x15\x17\x10\x15a76W[\x80a2\x96V[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x86Z\xF1P`4R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16a70W;\x15=\x17\x10\x15a7\x83W_\x80a70V[c>?\x8Fs_R`\x04`\x1C\xFD[\x7F\xB9\x19\x0B\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P` \x81=` \x11a7\xF3W[\x81a7\xD8` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uWa7\xEEc\xFF\xFF\xFF\xFF\x91a#tV[a2JV[=\x91Pa7\xCBV[\x90P` \x81=` \x11a8EW[\x81a8\x16` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uW` a8>s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a#\x03V[\x91Pa1\xF5V[=\x91Pa8\tV[a8f\x91P`\xA0=`\xA0\x11a\nUWa\nG\x81\x83a\"RV[_a1\xB4V[\x7F\xF9\x02R?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x93P` \x81=` \x11a8\xE6W[\x81a8\xB6` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uW` a8\xDEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a#\x03V[\x94\x91Pa1,V[=\x91Pa8\xA9V[a9\x08\x91\x92P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[\x90_a0\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x91\x90\x82\x01\x80\x92\x11a\x1C%WV[\x95\x92\x90\x93\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x90\x81\x81\x10_\x14a;>WP[_\x93\x80\x82\x10a9\xE4W[PPPa9\xD3\x92Pa9\x90V[\x81\x81\x11\x15a9\xDFWP\x90V[\x90P\x90V[a9\xF0\x92\x93\x94Pa#ZV[\x90\x80\x15\x80\x15\x90\x81a:\xE0W[P\x15a:\xD9WPP\x81[\x80\x83\x11a:\xD1W[P\x81\x83\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x84\x15\x82\x86\x86\x04\x14\x17\x02\x15a:CWPg\r\xE0\xB6\xB3\xA7d\0\0a9\xD3\x92\x04[\x90_\x80\x80a9\xC6V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x86\t\x82\x81\x10\x83\x01\x90\x03\x93\x85\t\x83g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a:\xC4Wa9\xD3\x93\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02a::V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x91P_a:\x0EV[\x02\x91a:\x06V[\x90Pa;\x11W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11_a9\xFCV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90Pa9\xBCV[Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x90a\x01\xE0\x82\x01\x80QQ`\x1F\x81\x01\x80\x91\x11a\x1C%W`\x05\x1C\x90\x81`\x12\x01\x80`\x12\x11a\x1C%W`\x13`@Q\x93\x84\x92\x83R\x01`\x05\x1B\x01`@Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0` \x84\x01\x95` \x87R`\xFF\x81Q\x16`@\x86\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16``\x86\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`\x80\x86\x01R``\x81\x01Q`\xA0\x86\x01R`\x80\x81\x01Q`\xC0\x86\x01R`\xA0\x81\x01Q`\xE0\x86\x01R`\xC0\x81\x01Qa\x01\0\x86\x01R`\xE0\x81\x01Qa\x01 \x86\x01Ra\x01\0\x81\x01Qa\x01@\x86\x01R\x82a\x01 \x82\x01Q\x16a\x01`\x86\x01R\x82a\x01@\x82\x01Q\x16a\x01\x80\x86\x01R\x82a\x01`\x82\x01Q\x16a\x01\xA0\x86\x01Ra\x01\x80\x81\x01Q\x82\x86\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x86\x01R\x01Q\x16a\x02\0\x83\x01Ra\x02\0a\x02 \x83\x01R\x80QQa\x02@\x83\x01RQ\x80Q\x80a<\x95W[PP\x80Q\x92\x83`\x05\x1B\x90 \x92`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x86\x01\x94\x01\x01\x90[\x81\x81\x10a=\x15WPP`\x1F\x16\x90\x81\x15a<sW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a<sV[\x80Q\x84R` \x93\x84\x01\x93\x01a<\xA6V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x80\x15a\n\\W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\xC0\x93_\x91a>\x1EW[P\x01Q\x16\x91\x01Q\x81\x81\x03a=\xF3WP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a>7\x91P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[_a=\xE1V[\x80`\xA0\x1Ca>^Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[=\x15a>\xB3W=\x90a>\x9A\x82a\"\x93V[\x91a>\xA8`@Q\x93\x84a\"RV[\x82R=_` \x84\x01>V[``\x90V[\x94_\x94\x84\x15aA\xC3W\x80Q\x15\x80\x15aA\xBAW[aA\xA9WZa\xC3Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa>\xF7\x81\x86\x16\x95g\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`\x05\x1C\x16\x90a)\x13V[\x16\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1C%Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aAyW0;\x15a\x02uW_a?\x90\x91`@Q\x80\x93\x81\x92\x7F\xACO\xCA\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x99\x8A`\x04\x86\x01R\x16\x96\x87`$\x85\x01R\x8A`D\x85\x01R`d\x84\x01R`\xA0`\x84\x84\x01R`\xA4\x83\x01\x90a!'V[\x03\x81\x830Z\xF1\x90\x81aAdW[PaA2WPa?\xABa>\x89V[\x91\x84\x92`$\x81Q\x14a@\xC1W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x96\x87\x15\x15\x80a@\xB7W[\x15a@\x1CWPP` \x92\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x94\x92a@\x11\x92\x87aC\xCEV[P`@Q`\x01\x81R\xA3V[` \x97P\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x95\x93P\x87\x94\x91\x92P\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x92\x16\x98\x89\x93\x84\x87R\x86\x82R`@\x87 \x86_R\x82R`@_ a@\xA3\x82\x82Ta9\x90V[\x90U`@Q\x90\x81R\xA4P`@Q`\x02\x81R\xA3V[P0\x88\x14\x15a?\xDAV[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x03a?\xB8Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P_a?\xB8V[\x95PPPP` \x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x91`@Q\x90\x81R\xA3V[aAq\x91\x96P_\x90a\"RV[_\x94_a?\x9DV[PZ\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[PP\x91\x93\x90\x92Pa(s\x94PaC\xCEV[P\x83;\x15a>\xCBV[PPPPPPPPV[\x95\x90\x92\x98\x93\x96\x91\x97\x94\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x97aA\xF9\x89a\"6V[\x16\x87R` \x87\x01R`@Q\x95aB\x0E\x87a!\xD0V[\x86R` \x86\x01\x96\x87R`@\x86\x01\x97\x88R`@Q\x98aB+\x8Aa\"6V[0\x8AR` \x8A\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x02uW\x83\x92`@Q\x99\x8A\x99\x8A\x99\x8A\x99\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8BR`\x04\x8B\x01\x90Q\x90aB\xA8\x91` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[Q`D\x8A\x01RQ`d\x89\x01R\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x84\x89\x01R` \x01Q`\xA4\x88\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC4\x87\x01R`\xE4\x86\x01Ra\x01\x04\x85\x01a\x01@\x90Ra\x01D\x85\x01aC\x12\x91a!'V[\x92\x84\x84\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x01a\x01$\x86\x01R\x81\x84R` \x84\x017\x80\x82\x01` \x01_\x90R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x03` \x01Z\x92_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81\x95\xF1\x80\x15a\n\\WaC\x9CWPV[_a(s\x91a\"RV[\x91\x90\x82`@\x91\x03\x12a\x02uW`@QaC\xBE\x81a\"6V[` \x80\x82\x94\x80Q\x84R\x01Q\x91\x01RV[\x90\x83\x15aD\xB4W_\x80`@Q\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x87\x01\x91\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16\x95\x86`$\x82\x01R\x87`D\x82\x01R`D\x81RaD5`d\x82a\"RV[Q\x90\x82\x85Z\xF1aDCa>\x89V[\x81aD\xBAW[PaD\xB4W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x92\x85_R_\x83R`@_ \x82\x82\x16_R\x83R`@_ aD\xA7\x88\x82Ta9\x90V[\x90U`@Q\x96\x87R\x16\x94\xA4V[PPPPV[\x80Q\x80\x15\x92P\x82\x15aD\xCFW[PP_aDIV[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x02uW` aD\xEA\x91\x01a#gV[_\x80aD\xC7V\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f803560e01c806316c38b3c14611f885780631e83409a14611e4b5780632569296214611de457806331eee44d14611c8d57806339c3321514611c525780634a6be468146117cb57806354d1f13d146117695780635778472a1461168c5780635975e7901461146f5780635c975abb1461142c5780636afdd850146113e5578063715018a6146113485780637376f1c0146112ed578063776ff3c71461103557806379502c5514610fc757806385c1783014610f8c5780638cda96de14610f135780638da5cb5b14610ea357806397c36bae14610ddc578063ac4fca8214610ceb578063ac9650d814610ba1578063d0a10260146103c8578063d4570c1c1461033d578063f04e283e146102d2578063f2fde38b14610279578063f3995c67146101965763fee81cf414610143575f80fd5b346101935760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101935761017a612009565b9063389a75e1600c5252602080600c2054604051908152f35b80fd5b50346102755760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576101ce612009565b6064359060ff82168092036102755773ffffffffffffffffffffffffffffffffffffffff1690813b15610275575f9160e4839260405194859384927fd505accf00000000000000000000000000000000000000000000000000000000845233600485015230602485015260243560448501526044356064850152608484015260843560a484015260a43560c48401525af1610267575080f35b61027391505f90612252565b005b5f80fd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576102ab612009565b6102b3612401565b8060601b156102c5576102739061390f565b637448fbae5f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557610304612009565b61030c612401565b63389a75e1600c52805f526020600c209081544211610330575f610273925561390f565b636f5e88185f526004601cfd5b346102755760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557610374612009565b73ffffffffffffffffffffffffffffffffffffffff61039161202c565b91165f525f60205273ffffffffffffffffffffffffffffffffffffffff60405f2091165f52602052602060405f2054604051908152f35b60a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576103fa612009565b60443567ffffffffffffffff81116102755761041a903690600401612096565b919061042461204f565b5060843567ffffffffffffffff811161027557610445903690600401612096565b5050688000000000ab143c065c610b94576001688000000000ab143c065d60ff60035460401c16610b6c57604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016939181604481875afa8015610a5c5773ffffffffffffffffffffffffffffffffffffffff915f91610b3d575b505116908115610b0b576040517f5e280f11000000000000000000000000000000000000000000000000000000008152602081600481865afa8015610a5c575f90610abe575b73ffffffffffffffffffffffffffffffffffffffff9150163303610a925773ffffffffffffffffffffffffffffffffffffffff16908103610a67575082600c1161027557600881013560e01c9183604c1161027557602c8201359380602c116102755761060990600c84013593604c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb436930191016122cd565b9261061261298c565b508351840193602085019060208187031261027557602081015167ffffffffffffffff81116102755701946102009086900312610275576040519161065683612219565b602086015160ff8116810361027557835261067360408701612374565b966020840197885261068760608801612374565b91604085019283526080880151606086015260a0880151956080860196875260c089015160a087015260e089015160c087015261010089015160e08701526101208901519961010087019a8b526106e16101408b01613b45565b6101208801526106f46101608b01613b45565b6101408801526107076101808b01613b45565b6101608801526101a08a01516101808801526101c08a01516101a08801526107326101e08b01613b45565b996101c088019a8b5261020081015167ffffffffffffffff811161027557602091010186601f820112156102755780519161076c83612293565b9761077a604051998a612252565b83895260208484010111610275575f60208460a0958263ffffffff9601838d015e8a0101526101e0890197885251166024604051809481937f0a70b05600000000000000000000000000000000000000000000000000000000835260048301525afa908115610a5c575f91610a2d575b5081815115918215610a18575b50506109ed57503081036109c2575063ffffffff905116804603610997575061081f82613b5a565b94855f52600160205260405f2090815493600260ff8660a01c16610842816120f0565b1461096b5792604095928795927f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a986108a561089e7fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff98613d25565b9651613e3d565b9373ffffffffffffffffffffffffffffffffffffffff86169c518083105f14610965575081935b740200000000000000000000000000000000000000006108ec868561235a565b998a98161790558c15155f146109405750505061090b908a858b6143ce565b8161092e575b5050505b82519182526020820152a35f688000000000ab143c065d005b61093892886143ce565b868181610911565b61096096955067ffffffffffffffff91935093919351935116938a612849565b610915565b936108cc565b877fb196a44a000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f25ea23d9000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fe73d23c3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b6040015163ffffffff1614159050818b6107f7565b610a4f915060a03d60a011610a55575b610a478183612252565b810190612385565b8a6107ea565b503d610a3d565b6040513d5f823e3d90fd5b7f89233a72000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f39e7e94b000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b506020813d602011610b03575b81610ad860209383612252565b8101031261027557610afe73ffffffffffffffffffffffffffffffffffffffff91612303565b61056f565b3d9150610acb565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff461660045260245ffd5b610b5f915060403d604011610b65575b610b578183612252565b810190612324565b86610529565b503d610b4d565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff8111610275573660238201121561027557806004013567ffffffffffffffff8111610275578060051b36602482850101116102755734610275576040519283926020845280846020015260408481019380602485018637850101928391610c56575b505050806040520360401b178060401c9067ffffffffffffffff16f35b919350915b80915f6044825187016024810135918291018537389084305af415610ce257602067ffffffffffffffe0603f5f937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08a87030181528301943d81523d858583013e3d010116933d0101528483821015610cd5575090610c5b565b9350509050838080610c39565b853d5f823e3d90fd5b346102755760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557610d22612009565b610d2a61202c565b9060843567ffffffffffffffff811161027557610d4b903690600401612096565b9190303303610db4575f92610d67610d6e938660443591612438565b36916122cd565b916040519238918360208351930191606435f13d60243d11610dac575b806020918452805f8386013e83010160405215610da457005b805190602001fd5b506024610d8b565b7f14d4a4e8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102755760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff8111610275576102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc82360301126102755780610e96610e8f610104604094013592610e6761014482016121bb565b610e7461016483016121bb565b600254916101846101a485013594013591602435918861399d565b809261235a565b9082519182526020820152f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557600435610f4d612401565b670de0b6b3a76400008111610f6157600255005b7fad6bb6d1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576020600254604051908152f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346102755760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff811161027557806004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc8336030112610275576110af61202c565b916044359067ffffffffffffffff821161027557816004019360607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc843603011261027557688000000000ab143c065c610b94576001688000000000ab143c065d60ff60035460401c16610b6c5761112781856125d9565b9783859a839895969960405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152611170606082612252565b5190206040519061118260a083612252565b606a82527f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c417560208301527f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b6560408301527f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e743260608301527f353620616d6f756e742900000000000000000000000000000000000000000000608083015261122b604484018561216a565b949093602401359035888d6101048d013590611246996141cd565b6101e484016112549161216a565b936101c401611262906121bb565b93369061126e926122cd565b916112799488612849565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a35f688000000000ab143c065d604051908152602090f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557602060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557611379612401565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a35f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755005b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760206040516e22d473030f116ddee9f6b43ac78ba38152f35b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557602060ff60035460401c166040519015158152f35b6101207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576114a2612072565b60443560843567ffffffffffffffff8111610275576114c5903690600401612096565b906114ce6120c4565b93610104359067ffffffffffffffff821161027557816004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc84360301126102755760ff60035460401c16610b6c5761157091602461154f610d6793611536612935565b9a8b9160e4359160c435918d6064359188359033612a00565b9467ffffffffffffffff611562846121bb565b166101c0870152019061216a565b6101e082015261157f81612c98565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa948515610a5c5773ffffffffffffffffffffffffffffffffffffffff60206116529761164d945f9161166d575b50015116309033906127e2565b613023565b6040805191825267ffffffffffffffff929092166020820152f35b611686915060403d604011610b6557610b578183612252565b8a611640565b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275575f604080516116c9816121d0565b82815282602082015201526004355f526001602052606060405f2064ffffffffff604051916116f7836121d0565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c166020840190611726816120f0565b815282604085019260a81c16825273ffffffffffffffffffffffffffffffffffffffff60405194511684525161175b816120f0565b602084015251166040820152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b6101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576117fe612072565b60843567ffffffffffffffff81116102755761181e903690600401612096565b6118299291926120c4565b92610104359267ffffffffffffffff841161027557836004019060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc863603011261027557610124359073ffffffffffffffffffffffffffffffffffffffff82168203610275576101443567ffffffffffffffff811161027557806004019260607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc83360301126102755760ff60035460401c16610b6c57611937611911611946946118f4612935565b9b8c9160e4359160c435916064359060443590602435908a612a00565b95602461191d826121bb565b9a67ffffffffffffffff6101c08a019c168c52019061216a565b93906101e087019436916122cd565b835261195185612c98565b61195c3688886122cd565b6020815191012060ff8651169463ffffffff6040880151169160808801519660e089019384519c6101008b0151938b67ffffffffffffffff61014081610160840151169201511690039967ffffffffffffffff8b11611c25576116529f67ffffffffffffffff60249c8f929861164d9f997f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b9a846101a0610180880151970151975160208151910120995116996040519b60208d019d8e5260408d015260608c015260808b015260a08a015260c08901521660e08701526101008601526101208501526101408401526101608301526101808201526101808152611a626101a082612252565b51902091611a7360a0890151613e3d565b90518061016095611a876040519788612252565b61012987527f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208801527f75696e743820627269646765547970652c75696e74333220647374436861696e60408801527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608801527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808801527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08801527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08801527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08801527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008801527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208801527f3620616d6f756e74290000000000000000000000000000000000000000000000610140880152611c18604489018661216a565b99909801359435936141cd565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b34610275575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610275576020604051624c4b408152f35b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043567ffffffffffffffff811161027557806004016102007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc833603011261027557688000000000ab143c065c610b94576001688000000000ab143c065d60ff60035460401c16610b6c57611d94611d8b917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d611dcd611d9c611da36101c4602098611d6933886125d9565b9b849d829993949692959c611d808730338b6127e2565b6101e484019061216a565b979092016121bb565b9536916122cd565b9289612849565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d604051908152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261027557611e82612009565b688000000000ab143c065c610b94576001688000000000ab143c065d335f525f60205260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5260205260405f20548015611f6057602091335f525f835260405f2073ffffffffffffffffffffffffffffffffffffffff82165f5283525f6040812055611f09823383612438565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd3992683843392a35f688000000000ab143c065d604051908152f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b346102755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102755760043580151580910361027557611fcc612401565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff00000000000000006003549260401b169116176003555f80f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b6064359073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b6004359063ffffffff8216820361027557565b359063ffffffff8216820361027557565b9181601f840112156102755782359167ffffffffffffffff8311610275576020838186019501011161027557565b60a4359067ffffffffffffffff8216820361027557565b359067ffffffffffffffff8216820361027557565b600311156120fa57565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610275570180359067ffffffffffffffff82116102755760200191813603831361027557565b3567ffffffffffffffff811681036102755790565b6060810190811067ffffffffffffffff8211176121ec57604052565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610200810190811067ffffffffffffffff8211176121ec57604052565b6040810190811067ffffffffffffffff8211176121ec57604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176121ec57604052565b67ffffffffffffffff81116121ec57601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b9291926122d982612293565b916122e76040519384612252565b829481845281830111610275578281602093845f960137010152565b519073ffffffffffffffffffffffffffffffffffffffff8216820361027557565b908160409103126102755761235260206040519261234184612236565b61234a81612303565b845201612303565b602082015290565b91908203918211611c2557565b5190811515820361027557565b519063ffffffff8216820361027557565b908160a0910312610275576040519060a0820182811067ffffffffffffffff8211176121ec576123f9916080916040526123be81612367565b84526123cc60208201612374565b60208501526123dd60408201612374565b60408501526123ee60608201612303565b606085015201612303565b608082015290565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392754330361242b57565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f51141615612474575b50505f603452565b3b153d171015612485575f8061246c565b6390b8ec185f526004601cfd5b3560ff811681036102755790565b3563ffffffff811681036102755790565b91906102008382031261027557604051906124cb82612219565b8193803560ff811681036102755783526124e760208201612085565b60208401526124f860408201612085565b6040840152606081013560608401526080810135608084015260a081013560a084015260c081013560c084015260e081013560e084015261010081013561010084015261254861012082016120db565b61012084015261255b61014082016120db565b61014084015261256e61016082016120db565b6101608401526101808101356101808401526101a08101356101a08401526125996101c082016120db565b6101c08401526101e08101359067ffffffffffffffff8211610275570181601f82011215610275576101e0918160206125d4933591016122cd565b910152565b90600160ff6125e784612492565b16036127a6576040820163ffffffff6125ff826124a0565b16460361276c575061261961261436846124b1565b613b5a565b92835f52600160205260405f209283549160ff8360a01c1661263a816120f0565b6127405761265061264b36846124b1565b613d25565b9461265e6080840135613e3d565b947fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff740100000000000000000000000000000000000000006126dd6126d6610100880135976126b061014082016121bb565b6126bd61016083016121bb565b600254916101806101a08501359401359142918d61399d565b809761235a565b967fffffffffffffffffffffff00000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff79ffffffffff0000000000000000000000000000000000000000004260a81b1695169116171716179055565b857f343e211e000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b61277a63ffffffff916124a0565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b60ff6127b183612492565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f5260016004521660245260445ffd5b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f5114161561282b575b50505f606052604052565b3b153d17101561283c575f80612820565b637939f4245f526004601cfd5b9161287395939181958051155f14612875575060405161286a602082612252565b5f815293613eb8565b565b61290d6128e1916040519283917f3866c1dc00000000000000000000000000000000000000000000000000000000602084015288602484015273ffffffffffffffffffffffffffffffffffffffff861660448401528760648401526080608484015260a4830190612127565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282612252565b93613eb8565b9067ffffffffffffffff8091169116019067ffffffffffffffff8211611c2557565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff8111611c255767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b6040519061299982612219565b60606101e0835f81525f60208201525f60408201525f838201525f60808201525f60a08201525f60c08201525f60e08201525f6101008201525f6101208201525f6101408201525f6101608201525f6101808201525f6101a08201525f6101c08201520152565b94939197929097612a0f61298c565b50604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff8116602483015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169290919081604481865afa918215610a5c5773ffffffffffffffffffffffffffffffffffffffff6020612b25946040945f91612c7b575b50015116938d83518095819482937f320005c50000000000000000000000000000000000000000000000000000000084526004840190929160ff60209163ffffffff604085019616845216910152565b03915afa908115610a5c5773ffffffffffffffffffffffffffffffffffffffff916020915f91612c5c575b50015116918115610b0b578215612c2a574267ffffffffffffffff1696612b779088612913565b976040519b612b858d612219565b60018d528c4663ffffffff16906020015263ffffffff1660408d015273ffffffffffffffffffffffffffffffffffffffff1660608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015261014085015267ffffffffffffffff166101608401526101808301526101a08201526101c081015f9052604051612c1e602082612252565b5f81526101e082015290565b63ffffffff8b7fb825dd76000000000000000000000000000000000000000000000000000000005f521660045260245ffd5b612c75915060403d604011610b6557610b578183612252565b5f612b50565b612c929150853d8711610b6557610b578183612252565b5f612ad5565b63ffffffff602082015116468103612ff8575063ffffffff60408201511673ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000166040517f0a70b05600000000000000000000000000000000000000000000000000000000815282600482015260a081602481855afa908115610a5c575f91612fd9575b50604080517f320005c500000000000000000000000000000000000000000000000000000000815263ffffffff851660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015292839060449082905afa918215610a5c575f92612fb8575b505115908115612f98575b8115612f75575b50612f4a575067ffffffffffffffff6101608201511667ffffffffffffffff6101408301511680821115612f1c5750506101008101805180158015612f0f575b612edb57506101a082015190519081811015612ead57505073ffffffffffffffffffffffffffffffffffffffff612e316080830151613e3d565b1615612e85576101c0015167ffffffffffffffff16624c4b408111612e535750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f52600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e0830151907f8dd09d91000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5060e08301518111612df7565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b7fb825dd76000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b73ffffffffffffffffffffffffffffffffffffffff91506020015116155f612db7565b805173ffffffffffffffffffffffffffffffffffffffff16159150612db0565b612fd291925060403d604011610b6557610b578183612252565b905f612da5565b612ff2915060a03d60a011610a5557610a478183612252565b5f612d30565b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b929161302e84613b5a565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff166024820152919573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016929182604481865afa918215610a5c575f926138ee575b5073ffffffffffffffffffffffffffffffffffffffff82511693604051927ffc0c546a000000000000000000000000000000000000000000000000000000008452602084600481895afa938415610a5c575f9461389a575b50602073ffffffffffffffffffffffffffffffffffffffff9101511673ffffffffffffffffffffffffffffffffffffffff841681810361386c5750506040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a081602481885afa908115610a5c5763ffffffff916040915f9161384d575b500151166040517f5e280f110000000000000000000000000000000000000000000000000000000081526020816004818a5afa908115610a5c575f916137fb575b50602073ffffffffffffffffffffffffffffffffffffffff916004604051809481937f416ecebf000000000000000000000000000000000000000000000000000000008352165afa8015610a5c575f906137be575b63ffffffff9150169080820361379057505060e0820192835186601452806034526f095ea7b30000000000000000000000005f5260205f6044601082865af18060015f51141615613724575b5050505f603452604082019360a063ffffffff8651166024604051809481937f0a70b05600000000000000000000000000000000000000000000000000000000835260048301525afa908115610a5c5763ffffffff916040915f91613705575b500151169583519561010084019283519860405160208082015260ff875116604082015263ffffffff602088015116606082015263ffffffff89511660808201526060870194855160a0830152608088015160c083015260a088015160e083015260c0880198895161010084015251610120830152865161014083015261342f826134036101e06101208c019b67ffffffffffffffff8d511661016085015267ffffffffffffffff6101408201511661018085015267ffffffffffffffff610160820151166101a08501526101808101516101c08501526101a08101518285015267ffffffffffffffff6101c0820151166102008501520151610200610220840152610240830190612127565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101845283612252565b6040519260e0840184811067ffffffffffffffff8211176121ec5760405283526020830193308552604084019b8c52606084019c8d523690613470926122cd565b9a608083019b8c5260a0830191825260209b8c92604051916134928584612252565b5f835260c08601928352604051946134a986612236565b3486528501935f85526040519e8f9889987fc7c7f5b3000000000000000000000000000000000000000000000000000000008a5260048a01608090525163ffffffff1660848a01525160a48901525160c48801525160e487015251610104860160e09052610164860161351b91612127565b9051908581037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7c0161012487015261355291612127565b9051908481037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7c0161014486015261358991612127565b915160248401525160448301523360648301520381345a9460c095f1958615610a5c578996613632575b509273ffffffffffffffffffffffffffffffffffffffff927f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a959267ffffffffffffffff63ffffffff61360860a09851613e3d565b95511693519151925116926040519960018b528a01526040890152606088015260808701521693a3565b93909491955060c03d60c0116136fe575b61364d8186612252565b84019284840360c081126102755760801361027557899667ffffffffffffffff63ffffffff6136087f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a996136ee73ffffffffffffffffffffffffffffffffffffffff9960808f9c60a09d604051906136c4826121d0565b825182526136d3818401613b45565b9082015260406136e5858285016143a6565b910152016143a6565b50959850505050929550926135b3565b503d613643565b61371e915060a03d60a011610a5557610a478183612252565b5f6132f6565b3d833b15171015613736575b80613296565b5f6034526f095ea7b30000000000000000000000005f525f386044601083865af15060345260205f6044601082855af1908160015f511416613730573b153d171015613783575f80613730565b633e3f8f735f526004601cfd5b7fb9190bb3000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b506020813d6020116137f3575b816137d860209383612252565b81010312610275576137ee63ffffffff91612374565b61324a565b3d91506137cb565b90506020813d602011613845575b8161381660209383612252565b8101031261027557602061383e73ffffffffffffffffffffffffffffffffffffffff92612303565b91506131f5565b3d9150613809565b613866915060a03d60a011610a5557610a478183612252565b5f6131b4565b7ff902523f000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b9093506020813d6020116138e6575b816138b660209383612252565b810103126102755760206138de73ffffffffffffffffffffffffffffffffffffffff92612303565b94915061312c565b3d91506138a9565b61390891925060403d604011610b6557610b578183612252565b905f6130d4565b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b91908201809211611c2557565b959290939167ffffffffffffffff8091169416908181105f14613b3e57505b5f938082106139e4575b5050506139d39250613990565b818111156139df575090565b905090565b6139f09293945061235a565b90801580159081613ae0575b5015613ad9575050815b808311613ad1575b5081830291670de0b6b3a764000084158286860414170215613a435750670de0b6b3a76400006139d392045b905f80806139c6565b91670de0b6b3a76400007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8486098281108301900393850983670de0b6b3a76400001115613ac4576139d393828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066902613a3a565b63ae47f7025f526004601cfd5b91505f613a0e565b0291613a06565b9050613b1157807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0482115f6139fc565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b90506139bc565b519067ffffffffffffffff8216820361027557565b906101e08201805151601f8101809111611c255760051c908160120180601211611c2557601360405193849283520160051b0160405267ffffffffffffffff6101c060208401956020875260ff815116604086015263ffffffff602082015116606086015263ffffffff6040820151166080860152606081015160a0860152608081015160c086015260a081015160e086015260c081015161010086015260e08101516101208601526101008101516101408601528261012082015116610160860152826101408201511661018086015282610160820151166101a0860152610180810151828601526101a08101516101e086015201511661020083015261020061022083015280515161024083015251805180613c95575b50508051928360051b902092604051828260010160051b011490151060061b52565b602082016020826102608601940101905b818110613d15575050601f16908115613c735760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80613c73565b8051845260209384019301613ca6565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa8015610a5c57602073ffffffffffffffffffffffffffffffffffffffff9160c0935f91613e1e575b50015116910151818103613df3575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b613e37915060403d604011610b6557610b578183612252565b5f613de1565b8060a01c613e5e5773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b3d15613eb3573d90613e9a82612293565b91613ea86040519384612252565b82523d5f602084013e565b606090565b945f9484156141c35780511580156141ba575b6141a9575a61c35067ffffffffffffffff613ef7818616956707ffffffffffffff8160051c1690612913565b160167ffffffffffffffff8111611c255767ffffffffffffffff161161417957303b15610275575f613f9091604051809381927fac4fca8200000000000000000000000000000000000000000000000000000000835273ffffffffffffffffffffffffffffffffffffffff808916998a600486015216968760248501528a6044850152606484015260a0608484015260a4830190612127565b038183305af19081614164575b506141325750613fab613e89565b91849260248151146140c1575b5073ffffffffffffffffffffffffffffffffffffffff831696871515806140b7575b1561401c5750506020927f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9949261401192876143ce565b5060405160018152a3565b602097507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a995935087949192507f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f73ffffffffffffffffffffffffffffffffffffffff88921698899384875286825260408720865f52825260405f206140a3828254613990565b9055604051908152a45060405160028152a3565b5030881415613fda565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff0000000000000000000000000000000000000000000000000000000060246020840151930151921603613fb85773ffffffffffffffffffffffffffffffffffffffff1692505f613fb8565b955050505060207f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a991604051908152a3565b6141719196505f90612252565b5f945f613f9d565b505a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b5050919390925061287394506143ce565b50833b15613ecb565b5050505050505050565b9590929893969197949773ffffffffffffffffffffffffffffffffffffffff604051976141f989612236565b16875260208701526040519561420e876121d0565b865260208601968752604086019788526040519861422b8a612236565b308a5260208a01526e22d473030f116ddee9f6b43ac78ba33b15610275578392604051998a998a998a997f137c29fe000000000000000000000000000000000000000000000000000000008b5260048b019051906142a8916020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b5160448a0152516064890152805173ffffffffffffffffffffffffffffffffffffffff1660848901526020015160a488015273ffffffffffffffffffffffffffffffffffffffff1660c487015260e486015261010485016101409052610144850161431291612127565b928484037ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0161012486015281845260208401378082016020015f9052601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01601036020015a925f6e22d473030f116ddee9f6b43ac78ba38195f18015610a5c5761439c5750565b5f61287391612252565b9190826040910312610275576040516143be81612236565b6020808294805184520151910152565b9083156144b4575f806040519473ffffffffffffffffffffffffffffffffffffffff60208701917fa9059cbb000000000000000000000000000000000000000000000000000000008352169586602482015287604482015260448152614435606482612252565b519082855af1614443613e89565b816144ba575b506144b457602073ffffffffffffffffffffffffffffffffffffffff7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f92855f525f835260405f208282165f52835260405f206144a7888254613990565b90556040519687521694a4565b50505050565b80518015925082156144cf575b50505f614449565b81925090602091810103126102755760206144ea9101612367565b5f806144c756fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_\x805`\xE0\x1C\x80c\x16\xC3\x8B<\x14a\x1F\x88W\x80c\x1E\x83@\x9A\x14a\x1EKW\x80c%i)b\x14a\x1D\xE4W\x80c1\xEE\xE4M\x14a\x1C\x8DW\x80c9\xC32\x15\x14a\x1CRW\x80cJk\xE4h\x14a\x17\xCBW\x80cT\xD1\xF1=\x14a\x17iW\x80cWxG*\x14a\x16\x8CW\x80cYu\xE7\x90\x14a\x14oW\x80c\\\x97Z\xBB\x14a\x14,W\x80cj\xFD\xD8P\x14a\x13\xE5W\x80cqP\x18\xA6\x14a\x13HW\x80csv\xF1\xC0\x14a\x12\xEDW\x80cwo\xF3\xC7\x14a\x105W\x80cyP,U\x14a\x0F\xC7W\x80c\x85\xC1x0\x14a\x0F\x8CW\x80c\x8C\xDA\x96\xDE\x14a\x0F\x13W\x80c\x8D\xA5\xCB[\x14a\x0E\xA3W\x80c\x97\xC3k\xAE\x14a\r\xDCW\x80c\xACO\xCA\x82\x14a\x0C\xEBW\x80c\xAC\x96P\xD8\x14a\x0B\xA1W\x80c\xD0\xA1\x02`\x14a\x03\xC8W\x80c\xD4W\x0C\x1C\x14a\x03=W\x80c\xF0N(>\x14a\x02\xD2W\x80c\xF2\xFD\xE3\x8B\x14a\x02yW\x80c\xF3\x99\\g\x14a\x01\x96Wc\xFE\xE8\x1C\xF4\x14a\x01CW_\x80\xFD[4a\x01\x93W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\x93Wa\x01za \tV[\x90c8\x9Au\xE1`\x0CRR` \x80`\x0C T`@Q\x90\x81R\xF3[\x80\xFD[P4a\x02uW`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x01\xCEa \tV[`d5\x90`\xFF\x82\x16\x80\x92\x03a\x02uWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81;\x15a\x02uW_\x91`\xE4\x83\x92`@Q\x94\x85\x93\x84\x92\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R3`\x04\x85\x01R0`$\x85\x01R`$5`D\x85\x01R`D5`d\x85\x01R`\x84\x84\x01R`\x845`\xA4\x84\x01R`\xA45`\xC4\x84\x01RZ\xF1a\x02gWP\x80\xF3[a\x02s\x91P_\x90a\"RV[\0[_\x80\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x02\xABa \tV[a\x02\xB3a$\x01V[\x80``\x1B\x15a\x02\xC5Wa\x02s\x90a9\x0FV[ctH\xFB\xAE_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x03\x04a \tV[a\x03\x0Ca$\x01V[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x90\x81TB\x11a\x030W_a\x02s\x92Ua9\x0FV[co^\x88\x18_R`\x04`\x1C\xFD[4a\x02uW`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x03ta \tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x03\x91a ,V[\x91\x16_R_` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x03\xFAa \tV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x04\x1A\x906\x90`\x04\x01a \x96V[\x91\x90a\x04$a OV[P`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x04E\x906\x90`\x04\x01a \x96V[PPh\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x0BlW`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x91\x81`D\x81\x87Z\xFA\x80\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91_\x91a\x0B=W[PQ\x16\x90\x81\x15a\x0B\x0BW`@Q\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x86Z\xFA\x80\x15a\n\\W_\x90a\n\xBEW[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P\x163\x03a\n\x92Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x03a\ngWP\x82`\x0C\x11a\x02uW`\x08\x81\x015`\xE0\x1C\x91\x83`L\x11a\x02uW`,\x82\x015\x93\x80`,\x11a\x02uWa\x06\t\x90`\x0C\x84\x015\x93`L\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xB46\x93\x01\x91\x01a\"\xCDV[\x92a\x06\x12a)\x8CV[P\x83Q\x84\x01\x93` \x85\x01\x90` \x81\x87\x03\x12a\x02uW` \x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x01\x94a\x02\0\x90\x86\x90\x03\x12a\x02uW`@Q\x91a\x06V\x83a\"\x19V[` \x86\x01Q`\xFF\x81\x16\x81\x03a\x02uW\x83Ra\x06s`@\x87\x01a#tV[\x96` \x84\x01\x97\x88Ra\x06\x87``\x88\x01a#tV[\x91`@\x85\x01\x92\x83R`\x80\x88\x01Q``\x86\x01R`\xA0\x88\x01Q\x95`\x80\x86\x01\x96\x87R`\xC0\x89\x01Q`\xA0\x87\x01R`\xE0\x89\x01Q`\xC0\x87\x01Ra\x01\0\x89\x01Q`\xE0\x87\x01Ra\x01 \x89\x01Q\x99a\x01\0\x87\x01\x9A\x8BRa\x06\xE1a\x01@\x8B\x01a;EV[a\x01 \x88\x01Ra\x06\xF4a\x01`\x8B\x01a;EV[a\x01@\x88\x01Ra\x07\x07a\x01\x80\x8B\x01a;EV[a\x01`\x88\x01Ra\x01\xA0\x8A\x01Qa\x01\x80\x88\x01Ra\x01\xC0\x8A\x01Qa\x01\xA0\x88\x01Ra\x072a\x01\xE0\x8B\x01a;EV[\x99a\x01\xC0\x88\x01\x9A\x8BRa\x02\0\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW` \x91\x01\x01\x86`\x1F\x82\x01\x12\x15a\x02uW\x80Q\x91a\x07l\x83a\"\x93V[\x97a\x07z`@Q\x99\x8Aa\"RV[\x83\x89R` \x84\x84\x01\x01\x11a\x02uW_` \x84`\xA0\x95\x82c\xFF\xFF\xFF\xFF\x96\x01\x83\x8D\x01^\x8A\x01\x01Ra\x01\xE0\x89\x01\x97\x88RQ\x16`$`@Q\x80\x94\x81\x93\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\n\\W_\x91a\n-W[P\x81\x81Q\x15\x91\x82\x15a\n\x18W[PPa\t\xEDWP0\x81\x03a\t\xC2WPc\xFF\xFF\xFF\xFF\x90Q\x16\x80F\x03a\t\x97WPa\x08\x1F\x82a;ZV[\x94\x85_R`\x01` R`@_ \x90\x81T\x93`\x02`\xFF\x86`\xA0\x1C\x16a\x08B\x81a \xF0V[\x14a\tkW\x92`@\x95\x92\x87\x95\x92\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x98a\x08\xA5a\x08\x9E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x98a=%V[\x96Qa>=V[\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x9CQ\x80\x83\x10_\x14a\teWP\x81\x93[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x08\xEC\x86\x85a#ZV[\x99\x8A\x98\x16\x17\x90U\x8C\x15\x15_\x14a\t@WPPPa\t\x0B\x90\x8A\x85\x8BaC\xCEV[\x81a\t.W[PPP[\x82Q\x91\x82R` \x82\x01R\xA3_h\x80\0\0\0\0\xAB\x14<\x06]\0[a\t8\x92\x88aC\xCEV[\x86\x81\x81a\t\x11V[a\t`\x96\x95Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x93P\x93\x91\x93Q\x93Q\x16\x93\x8Aa(IV[a\t\x15V[\x93a\x08\xCCV[\x87\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F%\xEA#\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xE7=#\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`@\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x15\x90P\x81\x8Ba\x07\xF7V[a\nO\x91P`\xA0=`\xA0\x11a\nUW[a\nG\x81\x83a\"RV[\x81\x01\x90a#\x85V[\x8Aa\x07\xEAV[P=a\n=V[`@Q=_\x82>=\x90\xFD[\x7F\x89#:r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F9\xE7\xE9K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[P` \x81=` \x11a\x0B\x03W[\x81a\n\xD8` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uWa\n\xFEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a#\x03V[a\x05oV[=\x91Pa\n\xCBV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFFF\x16`\x04R`$_\xFD[a\x0B_\x91P`@=`@\x11a\x0BeW[a\x0BW\x81\x83a\"RV[\x81\x01\x90a#$V[\x86a\x05)V[P=a\x0BMV[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW6`#\x82\x01\x12\x15a\x02uW\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x05\x1B6`$\x82\x85\x01\x01\x11a\x02uW4a\x02uW`@Q\x92\x83\x92` \x84R\x80\x84` \x01R`@\x84\x81\x01\x93\x80`$\x85\x01\x867\x85\x01\x01\x92\x83\x91a\x0CVW[PPP\x80`@R\x03`@\x1B\x17\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[\x91\x93P\x91[\x80\x91_`D\x82Q\x87\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x0C\xE2W` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?_\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8A\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x93=\x01\x01R\x84\x83\x82\x10\x15a\x0C\xD5WP\x90a\x0C[V[\x93PP\x90P\x83\x80\x80a\x0C9V[\x85=_\x82>=\x90\xFD[4a\x02uW`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\r\"a \tV[a\r*a ,V[\x90`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\rK\x906\x90`\x04\x01a \x96V[\x91\x9003\x03a\r\xB4W_\x92a\rga\rn\x93\x86`D5\x91a$8V[6\x91a\"\xCDV[\x91`@Q\x928\x91\x83` \x83Q\x93\x01\x91`d5\xF1=`$=\x11a\r\xACW[\x80` \x91\x84R\x80_\x83\x86\x01>\x83\x01\x01`@R\x15a\r\xA4W\0[\x80Q\x90` \x01\xFD[P`$a\r\x8BV[\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02uW`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x826\x03\x01\x12a\x02uW\x80a\x0E\x96a\x0E\x8Fa\x01\x04`@\x94\x015\x92a\x0Ega\x01D\x82\x01a!\xBBV[a\x0Eta\x01d\x83\x01a!\xBBV[`\x02T\x91a\x01\x84a\x01\xA4\x85\x015\x94\x015\x91`$5\x91\x88a9\x9DV[\x80\x92a#ZV[\x90\x82Q\x91\x82R` \x82\x01R\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045a\x0FMa$\x01V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x0FaW`\x02U\0[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `\x02T`@Q\x90\x81R\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x02uW``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x02uWa\x10\xAFa ,V[\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW\x81`\x04\x01\x93``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x02uWh\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x0BlWa\x11'\x81\x85a%\xD9V[\x97\x83\x85\x9A\x83\x98\x95\x96\x99`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\x11p``\x82a\"RV[Q\x90 `@Q\x90a\x11\x82`\xA0\x83a\"RV[`j\x82R\x7FFillAuthorization witness)FillAu` \x83\x01R\x7Fthorization(bytes32 orderId)Toke`@\x83\x01R\x7FnPermissions(address token,uint2``\x83\x01R\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x83\x01Ra\x12+`D\x84\x01\x85a!jV[\x94\x90\x93`$\x015\x905\x88\x8Da\x01\x04\x8D\x015\x90a\x12F\x99aA\xCDV[a\x01\xE4\x84\x01a\x12T\x91a!jV[\x93a\x01\xC4\x01a\x12b\x90a!\xBBV[\x936\x90a\x12n\x92a\"\xCDV[\x91a\x12y\x94\x88a(IV[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x13ya$\x01V[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\0[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[a\x01 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x14\xA2a rV[`D5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x14\xC5\x906\x90`\x04\x01a \x96V[\x90a\x14\xCEa \xC4V[\x93a\x01\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW\x81`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x846\x03\x01\x12a\x02uW`\xFF`\x03T`@\x1C\x16a\x0BlWa\x15p\x91`$a\x15Oa\rg\x93a\x156a)5V[\x9A\x8B\x91`\xE45\x91`\xC45\x91\x8D`d5\x91\x885\x903a*\0V[\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x15b\x84a!\xBBV[\x16a\x01\xC0\x87\x01R\x01\x90a!jV[a\x01\xE0\x82\x01Ra\x15\x7F\x81a,\x98V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x94\x85\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` a\x16R\x97a\x16M\x94_\x91a\x16mW[P\x01Q\x160\x903\x90a'\xE2V[a0#V[`@\x80Q\x91\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x82\x01R\xF3[a\x16\x86\x91P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[\x8Aa\x16@V[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW_`@\x80Qa\x16\xC9\x81a!\xD0V[\x82\x81R\x82` \x82\x01R\x01R`\x045_R`\x01` R```@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x16\xF7\x83a!\xD0V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16` \x84\x01\x90a\x17&\x81a \xF0V[\x81R\x82`@\x85\x01\x92`\xA8\x1C\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x94Q\x16\x84RQa\x17[\x81a \xF0V[` \x84\x01RQ\x16`@\x82\x01R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[a\x01`\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x17\xFEa rV[`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uWa\x18\x1E\x906\x90`\x04\x01a \x96V[a\x18)\x92\x91\x92a \xC4V[\x92a\x01\x045\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\x02uW\x83`\x04\x01\x90`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x866\x03\x01\x12a\x02uWa\x01$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWa\x01D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x04\x01\x92``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x02uW`\xFF`\x03T`@\x1C\x16a\x0BlWa\x197a\x19\x11a\x19F\x94a\x18\xF4a)5V[\x9B\x8C\x91`\xE45\x91`\xC45\x91`d5\x90`D5\x90`$5\x90\x8Aa*\0V[\x95`$a\x19\x1D\x82a!\xBBV[\x9Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x8A\x01\x9C\x16\x8CR\x01\x90a!jV[\x93\x90a\x01\xE0\x87\x01\x946\x91a\"\xCDV[\x83Ra\x19Q\x85a,\x98V[a\x19\\6\x88\x88a\"\xCDV[` \x81Q\x91\x01 `\xFF\x86Q\x16\x94c\xFF\xFF\xFF\xFF`@\x88\x01Q\x16\x91`\x80\x88\x01Q\x96`\xE0\x89\x01\x93\x84Q\x9Ca\x01\0\x8B\x01Q\x93\x8Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x81a\x01`\x84\x01Q\x16\x92\x01Q\x16\x90\x03\x99g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x11a\x1C%Wa\x16R\x9Fg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x9C\x8F\x92\x98a\x16M\x9F\x99\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x9A\x84a\x01\xA0a\x01\x80\x88\x01Q\x97\x01Q\x97Q` \x81Q\x91\x01 \x99Q\x16\x99`@Q\x9B` \x8D\x01\x9D\x8ER`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Ra\x01\x80\x82\x01Ra\x01\x80\x81Ra\x1Aba\x01\xA0\x82a\"RV[Q\x90 \x91a\x1As`\xA0\x89\x01Qa>=V[\x90Q\x80a\x01`\x95a\x1A\x87`@Q\x97\x88a\"RV[a\x01)\x87R\x7FOrderIntent witness)OrderIntent(` \x88\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x88\x01R\x7FId,bytes32 recipient,uint256 inp``\x88\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x88\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x88\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x88\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x88\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x88\x01R\x7FPermissions(address token,uint25a\x01 \x88\x01R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x88\x01Ra\x1C\x18`D\x89\x01\x86a!jV[\x99\x90\x98\x015\x945\x93aA\xCDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[4a\x02uW_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW` `@QbLK@\x81R\xF3[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02uW\x80`\x04\x01a\x02\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x836\x03\x01\x12a\x02uWh\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x0BlWa\x1D\x94a\x1D\x8B\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x1D\xCDa\x1D\x9Ca\x1D\xA3a\x01\xC4` \x98a\x1Di3\x88a%\xD9V[\x9B\x84\x9D\x82\x99\x93\x94\x96\x92\x95\x9Ca\x1D\x80\x8703\x8Ba'\xE2V[a\x01\xE4\x84\x01\x90a!jV[\x97\x90\x92\x01a!\xBBV[\x956\x91a\"\xCDV[\x92\x89a(IV[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uWa\x1E\x82a \tV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x0B\x94W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R_` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R` R`@_ T\x80\x15a\x1F`W` \x913_R_\x83R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R\x83R_`@\x81 Ua\x1F\t\x823\x83a$8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83\x843\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x02uW` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x02uW`\x045\x80\x15\x15\x80\x91\x03a\x02uWa\x1F\xCCa$\x01V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`d5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x91\x81`\x1F\x84\x01\x12\x15a\x02uW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x02uW` \x83\x81\x86\x01\x95\x01\x01\x11a\x02uWV[`\xA45\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[`\x03\x11\x15a \xFAWV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x02uW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW` \x01\x91\x816\x03\x83\x13a\x02uWV[5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x02uW\x90V[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x02\0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a!\xECW`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a\"\xD9\x82a\"\x93V[\x91a\"\xE7`@Q\x93\x84a\"RV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02uW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[Q\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x90\x81`@\x91\x03\x12a\x02uWa#R` `@Q\x92a#A\x84a\"6V[a#J\x81a#\x03V[\x84R\x01a#\x03V[` \x82\x01R\x90V[\x91\x90\x82\x03\x91\x82\x11a\x1C%WV[Q\x90\x81\x15\x15\x82\x03a\x02uWV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x90\x81`\xA0\x91\x03\x12a\x02uW`@Q\x90`\xA0\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECWa#\xF9\x91`\x80\x91`@Ra#\xBE\x81a#gV[\x84Ra#\xCC` \x82\x01a#tV[` \x85\x01Ra#\xDD`@\x82\x01a#tV[`@\x85\x01Ra#\xEE``\x82\x01a#\x03V[``\x85\x01R\x01a#\x03V[`\x80\x82\x01R\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a$+WV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a$tW[PP_`4RV[;\x15=\x17\x10\x15a$\x85W_\x80a$lV[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5`\xFF\x81\x16\x81\x03a\x02uW\x90V[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x02uW\x90V[\x91\x90a\x02\0\x83\x82\x03\x12a\x02uW`@Q\x90a$\xCB\x82a\"\x19V[\x81\x93\x805`\xFF\x81\x16\x81\x03a\x02uW\x83Ra$\xE7` \x82\x01a \x85V[` \x84\x01Ra$\xF8`@\x82\x01a \x85V[`@\x84\x01R``\x81\x015``\x84\x01R`\x80\x81\x015`\x80\x84\x01R`\xA0\x81\x015`\xA0\x84\x01R`\xC0\x81\x015`\xC0\x84\x01R`\xE0\x81\x015`\xE0\x84\x01Ra\x01\0\x81\x015a\x01\0\x84\x01Ra%Ha\x01 \x82\x01a \xDBV[a\x01 \x84\x01Ra%[a\x01@\x82\x01a \xDBV[a\x01@\x84\x01Ra%na\x01`\x82\x01a \xDBV[a\x01`\x84\x01Ra\x01\x80\x81\x015a\x01\x80\x84\x01Ra\x01\xA0\x81\x015a\x01\xA0\x84\x01Ra%\x99a\x01\xC0\x82\x01a \xDBV[a\x01\xC0\x84\x01Ra\x01\xE0\x81\x015\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x02uW\x01\x81`\x1F\x82\x01\x12\x15a\x02uWa\x01\xE0\x91\x81` a%\xD4\x935\x91\x01a\"\xCDV[\x91\x01RV[\x90`\x01`\xFFa%\xE7\x84a$\x92V[\x16\x03a'\xA6W`@\x82\x01c\xFF\xFF\xFF\xFFa%\xFF\x82a$\xA0V[\x16F\x03a'lWPa&\x19a&\x146\x84a$\xB1V[a;ZV[\x92\x83_R`\x01` R`@_ \x92\x83T\x91`\xFF\x83`\xA0\x1C\x16a&:\x81a \xF0V[a'@Wa&Pa&K6\x84a$\xB1V[a=%V[\x94a&^`\x80\x84\x015a>=V[\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&\xDDa&\xD6a\x01\0\x88\x015\x97a&\xB0a\x01@\x82\x01a!\xBBV[a&\xBDa\x01`\x83\x01a!\xBBV[`\x02T\x91a\x01\x80a\x01\xA0\x85\x015\x94\x015\x91B\x91\x8Da9\x9DV[\x80\x97a#ZV[\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFy\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0B`\xA8\x1B\x16\x95\x16\x91\x16\x17\x17\x16\x17\x90UV[\x85\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a'zc\xFF\xFF\xFF\xFF\x91a$\xA0V[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[`\xFFa'\xB1\x83a$\x92V[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R\x16`$R`D_\xFD[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a(+W[PP_``R`@RV[;\x15=\x17\x10\x15a(<W_\x80a( V[cy9\xF4$_R`\x04`\x1C\xFD[\x91a(s\x95\x93\x91\x81\x95\x80Q\x15_\x14a(uWP`@Qa(j` \x82a\"RV[_\x81R\x93a>\xB8V[V[a)\ra(\xE1\x91`@Q\x92\x83\x91\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01R\x88`$\x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`D\x84\x01R\x87`d\x84\x01R`\x80`\x84\x84\x01R`\xA4\x83\x01\x90a!'V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\"RV[\x93a>\xB8V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x1C%WV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1C%Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[`@Q\x90a)\x99\x82a\"\x19V[``a\x01\xE0\x83_\x81R_` \x82\x01R_`@\x82\x01R_\x83\x82\x01R_`\x80\x82\x01R_`\xA0\x82\x01R_`\xC0\x82\x01R_`\xE0\x82\x01R_a\x01\0\x82\x01R_a\x01 \x82\x01R_a\x01@\x82\x01R_a\x01`\x82\x01R_a\x01\x80\x82\x01R_a\x01\xA0\x82\x01R_a\x01\xC0\x82\x01R\x01RV[\x94\x93\x91\x97\x92\x90\x97a*\x0Fa)\x8CV[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x81\x16`$\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x90\x91\x90\x81`D\x81\x86Z\xFA\x91\x82\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` a+%\x94`@\x94_\x91a,{W[P\x01Q\x16\x93\x8D\x83Q\x80\x95\x81\x94\x82\x93\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\xFF` \x91c\xFF\xFF\xFF\xFF`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\n\\Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91` \x91_\x91a,\\W[P\x01Q\x16\x91\x81\x15a\x0B\x0BW\x82\x15a,*WBg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x96a+w\x90\x88a)\x13V[\x97`@Q\x9Ba+\x85\x8Da\"\x19V[`\x01\x8DR\x8CFc\xFF\xFF\xFF\xFF\x16\x90` \x01Rc\xFF\xFF\xFF\xFF\x16`@\x8D\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Ra\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01Ra\x01\xC0\x81\x01_\x90R`@Qa,\x1E` \x82a\"RV[_\x81Ra\x01\xE0\x82\x01R\x90V[c\xFF\xFF\xFF\xFF\x8B\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[a,u\x91P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[_a+PV[a,\x92\x91P\x85=\x87\x11a\x0BeWa\x0BW\x81\x83a\"RV[_a*\xD5V[c\xFF\xFF\xFF\xFF` \x82\x01Q\x16F\x81\x03a/\xF8WPc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R`\xA0\x81`$\x81\x85Z\xFA\x90\x81\x15a\n\\W_\x91a/\xD9W[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x92\x83\x90`D\x90\x82\x90Z\xFA\x91\x82\x15a\n\\W_\x92a/\xB8W[PQ\x15\x90\x81\x15a/\x98W[\x81\x15a/uW[Pa/JWPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x83\x01Q\x16\x80\x82\x11\x15a/\x1CWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a/\x0FW[a.\xDBWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a.\xADWPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.1`\x80\x83\x01Qa>=V[\x16\x15a.\x85Wa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a.SWPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x90\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a-\xF7V[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91P` \x01Q\x16\x15_a-\xB7V[\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x91Pa-\xB0V[a/\xD2\x91\x92P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[\x90_a-\xA5V[a/\xF2\x91P`\xA0=`\xA0\x11a\nUWa\nG\x81\x83a\"RV[_a-0V[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x92\x91a0.\x84a;ZV[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x91\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x91\x82`D\x81\x86Z\xFA\x91\x82\x15a\n\\W_\x92a8\xEEW[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82Q\x16\x93`@Q\x92\x7F\xFC\x0CTj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R` \x84`\x04\x81\x89Z\xFA\x93\x84\x15a\n\\W_\x94a8\x9AW[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x01Q\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x81\x03a8lWPP`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x88Z\xFA\x90\x81\x15a\n\\Wc\xFF\xFF\xFF\xFF\x91`@\x91_\x91a8MW[P\x01Q\x16`@Q\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x8AZ\xFA\x90\x81\x15a\n\\W_\x91a7\xFBW[P` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\x04`@Q\x80\x94\x81\x93\x7FAn\xCE\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x80\x15a\n\\W_\x90a7\xBEW[c\xFF\xFF\xFF\xFF\x91P\x16\x90\x80\x82\x03a7\x90WPP`\xE0\x82\x01\x92\x83Q\x86`\x14R\x80`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x86Z\xF1\x80`\x01_Q\x14\x16\x15a7$W[PPP_`4R`@\x82\x01\x93`\xA0c\xFF\xFF\xFF\xFF\x86Q\x16`$`@Q\x80\x94\x81\x93\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01RZ\xFA\x90\x81\x15a\n\\Wc\xFF\xFF\xFF\xFF\x91`@\x91_\x91a7\x05W[P\x01Q\x16\x95\x83Q\x95a\x01\0\x84\x01\x92\x83Q\x98`@Q` \x80\x82\x01R`\xFF\x87Q\x16`@\x82\x01Rc\xFF\xFF\xFF\xFF` \x88\x01Q\x16``\x82\x01Rc\xFF\xFF\xFF\xFF\x89Q\x16`\x80\x82\x01R``\x87\x01\x94\x85Q`\xA0\x83\x01R`\x80\x88\x01Q`\xC0\x83\x01R`\xA0\x88\x01Q`\xE0\x83\x01R`\xC0\x88\x01\x98\x89Qa\x01\0\x84\x01RQa\x01 \x83\x01R\x86Qa\x01@\x83\x01Ra4/\x82a4\x03a\x01\xE0a\x01 \x8C\x01\x9Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8DQ\x16a\x01`\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01@\x82\x01Q\x16a\x01\x80\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01`\x82\x01Q\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0\x82\x01Q\x16a\x02\0\x85\x01R\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a!'V[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x84R\x83a\"RV[`@Q\x92`\xE0\x84\x01\x84\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a!\xECW`@R\x83R` \x83\x01\x930\x85R`@\x84\x01\x9B\x8CR``\x84\x01\x9C\x8DR6\x90a4p\x92a\"\xCDV[\x9A`\x80\x83\x01\x9B\x8CR`\xA0\x83\x01\x91\x82R` \x9B\x8C\x92`@Q\x91a4\x92\x85\x84a\"RV[_\x83R`\xC0\x86\x01\x92\x83R`@Q\x94a4\xA9\x86a\"6V[4\x86R\x85\x01\x93_\x85R`@Q\x9E\x8F\x98\x89\x98\x7F\xC7\xC7\xF5\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8AR`\x04\x8A\x01`\x80\x90RQc\xFF\xFF\xFF\xFF\x16`\x84\x8A\x01RQ`\xA4\x89\x01RQ`\xC4\x88\x01RQ`\xE4\x87\x01RQa\x01\x04\x86\x01`\xE0\x90Ra\x01d\x86\x01a5\x1B\x91a!'V[\x90Q\x90\x85\x81\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF|\x01a\x01$\x87\x01Ra5R\x91a!'V[\x90Q\x90\x84\x81\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF|\x01a\x01D\x86\x01Ra5\x89\x91a!'V[\x91Q`$\x84\x01RQ`D\x83\x01R3`d\x83\x01R\x03\x814Z\x94`\xC0\x95\xF1\x95\x86\x15a\n\\W\x89\x96a62W[P\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x95\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFc\xFF\xFF\xFF\xFFa6\x08`\xA0\x98Qa>=V[\x95Q\x16\x93Q\x91Q\x92Q\x16\x92`@Q\x99`\x01\x8BR\x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R\x16\x93\xA3V[\x93\x90\x94\x91\x95P`\xC0=`\xC0\x11a6\xFEW[a6M\x81\x86a\"RV[\x84\x01\x92\x84\x84\x03`\xC0\x81\x12a\x02uW`\x80\x13a\x02uW\x89\x96g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFc\xFF\xFF\xFF\xFFa6\x08\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x99a6\xEEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x99`\x80\x8F\x9C`\xA0\x9D`@Q\x90a6\xC4\x82a!\xD0V[\x82Q\x82Ra6\xD3\x81\x84\x01a;EV[\x90\x82\x01R`@a6\xE5\x85\x82\x85\x01aC\xA6V[\x91\x01R\x01aC\xA6V[P\x95\x98PPPP\x92\x95P\x92a5\xB3V[P=a6CV[a7\x1E\x91P`\xA0=`\xA0\x11a\nUWa\nG\x81\x83a\"RV[_a2\xF6V[=\x83;\x15\x17\x10\x15a76W[\x80a2\x96V[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x86Z\xF1P`4R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16a70W;\x15=\x17\x10\x15a7\x83W_\x80a70V[c>?\x8Fs_R`\x04`\x1C\xFD[\x7F\xB9\x19\x0B\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[P` \x81=` \x11a7\xF3W[\x81a7\xD8` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uWa7\xEEc\xFF\xFF\xFF\xFF\x91a#tV[a2JV[=\x91Pa7\xCBV[\x90P` \x81=` \x11a8EW[\x81a8\x16` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uW` a8>s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a#\x03V[\x91Pa1\xF5V[=\x91Pa8\tV[a8f\x91P`\xA0=`\xA0\x11a\nUWa\nG\x81\x83a\"RV[_a1\xB4V[\x7F\xF9\x02R?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90\x93P` \x81=` \x11a8\xE6W[\x81a8\xB6` \x93\x83a\"RV[\x81\x01\x03\x12a\x02uW` a8\xDEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92a#\x03V[\x94\x91Pa1,V[=\x91Pa8\xA9V[a9\x08\x91\x92P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[\x90_a0\xD4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x91\x90\x82\x01\x80\x92\x11a\x1C%WV[\x95\x92\x90\x93\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x90\x81\x81\x10_\x14a;>WP[_\x93\x80\x82\x10a9\xE4W[PPPa9\xD3\x92Pa9\x90V[\x81\x81\x11\x15a9\xDFWP\x90V[\x90P\x90V[a9\xF0\x92\x93\x94Pa#ZV[\x90\x80\x15\x80\x15\x90\x81a:\xE0W[P\x15a:\xD9WPP\x81[\x80\x83\x11a:\xD1W[P\x81\x83\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x84\x15\x82\x86\x86\x04\x14\x17\x02\x15a:CWPg\r\xE0\xB6\xB3\xA7d\0\0a9\xD3\x92\x04[\x90_\x80\x80a9\xC6V[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x86\t\x82\x81\x10\x83\x01\x90\x03\x93\x85\t\x83g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a:\xC4Wa9\xD3\x93\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02a::V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x91P_a:\x0EV[\x02\x91a:\x06V[\x90Pa;\x11W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x82\x11_a9\xFCV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90Pa9\xBCV[Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02uWV[\x90a\x01\xE0\x82\x01\x80QQ`\x1F\x81\x01\x80\x91\x11a\x1C%W`\x05\x1C\x90\x81`\x12\x01\x80`\x12\x11a\x1C%W`\x13`@Q\x93\x84\x92\x83R\x01`\x05\x1B\x01`@Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x01\xC0` \x84\x01\x95` \x87R`\xFF\x81Q\x16`@\x86\x01Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16``\x86\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`\x80\x86\x01R``\x81\x01Q`\xA0\x86\x01R`\x80\x81\x01Q`\xC0\x86\x01R`\xA0\x81\x01Q`\xE0\x86\x01R`\xC0\x81\x01Qa\x01\0\x86\x01R`\xE0\x81\x01Qa\x01 \x86\x01Ra\x01\0\x81\x01Qa\x01@\x86\x01R\x82a\x01 \x82\x01Q\x16a\x01`\x86\x01R\x82a\x01@\x82\x01Q\x16a\x01\x80\x86\x01R\x82a\x01`\x82\x01Q\x16a\x01\xA0\x86\x01Ra\x01\x80\x81\x01Q\x82\x86\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x86\x01R\x01Q\x16a\x02\0\x83\x01Ra\x02\0a\x02 \x83\x01R\x80QQa\x02@\x83\x01RQ\x80Q\x80a<\x95W[PP\x80Q\x92\x83`\x05\x1B\x90 \x92`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x86\x01\x94\x01\x01\x90[\x81\x81\x10a=\x15WPP`\x1F\x16\x90\x81\x15a<sW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a<sV[\x80Q\x84R` \x93\x84\x01\x93\x01a<\xA6V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x80\x15a\n\\W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91`\xC0\x93_\x91a>\x1EW[P\x01Q\x16\x91\x01Q\x81\x81\x03a=\xF3WP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[a>7\x91P`@=`@\x11a\x0BeWa\x0BW\x81\x83a\"RV[_a=\xE1V[\x80`\xA0\x1Ca>^Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[=\x15a>\xB3W=\x90a>\x9A\x82a\"\x93V[\x91a>\xA8`@Q\x93\x84a\"RV[\x82R=_` \x84\x01>V[``\x90V[\x94_\x94\x84\x15aA\xC3W\x80Q\x15\x80\x15aA\xBAW[aA\xA9WZa\xC3Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa>\xF7\x81\x86\x16\x95g\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`\x05\x1C\x16\x90a)\x13V[\x16\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1C%Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aAyW0;\x15a\x02uW_a?\x90\x91`@Q\x80\x93\x81\x92\x7F\xACO\xCA\x82\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x89\x16\x99\x8A`\x04\x86\x01R\x16\x96\x87`$\x85\x01R\x8A`D\x85\x01R`d\x84\x01R`\xA0`\x84\x84\x01R`\xA4\x83\x01\x90a!'V[\x03\x81\x830Z\xF1\x90\x81aAdW[PaA2WPa?\xABa>\x89V[\x91\x84\x92`$\x81Q\x14a@\xC1W[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x96\x87\x15\x15\x80a@\xB7W[\x15a@\x1CWPP` \x92\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x94\x92a@\x11\x92\x87aC\xCEV[P`@Q`\x01\x81R\xA3V[` \x97P\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x95\x93P\x87\x94\x91\x92P\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x92\x16\x98\x89\x93\x84\x87R\x86\x82R`@\x87 \x86_R\x82R`@_ a@\xA3\x82\x82Ta9\x90V[\x90U`@Q\x90\x81R\xA4P`@Q`\x02\x81R\xA3V[P0\x88\x14\x15a?\xDAV[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x03a?\xB8Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P_a?\xB8V[\x95PPPP` \x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9\x91`@Q\x90\x81R\xA3V[aAq\x91\x96P_\x90a\"RV[_\x94_a?\x9DV[PZ\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[PP\x91\x93\x90\x92Pa(s\x94PaC\xCEV[P\x83;\x15a>\xCBV[PPPPPPPPV[\x95\x90\x92\x98\x93\x96\x91\x97\x94\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x97aA\xF9\x89a\"6V[\x16\x87R` \x87\x01R`@Q\x95aB\x0E\x87a!\xD0V[\x86R` \x86\x01\x96\x87R`@\x86\x01\x97\x88R`@Q\x98aB+\x8Aa\"6V[0\x8AR` \x8A\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x02uW\x83\x92`@Q\x99\x8A\x99\x8A\x99\x8A\x99\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8BR`\x04\x8B\x01\x90Q\x90aB\xA8\x91` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[Q`D\x8A\x01RQ`d\x89\x01R\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x84\x89\x01R` \x01Q`\xA4\x88\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC4\x87\x01R`\xE4\x86\x01Ra\x01\x04\x85\x01a\x01@\x90Ra\x01D\x85\x01aC\x12\x91a!'V[\x92\x84\x84\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC\x01a\x01$\x86\x01R\x81\x84R` \x84\x017\x80\x82\x01` \x01_\x90R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x03` \x01Z\x92_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81\x95\xF1\x80\x15a\n\\WaC\x9CWPV[_a(s\x91a\"RV[\x91\x90\x82`@\x91\x03\x12a\x02uW`@QaC\xBE\x81a\"6V[` \x80\x82\x94\x80Q\x84R\x01Q\x91\x01RV[\x90\x83\x15aD\xB4W_\x80`@Q\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x87\x01\x91\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16\x95\x86`$\x82\x01R\x87`D\x82\x01R`D\x81RaD5`d\x82a\"RV[Q\x90\x82\x85Z\xF1aDCa>\x89V[\x81aD\xBAW[PaD\xB4W` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x92\x85_R_\x83R`@_ \x82\x82\x16_R\x83R`@_ aD\xA7\x88\x82Ta9\x90V[\x90U`@Q\x96\x87R\x16\x94\xA4V[PPPPV[\x80Q\x80\x15\x92P\x82\x15aD\xCFW[PP_aDIV[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x02uW` aD\xEA\x91\x01a#gV[_\x80aD\xC7V\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Custom error with signature `EidMismatch(uint32,uint32)` and selector `0xb9190bb3`.
```solidity
error EidMismatch(uint32 configured, uint32 onchain);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EidMismatch {
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
        impl ::core::convert::From<EidMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: EidMismatch) -> Self {
                (value.configured, value.onchain)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EidMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    configured: tuple.0,
                    onchain: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EidMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EidMismatch(uint32,uint32)";
            const SELECTOR: [u8; 4] = [185u8, 25u8, 11u8, 179u8];
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
    /**Custom error with signature `NotEndpoint(address)` and selector `0x39e7e94b`.
```solidity
error NotEndpoint(address caller);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEndpoint {
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
        impl ::core::convert::From<NotEndpoint> for UnderlyingRustTuple<'_> {
            fn from(value: NotEndpoint) -> Self {
                (value.caller,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEndpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { caller: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEndpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEndpoint(address)";
            const SELECTOR: [u8; 4] = [57u8, 231u8, 233u8, 75u8];
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
    /**Custom error with signature `TokenMismatch(address,address)` and selector `0xf902523f`.
```solidity
error TokenMismatch(address onchain, address configured);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TokenMismatch {
        #[allow(missing_docs)]
        pub onchain: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub configured: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<TokenMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: TokenMismatch) -> Self {
                (value.onchain, value.configured)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TokenMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    onchain: tuple.0,
                    configured: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TokenMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TokenMismatch(address,address)";
            const SELECTOR: [u8; 4] = [249u8, 2u8, 82u8, 63u8];
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
                        &self.onchain,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.configured,
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
    /**Custom error with signature `UntrustedLocalOFT(address)` and selector `0x89233a72`.
```solidity
error UntrustedLocalOFT(address from);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UntrustedLocalOFT {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<UntrustedLocalOFT> for UnderlyingRustTuple<'_> {
            fn from(value: UntrustedLocalOFT) -> Self {
                (value.from,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UntrustedLocalOFT {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { from: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UntrustedLocalOFT {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UntrustedLocalOFT(address)";
            const SELECTOR: [u8; 4] = [137u8, 35u8, 58u8, 114u8];
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
                        &self.from,
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
    /**Custom error with signature `UntrustedPeer(bytes32)` and selector `0x25ea23d9`.
```solidity
error UntrustedPeer(bytes32 composeFrom);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UntrustedPeer {
        #[allow(missing_docs)]
        pub composeFrom: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<UntrustedPeer> for UnderlyingRustTuple<'_> {
            fn from(value: UntrustedPeer) -> Self {
                (value.composeFrom,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UntrustedPeer {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { composeFrom: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UntrustedPeer {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UntrustedPeer(bytes32)";
            const SELECTOR: [u8; 4] = [37u8, 234u8, 35u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.composeFrom),
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
    /**Custom error with signature `UntrustedSourceEid(uint32)` and selector `0xe73d23c3`.
```solidity
error UntrustedSourceEid(uint32 srcEid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UntrustedSourceEid {
        #[allow(missing_docs)]
        pub srcEid: u32,
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
        impl ::core::convert::From<UntrustedSourceEid> for UnderlyingRustTuple<'_> {
            fn from(value: UntrustedSourceEid) -> Self {
                (value.srcEid,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UntrustedSourceEid {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { srcEid: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UntrustedSourceEid {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UntrustedSourceEid(uint32)";
            const SELECTOR: [u8; 4] = [231u8, 61u8, 35u8, 195u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.srcEid),
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
constructor(address config_, address owner_, uint256 maxFeeRate_, uint8 oftId_);
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
        pub oftId_: u8,
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
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
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
                    (value.config_, value.owner_, value.maxFeeRate_, value.oftId_)
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
                        oftId_: tuple.3,
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
                alloy::sol_types::sol_data::Uint<8>,
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
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.oftId_),
                )
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
    /**Function with signature `initiateOFT(uint32,bytes32,uint256,uint256,bytes,uint64,uint256,uint256,(uint64,bytes))` and selector `0x5975e790`.
```solidity
function initiateOFT(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 minAmountLD, bytes memory extraOptions, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec) external payable returns (bytes32 orderId, uint64 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOFTCall {
        #[allow(missing_docs)]
        pub dstChainId: u32,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minAmountLD: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub extraOptions: alloy::sol_types::private::Bytes,
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
    ///Container type for the return parameters of the [`initiateOFT(uint32,bytes32,uint256,uint256,bytes,uint64,uint256,uint256,(uint64,bytes))`](initiateOFTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOFTReturn {
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
                alloy::sol_types::sol_data::Bytes,
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
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<initiateOFTCall> for UnderlyingRustTuple<'_> {
                fn from(value: initiateOFTCall) -> Self {
                    (
                        value.dstChainId,
                        value.recipient,
                        value.inputAmount,
                        value.minAmountLD,
                        value.extraOptions,
                        value.deliveryWindow,
                        value.discountRate,
                        value.baseFee,
                        value.exec,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initiateOFTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dstChainId: tuple.0,
                        recipient: tuple.1,
                        inputAmount: tuple.2,
                        minAmountLD: tuple.3,
                        extraOptions: tuple.4,
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
            impl ::core::convert::From<initiateOFTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initiateOFTReturn) -> Self {
                    (value.orderId, value.nonce)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initiateOFTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        nonce: tuple.1,
                    }
                }
            }
        }
        impl initiateOFTReturn {
            fn _tokenize(
                &self,
            ) -> <initiateOFTCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
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
        impl alloy_sol_types::SolCall for initiateOFTCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Execution,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initiateOFTReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initiateOFT(uint32,bytes32,uint256,uint256,bytes,uint64,uint256,uint256,(uint64,bytes))";
            const SELECTOR: [u8; 4] = [89u8, 117u8, 231u8, 144u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.minAmountLD),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraOptions,
                    ),
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
                initiateOFTReturn::_tokenize(ret)
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
    /**Function with signature `initiateOFTFor(uint32,bytes32,uint256,uint256,bytes,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))` and selector `0x4a6be468`.
```solidity
function initiateOFTFor(uint32 dstChainId, bytes32 recipient, uint256 inputAmount, uint256 minAmountLD, bytes memory extraOptions, uint64 deliveryWindow, uint256 discountRate, uint256 baseFee, Execution memory exec, address from, PermitLib.Permit2Data memory permit) external payable returns (bytes32 orderId, uint64 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOFTForCall {
        #[allow(missing_docs)]
        pub dstChainId: u32,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minAmountLD: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub extraOptions: alloy::sol_types::private::Bytes,
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
    ///Container type for the return parameters of the [`initiateOFTFor(uint32,bytes32,uint256,uint256,bytes,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))`](initiateOFTForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initiateOFTForReturn {
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
                alloy::sol_types::sol_data::Bytes,
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
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<initiateOFTForCall> for UnderlyingRustTuple<'_> {
                fn from(value: initiateOFTForCall) -> Self {
                    (
                        value.dstChainId,
                        value.recipient,
                        value.inputAmount,
                        value.minAmountLD,
                        value.extraOptions,
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initiateOFTForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dstChainId: tuple.0,
                        recipient: tuple.1,
                        inputAmount: tuple.2,
                        minAmountLD: tuple.3,
                        extraOptions: tuple.4,
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
            impl ::core::convert::From<initiateOFTForReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initiateOFTForReturn) -> Self {
                    (value.orderId, value.nonce)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initiateOFTForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        nonce: tuple.1,
                    }
                }
            }
        }
        impl initiateOFTForReturn {
            fn _tokenize(
                &self,
            ) -> <initiateOFTForCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
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
        impl alloy_sol_types::SolCall for initiateOFTForCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
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
            type Return = initiateOFTForReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initiateOFTFor(uint32,bytes32,uint256,uint256,bytes,uint64,uint256,uint256,(uint64,bytes),address,(uint256,uint256,bytes))";
            const SELECTOR: [u8; 4] = [74u8, 107u8, 228u8, 104u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.minAmountLD),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraOptions,
                    ),
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
                initiateOFTForReturn::_tokenize(ret)
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
    /**Function with signature `lzCompose(address,bytes32,bytes,address,bytes)` and selector `0xd0a10260`.
```solidity
function lzCompose(address from, bytes32, bytes memory message, address, bytes memory) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lzComposeCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _4: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`lzCompose(address,bytes32,bytes,address,bytes)`](lzComposeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lzComposeReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<lzComposeCall> for UnderlyingRustTuple<'_> {
                fn from(value: lzComposeCall) -> Self {
                    (value.from, value._1, value.message, value._3, value._4)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lzComposeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        _1: tuple.1,
                        message: tuple.2,
                        _3: tuple.3,
                        _4: tuple.4,
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
            impl ::core::convert::From<lzComposeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lzComposeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lzComposeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl lzComposeReturn {
            fn _tokenize(
                &self,
            ) -> <lzComposeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lzComposeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lzComposeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lzCompose(address,bytes32,bytes,address,bytes)";
            const SELECTOR: [u8; 4] = [208u8, 161u8, 2u8, 96u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._3,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._4,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                lzComposeReturn::_tokenize(ret)
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
    /**Function with signature `oftId()` and selector `0x7376f1c0`.
```solidity
function oftId() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct oftIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`oftId()`](oftIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct oftIdReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<oftIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: oftIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for oftIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<oftIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: oftIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for oftIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for oftIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "oftId()";
            const SELECTOR: [u8; 4] = [115u8, 118u8, 241u8, 192u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: oftIdReturn = r.into();
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
                        let r: oftIdReturn = r.into();
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
    ///Container for all the [`OftAdapter`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum OftAdapterCalls {
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
        initiateOFT(initiateOFTCall),
        #[allow(missing_docs)]
        initiateOFTFor(initiateOFTForCall),
        #[allow(missing_docs)]
        lzCompose(lzComposeCall),
        #[allow(missing_docs)]
        maxFeeRate(maxFeeRateCall),
        #[allow(missing_docs)]
        multicall(multicallCall),
        #[allow(missing_docs)]
        oftId(oftIdCall),
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
        transferOwnership(transferOwnershipCall),
    }
    impl OftAdapterCalls {
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
            [74u8, 107u8, 228u8, 104u8],
            [84u8, 209u8, 241u8, 61u8],
            [87u8, 120u8, 71u8, 42u8],
            [89u8, 117u8, 231u8, 144u8],
            [92u8, 151u8, 90u8, 187u8],
            [106u8, 253u8, 216u8, 80u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 118u8, 241u8, 192u8],
            [119u8, 111u8, 243u8, 199u8],
            [121u8, 80u8, 44u8, 85u8],
            [133u8, 193u8, 120u8, 48u8],
            [140u8, 218u8, 150u8, 222u8],
            [141u8, 165u8, 203u8, 91u8],
            [151u8, 195u8, 107u8, 174u8],
            [172u8, 79u8, 202u8, 130u8],
            [172u8, 150u8, 80u8, 216u8],
            [208u8, 161u8, 2u8, 96u8],
            [212u8, 87u8, 12u8, 28u8],
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
            ::core::stringify!(initiateOFTFor),
            ::core::stringify!(cancelOwnershipHandover),
            ::core::stringify!(getOrder),
            ::core::stringify!(initiateOFT),
            ::core::stringify!(paused),
            ::core::stringify!(PERMIT2),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(oftId),
            ::core::stringify!(fillFor),
            ::core::stringify!(config),
            ::core::stringify!(maxFeeRate),
            ::core::stringify!(setMaxFeeRate),
            ::core::stringify!(owner),
            ::core::stringify!(quoteFill),
            ::core::stringify!(_executeDelivery),
            ::core::stringify!(multicall),
            ::core::stringify!(lzCompose),
            ::core::stringify!(claimable),
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
            <initiateOFTForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getOrderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initiateOFTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PERMIT2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <oftIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fillForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <configCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setMaxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <quoteFillCall as alloy_sol_types::SolCall>::SIGNATURE,
            <_executeDeliveryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multicallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lzComposeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimableCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OftAdapterCalls {
        const NAME: &'static str = "OftAdapterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                Self::initiateOFT(_) => {
                    <initiateOFTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initiateOFTFor(_) => {
                    <initiateOFTForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lzCompose(_) => {
                    <lzComposeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxFeeRate(_) => {
                    <maxFeeRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multicall(_) => {
                    <multicallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::oftId(_) => <oftIdCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<OftAdapterCalls>] = &[
                {
                    fn setPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <setPausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::setPaused)
                    }
                    setPaused
                },
                {
                    fn claim(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <claimCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::claim)
                    }
                    claim
                },
                {
                    fn requestOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::requestOwnershipHandover)
                    }
                    requestOwnershipHandover
                },
                {
                    fn fill(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <fillCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::fill)
                    }
                    fill
                },
                {
                    fn MAX_CALLBACK_GAS_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::MAX_CALLBACK_GAS_LIMIT)
                    }
                    MAX_CALLBACK_GAS_LIMIT
                },
                {
                    fn initiateOFTFor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <initiateOFTForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::initiateOFTFor)
                    }
                    initiateOFTFor
                },
                {
                    fn cancelOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::cancelOwnershipHandover)
                    }
                    cancelOwnershipHandover
                },
                {
                    fn getOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <getOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::getOrder)
                    }
                    getOrder
                },
                {
                    fn initiateOFT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <initiateOFTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::initiateOFT)
                    }
                    initiateOFT
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn PERMIT2(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <PERMIT2Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::PERMIT2)
                    }
                    PERMIT2
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn oftId(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <oftIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::oftId)
                    }
                    oftId
                },
                {
                    fn fillFor(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <fillForCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::fillFor)
                    }
                    fillFor
                },
                {
                    fn config(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <configCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::config)
                    }
                    config
                },
                {
                    fn maxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <maxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::maxFeeRate)
                    }
                    maxFeeRate
                },
                {
                    fn setMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <setMaxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::setMaxFeeRate)
                    }
                    setMaxFeeRate
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn quoteFill(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <quoteFillCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::quoteFill)
                    }
                    quoteFill
                },
                {
                    fn _executeDelivery(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <_executeDeliveryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::_executeDelivery)
                    }
                    _executeDelivery
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::multicall)
                    }
                    multicall
                },
                {
                    fn lzCompose(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <lzComposeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::lzCompose)
                    }
                    lzCompose
                },
                {
                    fn claimable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <claimableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OftAdapterCalls::claimable)
                    }
                    claimable
                },
                {
                    fn completeOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::completeOwnershipHandover)
                    }
                    completeOwnershipHandover
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn selfPermit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <selfPermitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::selfPermit)
                    }
                    selfPermit
                },
                {
                    fn ownershipHandoverExpiresAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterCalls::ownershipHandoverExpiresAt)
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
            ) -> alloy_sol_types::Result<OftAdapterCalls>] = &[
                {
                    fn setPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <setPausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::setPaused)
                    }
                    setPaused
                },
                {
                    fn claim(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <claimCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::claim)
                    }
                    claim
                },
                {
                    fn requestOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <requestOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::requestOwnershipHandover)
                    }
                    requestOwnershipHandover
                },
                {
                    fn fill(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <fillCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::fill)
                    }
                    fill
                },
                {
                    fn MAX_CALLBACK_GAS_LIMIT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <MAX_CALLBACK_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::MAX_CALLBACK_GAS_LIMIT)
                    }
                    MAX_CALLBACK_GAS_LIMIT
                },
                {
                    fn initiateOFTFor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <initiateOFTForCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::initiateOFTFor)
                    }
                    initiateOFTFor
                },
                {
                    fn cancelOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <cancelOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::cancelOwnershipHandover)
                    }
                    cancelOwnershipHandover
                },
                {
                    fn getOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <getOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::getOrder)
                    }
                    getOrder
                },
                {
                    fn initiateOFT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <initiateOFTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::initiateOFT)
                    }
                    initiateOFT
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::paused)
                    }
                    paused
                },
                {
                    fn PERMIT2(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <PERMIT2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::PERMIT2)
                    }
                    PERMIT2
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn oftId(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <oftIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::oftId)
                    }
                    oftId
                },
                {
                    fn fillFor(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <fillForCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::fillFor)
                    }
                    fillFor
                },
                {
                    fn config(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <configCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::config)
                    }
                    config
                },
                {
                    fn maxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <maxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::maxFeeRate)
                    }
                    maxFeeRate
                },
                {
                    fn setMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <setMaxFeeRateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::setMaxFeeRate)
                    }
                    setMaxFeeRate
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::owner)
                    }
                    owner
                },
                {
                    fn quoteFill(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <quoteFillCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::quoteFill)
                    }
                    quoteFill
                },
                {
                    fn _executeDelivery(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <_executeDeliveryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::_executeDelivery)
                    }
                    _executeDelivery
                },
                {
                    fn multicall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::multicall)
                    }
                    multicall
                },
                {
                    fn lzCompose(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <lzComposeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::lzCompose)
                    }
                    lzCompose
                },
                {
                    fn claimable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <claimableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::claimable)
                    }
                    claimable
                },
                {
                    fn completeOwnershipHandover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <completeOwnershipHandoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::completeOwnershipHandover)
                    }
                    completeOwnershipHandover
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn selfPermit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <selfPermitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::selfPermit)
                    }
                    selfPermit
                },
                {
                    fn ownershipHandoverExpiresAt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterCalls> {
                        <ownershipHandoverExpiresAtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterCalls::ownershipHandoverExpiresAt)
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
                Self::initiateOFT(inner) => {
                    <initiateOFTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initiateOFTFor(inner) => {
                    <initiateOFTForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lzCompose(inner) => {
                    <lzComposeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::maxFeeRate(inner) => {
                    <maxFeeRateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::oftId(inner) => {
                    <oftIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::initiateOFT(inner) => {
                    <initiateOFTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initiateOFTFor(inner) => {
                    <initiateOFTForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lzCompose(inner) => {
                    <lzComposeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::oftId(inner) => {
                    <oftIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OftAdapter`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OftAdapterErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AlreadySettled(AlreadySettled),
        #[allow(missing_docs)]
        EidMismatch(EidMismatch),
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
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        #[allow(missing_docs)]
        NoHandoverRequest(NoHandoverRequest),
        #[allow(missing_docs)]
        NotEndpoint(NotEndpoint),
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
        Reentrancy(Reentrancy),
        #[allow(missing_docs)]
        TokenMismatch(TokenMismatch),
        #[allow(missing_docs)]
        Unauthorized(Unauthorized),
        #[allow(missing_docs)]
        UnsupportedChain(UnsupportedChain),
        #[allow(missing_docs)]
        UntrustedLocalOFT(UntrustedLocalOFT),
        #[allow(missing_docs)]
        UntrustedPeer(UntrustedPeer),
        #[allow(missing_docs)]
        UntrustedSourceEid(UntrustedSourceEid),
        #[allow(missing_docs)]
        WrongBridgeType(WrongBridgeType),
        #[allow(missing_docs)]
        WrongDestinationChain(WrongDestinationChain),
        #[allow(missing_docs)]
        WrongOutputToken(WrongOutputToken),
        #[allow(missing_docs)]
        ZeroRecipient(ZeroRecipient),
    }
    impl OftAdapterErrors {
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
            [37u8, 234u8, 35u8, 217u8],
            [40u8, 2u8, 221u8, 158u8],
            [43u8, 249u8, 80u8, 101u8],
            [46u8, 119u8, 92u8, 124u8],
            [52u8, 62u8, 33u8, 30u8],
            [57u8, 231u8, 233u8, 75u8],
            [88u8, 135u8, 0u8, 199u8],
            [111u8, 94u8, 136u8, 24u8],
            [116u8, 72u8, 251u8, 174u8],
            [130u8, 180u8, 41u8, 0u8],
            [137u8, 35u8, 58u8, 114u8],
            [141u8, 0u8, 185u8, 27u8],
            [141u8, 174u8, 45u8, 43u8],
            [141u8, 208u8, 157u8, 145u8],
            [150u8, 155u8, 247u8, 40u8],
            [158u8, 135u8, 250u8, 200u8],
            [171u8, 20u8, 60u8, 6u8],
            [173u8, 107u8, 182u8, 209u8],
            [177u8, 150u8, 164u8, 74u8],
            [178u8, 195u8, 182u8, 253u8],
            [184u8, 37u8, 221u8, 118u8],
            [185u8, 25u8, 11u8, 179u8],
            [210u8, 123u8, 68u8, 67u8],
            [231u8, 61u8, 35u8, 195u8],
            [249u8, 2u8, 82u8, 63u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(OnlySelf),
            ::core::stringify!(NotSourceChain),
            ::core::stringify!(InvalidCallbackGasLimit),
            ::core::stringify!(UntrustedPeer),
            ::core::stringify!(InvalidWindow),
            ::core::stringify!(InvalidAddress),
            ::core::stringify!(WrongOutputToken),
            ::core::stringify!(OrderAlreadyActive),
            ::core::stringify!(NotEndpoint),
            ::core::stringify!(InsufficientCallbackGas),
            ::core::stringify!(NoHandoverRequest),
            ::core::stringify!(NewOwnerIsZeroAddress),
            ::core::stringify!(Unauthorized),
            ::core::stringify!(UntrustedLocalOFT),
            ::core::stringify!(InvalidBaseFee),
            ::core::stringify!(WrongDestinationChain),
            ::core::stringify!(InvalidOutputAmount),
            ::core::stringify!(NothingToClaim),
            ::core::stringify!(Paused),
            ::core::stringify!(Reentrancy),
            ::core::stringify!(InvalidMaxFeeRate),
            ::core::stringify!(AlreadySettled),
            ::core::stringify!(WrongBridgeType),
            ::core::stringify!(UnsupportedChain),
            ::core::stringify!(EidMismatch),
            ::core::stringify!(ZeroRecipient),
            ::core::stringify!(UntrustedSourceEid),
            ::core::stringify!(TokenMismatch),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <OnlySelf as alloy_sol_types::SolError>::SIGNATURE,
            <NotSourceChain as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidCallbackGasLimit as alloy_sol_types::SolError>::SIGNATURE,
            <UntrustedPeer as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidWindow as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidAddress as alloy_sol_types::SolError>::SIGNATURE,
            <WrongOutputToken as alloy_sol_types::SolError>::SIGNATURE,
            <OrderAlreadyActive as alloy_sol_types::SolError>::SIGNATURE,
            <NotEndpoint as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientCallbackGas as alloy_sol_types::SolError>::SIGNATURE,
            <NoHandoverRequest as alloy_sol_types::SolError>::SIGNATURE,
            <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::SIGNATURE,
            <Unauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <UntrustedLocalOFT as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidBaseFee as alloy_sol_types::SolError>::SIGNATURE,
            <WrongDestinationChain as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidOutputAmount as alloy_sol_types::SolError>::SIGNATURE,
            <NothingToClaim as alloy_sol_types::SolError>::SIGNATURE,
            <Paused as alloy_sol_types::SolError>::SIGNATURE,
            <Reentrancy as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidMaxFeeRate as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadySettled as alloy_sol_types::SolError>::SIGNATURE,
            <WrongBridgeType as alloy_sol_types::SolError>::SIGNATURE,
            <UnsupportedChain as alloy_sol_types::SolError>::SIGNATURE,
            <EidMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroRecipient as alloy_sol_types::SolError>::SIGNATURE,
            <UntrustedSourceEid as alloy_sol_types::SolError>::SIGNATURE,
            <TokenMismatch as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for OftAdapterErrors {
        const NAME: &'static str = "OftAdapterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AlreadySettled(_) => {
                    <AlreadySettled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EidMismatch(_) => {
                    <EidMismatch as alloy_sol_types::SolError>::SELECTOR
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
                Self::NewOwnerIsZeroAddress(_) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoHandoverRequest(_) => {
                    <NoHandoverRequest as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEndpoint(_) => {
                    <NotEndpoint as alloy_sol_types::SolError>::SELECTOR
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
                Self::Reentrancy(_) => {
                    <Reentrancy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TokenMismatch(_) => {
                    <TokenMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedChain(_) => {
                    <UnsupportedChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UntrustedLocalOFT(_) => {
                    <UntrustedLocalOFT as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UntrustedPeer(_) => {
                    <UntrustedPeer as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UntrustedSourceEid(_) => {
                    <UntrustedSourceEid as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<OftAdapterErrors>] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OnlySelf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <OnlySelf as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OftAdapterErrors::OnlySelf)
                    }
                    OnlySelf
                },
                {
                    fn NotSourceChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NotSourceChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::NotSourceChain)
                    }
                    NotSourceChain
                },
                {
                    fn InvalidCallbackGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidCallbackGasLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidCallbackGasLimit)
                    }
                    InvalidCallbackGasLimit
                },
                {
                    fn UntrustedPeer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UntrustedPeer as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::UntrustedPeer)
                    }
                    UntrustedPeer
                },
                {
                    fn InvalidWindow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidWindow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidWindow)
                    }
                    InvalidWindow
                },
                {
                    fn InvalidAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidAddress)
                    }
                    InvalidAddress
                },
                {
                    fn WrongOutputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <WrongOutputToken as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::WrongOutputToken)
                    }
                    WrongOutputToken
                },
                {
                    fn OrderAlreadyActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <OrderAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::OrderAlreadyActive)
                    }
                    OrderAlreadyActive
                },
                {
                    fn NotEndpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NotEndpoint as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OftAdapterErrors::NotEndpoint)
                    }
                    NotEndpoint
                },
                {
                    fn InsufficientCallbackGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InsufficientCallbackGas as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InsufficientCallbackGas)
                    }
                    InsufficientCallbackGas
                },
                {
                    fn NoHandoverRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NoHandoverRequest as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::NoHandoverRequest)
                    }
                    NoHandoverRequest
                },
                {
                    fn NewOwnerIsZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::NewOwnerIsZeroAddress)
                    }
                    NewOwnerIsZeroAddress
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OftAdapterErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn UntrustedLocalOFT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UntrustedLocalOFT as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::UntrustedLocalOFT)
                    }
                    UntrustedLocalOFT
                },
                {
                    fn InvalidBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidBaseFee as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidBaseFee)
                    }
                    InvalidBaseFee
                },
                {
                    fn WrongDestinationChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <WrongDestinationChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::WrongDestinationChain)
                    }
                    WrongDestinationChain
                },
                {
                    fn InvalidOutputAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidOutputAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidOutputAmount)
                    }
                    InvalidOutputAmount
                },
                {
                    fn NothingToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NothingToClaim as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::NothingToClaim)
                    }
                    NothingToClaim
                },
                {
                    fn Paused(data: &[u8]) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <Paused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OftAdapterErrors::Paused)
                    }
                    Paused
                },
                {
                    fn Reentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <Reentrancy as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OftAdapterErrors::Reentrancy)
                    }
                    Reentrancy
                },
                {
                    fn InvalidMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidMaxFeeRate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidMaxFeeRate)
                    }
                    InvalidMaxFeeRate
                },
                {
                    fn AlreadySettled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <AlreadySettled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::AlreadySettled)
                    }
                    AlreadySettled
                },
                {
                    fn WrongBridgeType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <WrongBridgeType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::WrongBridgeType)
                    }
                    WrongBridgeType
                },
                {
                    fn UnsupportedChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UnsupportedChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::UnsupportedChain)
                    }
                    UnsupportedChain
                },
                {
                    fn EidMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <EidMismatch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OftAdapterErrors::EidMismatch)
                    }
                    EidMismatch
                },
                {
                    fn ZeroRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <ZeroRecipient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::ZeroRecipient)
                    }
                    ZeroRecipient
                },
                {
                    fn UntrustedSourceEid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UntrustedSourceEid as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::UntrustedSourceEid)
                    }
                    UntrustedSourceEid
                },
                {
                    fn TokenMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <TokenMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(OftAdapterErrors::TokenMismatch)
                    }
                    TokenMismatch
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
            ) -> alloy_sol_types::Result<OftAdapterErrors>] = &[
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn OnlySelf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <OnlySelf as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::OnlySelf)
                    }
                    OnlySelf
                },
                {
                    fn NotSourceChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NotSourceChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::NotSourceChain)
                    }
                    NotSourceChain
                },
                {
                    fn InvalidCallbackGasLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidCallbackGasLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidCallbackGasLimit)
                    }
                    InvalidCallbackGasLimit
                },
                {
                    fn UntrustedPeer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UntrustedPeer as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::UntrustedPeer)
                    }
                    UntrustedPeer
                },
                {
                    fn InvalidWindow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidWindow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidWindow)
                    }
                    InvalidWindow
                },
                {
                    fn InvalidAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidAddress)
                    }
                    InvalidAddress
                },
                {
                    fn WrongOutputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <WrongOutputToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::WrongOutputToken)
                    }
                    WrongOutputToken
                },
                {
                    fn OrderAlreadyActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <OrderAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::OrderAlreadyActive)
                    }
                    OrderAlreadyActive
                },
                {
                    fn NotEndpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NotEndpoint as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::NotEndpoint)
                    }
                    NotEndpoint
                },
                {
                    fn InsufficientCallbackGas(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InsufficientCallbackGas as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InsufficientCallbackGas)
                    }
                    InsufficientCallbackGas
                },
                {
                    fn NoHandoverRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NoHandoverRequest as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::NoHandoverRequest)
                    }
                    NoHandoverRequest
                },
                {
                    fn NewOwnerIsZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::NewOwnerIsZeroAddress)
                    }
                    NewOwnerIsZeroAddress
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn UntrustedLocalOFT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UntrustedLocalOFT as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::UntrustedLocalOFT)
                    }
                    UntrustedLocalOFT
                },
                {
                    fn InvalidBaseFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidBaseFee as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidBaseFee)
                    }
                    InvalidBaseFee
                },
                {
                    fn WrongDestinationChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <WrongDestinationChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::WrongDestinationChain)
                    }
                    WrongDestinationChain
                },
                {
                    fn InvalidOutputAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidOutputAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidOutputAmount)
                    }
                    InvalidOutputAmount
                },
                {
                    fn NothingToClaim(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <NothingToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::NothingToClaim)
                    }
                    NothingToClaim
                },
                {
                    fn Paused(data: &[u8]) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <Paused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::Paused)
                    }
                    Paused
                },
                {
                    fn Reentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <Reentrancy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::Reentrancy)
                    }
                    Reentrancy
                },
                {
                    fn InvalidMaxFeeRate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <InvalidMaxFeeRate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::InvalidMaxFeeRate)
                    }
                    InvalidMaxFeeRate
                },
                {
                    fn AlreadySettled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <AlreadySettled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::AlreadySettled)
                    }
                    AlreadySettled
                },
                {
                    fn WrongBridgeType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <WrongBridgeType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::WrongBridgeType)
                    }
                    WrongBridgeType
                },
                {
                    fn UnsupportedChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UnsupportedChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::UnsupportedChain)
                    }
                    UnsupportedChain
                },
                {
                    fn EidMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <EidMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::EidMismatch)
                    }
                    EidMismatch
                },
                {
                    fn ZeroRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <ZeroRecipient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::ZeroRecipient)
                    }
                    ZeroRecipient
                },
                {
                    fn UntrustedSourceEid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <UntrustedSourceEid as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::UntrustedSourceEid)
                    }
                    UntrustedSourceEid
                },
                {
                    fn TokenMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OftAdapterErrors> {
                        <TokenMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OftAdapterErrors::TokenMismatch)
                    }
                    TokenMismatch
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
                Self::EidMismatch(inner) => {
                    <EidMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::NotEndpoint(inner) => {
                    <NotEndpoint as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::Reentrancy(inner) => {
                    <Reentrancy as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TokenMismatch(inner) => {
                    <TokenMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UnsupportedChain(inner) => {
                    <UnsupportedChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UntrustedLocalOFT(inner) => {
                    <UntrustedLocalOFT as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UntrustedPeer(inner) => {
                    <UntrustedPeer as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UntrustedSourceEid(inner) => {
                    <UntrustedSourceEid as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EidMismatch(inner) => {
                    <EidMismatch as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotEndpoint(inner) => {
                    <NotEndpoint as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::Reentrancy(inner) => {
                    <Reentrancy as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::TokenMismatch(inner) => {
                    <TokenMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::UntrustedLocalOFT(inner) => {
                    <UntrustedLocalOFT as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UntrustedPeer(inner) => {
                    <UntrustedPeer as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UntrustedSourceEid(inner) => {
                    <UntrustedSourceEid as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`OftAdapter`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum OftAdapterEvents {
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
    impl OftAdapterEvents {
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
    impl alloy_sol_types::SolEventInterface for OftAdapterEvents {
        const NAME: &'static str = "OftAdapterEvents";
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
    impl alloy_sol_types::private::IntoLogData for OftAdapterEvents {
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
    /**Creates a new wrapper around an on-chain [`OftAdapter`](self) contract instance.

See the [wrapper's documentation](`OftAdapterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> OftAdapterInstance<P, N> {
        OftAdapterInstance::<P, N>::new(address, __provider)
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
        oftId_: u8,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OftAdapterInstance<P, N>>,
    > {
        OftAdapterInstance::<
            P,
            N,
        >::deploy(__provider, config_, owner_, maxFeeRate_, oftId_)
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
        oftId_: u8,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        OftAdapterInstance::<
            P,
            N,
        >::deploy_builder(__provider, config_, owner_, maxFeeRate_, oftId_)
    }
    /**A [`OftAdapter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OftAdapter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OftAdapterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OftAdapterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OftAdapterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > OftAdapterInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`OftAdapter`](self) contract instance.

See the [wrapper's documentation](`OftAdapterInstance`) for more details.*/
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
            oftId_: u8,
        ) -> alloy_contract::Result<OftAdapterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                config_,
                owner_,
                maxFeeRate_,
                oftId_,
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
            oftId_: u8,
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
                            oftId_,
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
    impl<P: ::core::clone::Clone, N> OftAdapterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OftAdapterInstance<P, N> {
            OftAdapterInstance {
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
    > OftAdapterInstance<P, N> {
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
        ///Creates a new call builder for the [`initiateOFT`] function.
        pub fn initiateOFT(
            &self,
            dstChainId: u32,
            recipient: alloy::sol_types::private::FixedBytes<32>,
            inputAmount: alloy::sol_types::private::primitives::aliases::U256,
            minAmountLD: alloy::sol_types::private::primitives::aliases::U256,
            extraOptions: alloy::sol_types::private::Bytes,
            deliveryWindow: u64,
            discountRate: alloy::sol_types::private::primitives::aliases::U256,
            baseFee: alloy::sol_types::private::primitives::aliases::U256,
            exec: <Execution as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initiateOFTCall, N> {
            self.call_builder(
                &initiateOFTCall {
                    dstChainId,
                    recipient,
                    inputAmount,
                    minAmountLD,
                    extraOptions,
                    deliveryWindow,
                    discountRate,
                    baseFee,
                    exec,
                },
            )
        }
        ///Creates a new call builder for the [`initiateOFTFor`] function.
        pub fn initiateOFTFor(
            &self,
            dstChainId: u32,
            recipient: alloy::sol_types::private::FixedBytes<32>,
            inputAmount: alloy::sol_types::private::primitives::aliases::U256,
            minAmountLD: alloy::sol_types::private::primitives::aliases::U256,
            extraOptions: alloy::sol_types::private::Bytes,
            deliveryWindow: u64,
            discountRate: alloy::sol_types::private::primitives::aliases::U256,
            baseFee: alloy::sol_types::private::primitives::aliases::U256,
            exec: <Execution as alloy::sol_types::SolType>::RustType,
            from: alloy::sol_types::private::Address,
            permit: <PermitLib::Permit2Data as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initiateOFTForCall, N> {
            self.call_builder(
                &initiateOFTForCall {
                    dstChainId,
                    recipient,
                    inputAmount,
                    minAmountLD,
                    extraOptions,
                    deliveryWindow,
                    discountRate,
                    baseFee,
                    exec,
                    from,
                    permit,
                },
            )
        }
        ///Creates a new call builder for the [`lzCompose`] function.
        pub fn lzCompose(
            &self,
            from: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
            message: alloy::sol_types::private::Bytes,
            _3: alloy::sol_types::private::Address,
            _4: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, lzComposeCall, N> {
            self.call_builder(
                &lzComposeCall {
                    from,
                    _1,
                    message,
                    _3,
                    _4,
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
        ///Creates a new call builder for the [`oftId`] function.
        pub fn oftId(&self) -> alloy_contract::SolCallBuilder<&P, oftIdCall, N> {
            self.call_builder(&oftIdCall)
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
    > OftAdapterInstance<P, N> {
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
