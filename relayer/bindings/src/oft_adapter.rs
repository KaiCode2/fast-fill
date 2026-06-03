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
    event DestinationCallback(bytes32 indexed orderId, address indexed fundsTo, CallbackResult result);
    event OrderCreated(bytes32 indexed orderId, uint8 bridgeType, address indexed sender, uint32 dstChainId, bytes32 outputToken, uint256 outputAmount, uint64 nonce);
    event OrderFilled(bytes32 indexed orderId, address indexed filler, uint256 payoutToRecipient, uint256 feeToFiller, uint40 fillTime);
    event OrderSettled(bytes32 indexed orderId, address indexed filler, uint256 arrivedAmount, uint256 surplusToRecipient);
    event OwnershipHandoverCanceled(address indexed pendingOwner);
    event OwnershipHandoverRequested(address indexed pendingOwner);
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
    event PayoutDeferred(bytes32 indexed orderId, address indexed to, address indexed token, uint256 amount);

    constructor(address config_, address owner_, uint256 maxFeeRate_, uint8 oftId_);

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
    ///0x60c03461014957601f614b0638819003918201601f19168301916001600160401b0383118484101761014d578084926080946040528339810103126101495761004781610161565b61005360208301610161565b91606060408201519101519260ff8416840361014957670de0b6b3a76400008211610136576001600160a01b0316638b78c6d8198190555f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a36002556001600160a01b031660805260a05260405161499090816101768239608051818181610f5c015281816113f201528181611e6f01528181612a9f0152818161310101528181613e680152614176015260a051818181610c73015281816113b401528181611e4101528181612a60015281816130c101528181613e3a01526141e40152f35b5063ad6bb6d160e01b5f5260045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b03821682036101495756fe60806040526004361015610011575f80fd5b5f3560e01c806316c38b3c146101c45780631e83409a146101bf57806325692962146101ba57806331eee44d146101b557806339c33215146101b05780634a6be468146101ab57806354d1f13d146101a65780635778472a146101a15780635975e7901461019c5780635c975abb14610197578063617c537c146101925780636afdd8501461018d578063715018a6146101885780637376f1c014610183578063776ff3c71461017e57806379502c551461017957806385c17830146101745780638cda96de1461016f5780638da5cb5b1461016a57806397c36bae14610165578063ac9650d814610160578063d0a102601461015b578063d4570c1c14610156578063f04e283e14610151578063f2fde38b1461014c578063f3995c67146101475763fee81cf414610142575f80fd5b611a35565b61194c565b6118d9565b611868565b6117cf565b6112d9565b61119b565b6110a4565b611034565b610fbb565b610f80565b610f12565b610c97565b610c3c565b610b9f565b610b58565b6109fb565b6109b8565b61091b565b61086c565b61078d565b6106ae565b6105d9565b610475565b6103ff565b610287565b6101d7565b801515036101d357565b5f80fd5b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435610212816101c9565b61021a61206d565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff000000000000000060035492151560401b169116176003555f80f35b73ffffffffffffffffffffffffffffffffffffffff8116036101d357565b610124359061028582610259565b565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576004356102c281610259565b688000000000ab143c065c6103e8576001688000000000ab143c065d335f5260016020526103118160405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5480156103c0576103bc91335f5260016020525f61035082604083209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5561035c8233836120a4565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd399268360203392a35f688000000000ab143c065d6040519081529081906020820190565b0390f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b5f9103126101d357565b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b90816102009103126101d35790565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d3576104c4903690600401610466565b688000000000ab143c065c6103e8576001688000000000ab143c065d60ff60035460401c166105b1576101c06105686105617fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d6105926103bc9561052833826122cc565b98939992918a98829661053d843033886124f3565b61054b6101e0820182611a89565b96909101359561055a87610668565b3691611c09565b9289612696565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d6040519081529081906020820190565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576020604051624c4b408152f35b63ffffffff8116036101d357565b6004359061028582610614565b359061028582610614565b9181601f840112156101d35782359167ffffffffffffffff83116101d357602083818601950101116101d357565b67ffffffffffffffff8116036101d357565b60a4359061028582610668565b359061028582610668565b908160409103126101d35790565b908160609103126101d35790565b6101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576106e1610622565b6024359060443560643560843567ffffffffffffffff81116101d35761070b90369060040161063a565b61071361067a565b60c4359160e435936101043567ffffffffffffffff81116101d35761073c903690600401610692565b95610745610277565b97610144359a67ffffffffffffffff8c116101d35761076b6107719c36906004016106a0565b9a611c3f565b6040805192835267ffffffffffffffff91909116602083015290f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b600311156107f957565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b919091604064ffffffffff81606084019573ffffffffffffffffffffffffffffffffffffffff8151168552602081015161085f816107ef565b6020860152015116910152565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576004355f604080516108ac81611b14565b82815282602082015201525f525f6020526103bc60405f2064ffffffffff604051916108d783611b14565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c16610901816107ef565b602084015260a81c16604082015260405191829182610826565b6101207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561095281610614565b6024359060443560643560843567ffffffffffffffff81116101d35761097c90369060040161063a565b61098461067a565b9160c4359360e43595610104359867ffffffffffffffff8a116101d3576109b26107719a3690600401610692565b98611da5565b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602060ff60035460401c166040519015158152f35b346101d35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435610a3681610259565b60243590610a4382610259565b6044356064359260843567ffffffffffffffff81116101d357610a6a90369060040161063a565b909460a43593303303610b3057610b2196610b1c9382610a8e610af094888b6120a4565b73ffffffffffffffffffffffffffffffffffffffff604051998a967f3866c1dc000000000000000000000000000000000000000000000000000000006020890152602488015216604486015260648501526080608485015260a4840191611f4f565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101855284611b51565b61364d565b91905015610b2b57005b61368b565b7f14d4a4e8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760206040516e22d473030f116ddee9f6b43ac78ba38152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357610bd061206d565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a35f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755005b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101d35760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d357610ce6903690600401610466565b602435610cf281610259565b60443567ffffffffffffffff81116101d357610d129036906004016106a0565b688000000000ab143c065c6103e8576001688000000000ab143c065d60ff60035460401c166105b157610d4582846122cc565b959291849791949585610100840135823592602081013560405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152610d9c606082611b51565b5190209060405192610daf60a085611b51565b606a8452602084017f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c41759052604084017f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b659052606084017f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e74329052608084017f353620616d6f756e742900000000000000000000000000000000000000000000905260408101610e5c91611a89565b95610e6a9791958c8c614477565b84610e796101e0840184611a89565b936101c001610e8790611ada565b933690610e9392611c09565b91610e9e9488612696565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a35f688000000000ab143c065d604051908152602090f35b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576020600254604051908152f35b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435610ff561206d565b670de0b6b3a7640000811161100957600255005b7fad6bb6d1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b346101d35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d3576110f3903690600401610466565b611137602435610100830135926101408101359161111083610668565b6101608201359061112082610668565b610180830135916101a06002549401359487613714565b9081810390811161115357604080519182526020820192909252f35b611f8d565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d357366023820112156101d357806004013567ffffffffffffffff81116101d3578060051b36602482850101116101d357346101d3576040519283926020845280846020015260408481019380602485018637850101928391611244575b505050806040520360401b1761381e565b919350915b80915f6044825187016024810135918291018537389084305af4156112d057602067ffffffffffffffe0603f5f937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08a87030181528301943d81523d858583013e3d010116933d01015284838210156112c3575090611249565b93505090505f8080611233565b853d5f823e3d90fd5b60a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561130f81610259565b60443567ffffffffffffffff81116101d35761132f90369060040161063a565b61133a606435610259565b60843567ffffffffffffffff81116101d35761135a90369060040161063a565b5050688000000000ab143c065c6103e8576001688000000000ab143c065d60ff60035460401c166105b157604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016949181604481885afa9081156116cd5773ffffffffffffffffffffffffffffffffffffffff9161145e915f916117a0575b505173ffffffffffffffffffffffffffffffffffffffff1690565b16801561176e576040517f5e280f11000000000000000000000000000000000000000000000000000000008152602081600481855afa80156116cd5773ffffffffffffffffffffffffffffffffffffffff915f9161173f575b501633036117135773ffffffffffffffffffffffffffffffffffffffff8216036116d257506114e6818361382f565b61156860a061151461150f61055a6114fe8789613841565b96611509818a613850565b9861385f565b6138ef565b95611526602088015163ffffffff1690565b9060405180809581947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa9081156116cd575f9161169e575b5061158c6115888251151590565b1590565b908115611686575b5061165557506115b8305b73ffffffffffffffffffffffffffffffffffffffff1690565b810361162a5750604082015163ffffffff1691468390036115f5576115e792506115e181613a2c565b90613c97565b5f688000000000ab143c065d005b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5263ffffffff831660045260245ffd5b5ffd5b7f25ea23d9000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fe73d23c3000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6040015163ffffffff8381169116141590505f611594565b6116c0915060a03d60a0116116c6575b6116b88183611b51565b810190611fe7565b5f61157a565b503d6116ae565b611f44565b7f89233a72000000000000000000000000000000000000000000000000000000005f5273ffffffffffffffffffffffffffffffffffffffff1660045260245ffd5b7f39e7e94b000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b611761915060203d602011611767575b6117598183611b51565b810190611fc7565b5f6114b7565b503d61174f565b7fb825dd76000000000000000000000000000000000000000000000000000000005f524663ffffffff1660045260245ffd5b6117c2915060403d6040116117c8575b6117ba8183611b51565b810190611f0a565b5f611443565b503d6117b0565b346101d35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602061185f60043561180f81610259565b73ffffffffffffffffffffffffffffffffffffffff6024359161183183610259565b165f526001835260405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b54604051908152f35b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561189e81610259565b6118a661206d565b63389a75e1600c52805f526020600c2090815442116118cc575f6118ca9255613693565b005b636f5e88185f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561190f81610259565b61191761206d565b8060601b15611929576118ca90613693565b637448fbae5f526004601cfd5b60ff8116036101d357565b359061028582611936565b346101d35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561198781610259565b6024356044356064359261199a84611936565b73ffffffffffffffffffffffffffffffffffffffff169160a43590608435843b156101d3575f9460e493869260ff604051998a9889977fd505accf0000000000000000000000000000000000000000000000000000000089523360048a01523060248a01526044890152606488015216608486015260a485015260c48401525af1611a2157005b80611a2f5f6118ca93611b51565b806103f5565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435611a7081610259565b63389a75e1600c525f52602080600c2054604051908152f35b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101d3570180359067ffffffffffffffff82116101d3576020019181360383136101d357565b35611ae481610668565b90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6060810190811067ffffffffffffffff821117611b3057604052565b611ae7565b6040810190811067ffffffffffffffff821117611b3057604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117611b3057604052565b6040519061028561020083611b51565b6040519061028560e083611b51565b60405190610285604083611b51565b60405190610285606083611b51565b67ffffffffffffffff8111611b3057601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b929192611c1582611bcf565b91611c236040519384611b51565b8294818452818301116101d3578281602093845f960137010152565b9397929b9a96919b99959094989960ff60035460401c166105b157611c62612959565b9c611c6f968e9689612a21565b94803590611c7c82610668565b6101c087019167ffffffffffffffff16825260208101611c9b91611a89565b6101e088019291611cad913691611c09565b8252611cb887612cd0565b611cc3368787611c09565b8051602090910120875160ff16916040890151611ce39063ffffffff1690565b9160808a0151948a60e0810196875161010083015190610160840151611d109067ffffffffffffffff1690565b61014085015167ffffffffffffffff16611d2991612ecd565b92610180850151946101a001519551805190602001209751611d529067ffffffffffffffff1690565b98611d5c9a614317565b9260a0870151611d6b90613f1f565b9151936020820135823586611d7e612eef565b9460408101611d8c91611a89565b97611d98999197614477565b611da192613085565b9190565b9099989496939592919760ff60035460401c166105b157611e039461055a94611dda948b611dd1612959565b9e8f9633612a21565b92611df9611de782611ada565b67ffffffffffffffff166101c0860152565b6020810190611a89565b6101e0820152611e1281612cd0565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa9485156116cd57611edd6020611da197611ee6945f91611eeb575b50015173ffffffffffffffffffffffffffffffffffffffff1690565b309033906124f3565b613085565b611f04915060403d6040116117c8576117ba8183611b51565b5f611ec1565b908160409103126101d357602060405191611f2483611b35565b8051611f2f81610259565b83520151611f3c81610259565b602082015290565b6040513d5f823e3d90fd5b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9190820391821161115357565b908160209103126101d35751611ae481610259565b519061028582610614565b908160a09103126101d3576040519060a082019082821067ffffffffffffffff831117611b3057608091604052805161201f816101c9565b8352602081015161202f81610614565b6020840152604081015161204281610614565b6040840152606081015161205581610259565b6060840152015161206581610259565b608082015290565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392754330361209757565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f511416156120e0575b50505f603452565b3b153d1710156120f1575f806120d8565b6390b8ec185f526004601cfd5b35611ae481611936565b35611ae481610614565b9080601f830112156101d357816020611ae493359101611c09565b919091610200818403126101d357612143611b92565b9261214d82611941565b845261215b6020830161062f565b602085015261216c6040830161062f565b6040850152606082013560608501526080820135608085015260a082013560a085015260c082013560c085015260e082013560e08501526101008201356101008501526121bc6101208301610687565b6101208501526121cf6101408301610687565b6101408501526121e26101608301610687565b6101608501526101808201356101808501526101a08201356101a085015261220d6101c08301610687565b6101c08501526101e082013567ffffffffffffffff81116101d3576122329201612112565b6101e0830152565b61224460016107ef565b740100000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b61228d60026107ef565b740200000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b9190916122d8816120fe565b60ff806001169116036124b457604081016122fe6122f582612108565b63ffffffff1690565b46036124775750612317612312368361212d565b613a2c565b92612329845f525f60205260405f2090565b91612339835460ff9060a01c1690565b612342816107ef565b61244a57612358612353368361212d565b613e0b565b936102856123696080840135613f1f565b946123f86123b66123af610100870135966123876101408201611ada565b906123956101608201611ada565b91610180820135906101a06002549301359342918c613714565b8096611fba565b95829073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b6124018161223a565b80547fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff164260a81b79ffffffffff00000000000000000000000000000000000000000016179055565b7f343e211e000000000000000000000000000000000000000000000000000000005f52600485905260245ffd5b61248361162791612108565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5263ffffffff16600452602490565b6124c0611627916120fe565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f52600160045260ff16602452604490565b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f5114161561253c575b50505f606052604052565b3b153d17101561254d575f80612531565b637939f4245f526004601cfd5b67ffffffffffffffff61c3509116019067ffffffffffffffff821161115357565b9067ffffffffffffffff8091169116019067ffffffffffffffff821161115357565b94909367ffffffffffffffff9373ffffffffffffffffffffffffffffffffffffffff6125ee948160a0989b9a9b1689521660208801526040870152606086015260c0608086015260c0850190611158565b9416910152565b905f6020830192612605826107ef565b52565b9060026020830192612605826107ef565b9060016020830192612605826107ef565b60405190612639602083611b51565b5f8252565b3d15612668573d9061264f82611bcf565b9161265d6040519384611b51565b82523d5f602084013e565b606090565b90601f820180921161115357565b601201908160121161115357565b9190820180921161115357565b93949091948315612951578051158015612948575b61293b575a6126e06126d36126ce600586901c6707ffffffffffffff168661257b565b61255a565b67ffffffffffffffff1690565b116128fe57303b156101d35761272a915f9160405193849283927f617c537c00000000000000000000000000000000000000000000000000000000845289898c8a6004880161259d565b038183305af190816128ea575b506128a25761274c61274761263e565b6140ae565b9073ffffffffffffffffffffffffffffffffffffffff82169485151580612898575b156127b3575090612780929184613f6b565b7f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9604051806127ae81612619565b0390a3565b73ffffffffffffffffffffffffffffffffffffffff80965082935081612840879461281d7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9573ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b9073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b61284b878254612689565b9055169586936128676040519283921696829190602083019252565b0390a47f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9604051806127ae81612608565b503086141561276e565b50507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a973ffffffffffffffffffffffffffffffffffffffff604051931692806127ae816125f5565b80611a2f5f6128f893611b51565b5f612737565b611627825a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045267ffffffffffffffff16602452604490565b5050610285939192613f6b565b50853b156126ab565b505050505050565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff81116111535767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b6129b8611b92565b905f82525f60208301525f60408301525f60608301525f60808301525f60a08301525f60c08301525f60e08301525f6101008301525f6101208301525f6101408301525f6101608301525f6101808301525f6101a08301525f6101c083015260606101e0830152565b94939197929097612a306129b0565b50604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff8116602483015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169892909190816044818c5afa9182156116cd57612b006020612b4c946040945f91612cb95750015173ffffffffffffffffffffffffffffffffffffffff1690565b998d83518095819482937f320005c50000000000000000000000000000000000000000000000000000000084526004840190929160ff60209163ffffffff604085019616845216910152565b03915afa9081156116cd57612b80916020915f91611eeb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b9073ffffffffffffffffffffffffffffffffffffffff88161561176e5773ffffffffffffffffffffffffffffffffffffffff821615612c875773ffffffffffffffffffffffffffffffffffffffff9081169767ffffffffffffffff42169792821693911691612bef908861257b565b97612bf8611b92565b600181529b63ffffffff461660208e015263ffffffff1660408d015260608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015267ffffffffffffffff1661014085015267ffffffffffffffff166101608401526101808301526101a08201525f6101c0820152612c7e61262a565b6101e082015290565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff8b1660045260245ffd5b611f049150853d87116117c8576117ba8183611b51565b602081015163ffffffff16468103612e9c5750612cfc612cf7604083015163ffffffff1690565b61412d565b61016081015161014082015167ffffffffffffffff918216911680821115612e615750506101008101805180158015612e54575b612e1e57506101a082015190519081811015612df057505073ffffffffffffffffffffffffffffffffffffffff612d6a6080830151613f1f565b1615612dc8576101c0015167ffffffffffffffff16624c4b408111612d8c5750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff16600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e08301517f8dd09d91000000000000000000000000000000000000000000000000000000005f5260049190915260245260445ffd5b5060e08301518111612d30565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff9081166004521660245260445ffd5b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b9067ffffffffffffffff8091169116039067ffffffffffffffff821161115357565b60405190612eff61016083611b51565b61012982527f3620616d6f756e74290000000000000000000000000000000000000000000000610140837f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208201527f75696e743820627269646765547970652c75696e74333220647374436861696e60408201527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608201527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808201527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08201527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08201527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08201527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008201527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208201520152565b9092919261309282613a2c565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff166024820152919573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016939092909181604481875afa9081156116cd575f9161362e575b5061316061159f61159f835173ffffffffffffffffffffffffffffffffffffffff1690565b604051917ffc0c546a000000000000000000000000000000000000000000000000000000008352602083600481855afa9283156116cd575f93613608575b506020015173ffffffffffffffffffffffffffffffffffffffff165b73ffffffffffffffffffffffffffffffffffffffff811673ffffffffffffffffffffffffffffffffffffffff8416036135c157506040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a081602481895afa9081156116cd57613244916040915f9161351d575b50015163ffffffff1690565b604051907f5e280f11000000000000000000000000000000000000000000000000000000008252602082600481865afa9182156116cd5773ffffffffffffffffffffffffffffffffffffffff926020915f916135a4575b506004604051809581937f416ecebf000000000000000000000000000000000000000000000000000000008352165afa9182156116cd575f92613573575b5063ffffffff811663ffffffff83160361353c5750509061335d929161330660e088019282845191614796565b60a0604088019661331b885163ffffffff1690565b9060405180809881947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa9283156116cd576133cc613389604060c09661342a985f9161351d5750015163ffffffff1690565b9351916101008a0197885161339d8c61483f565b946133b56133a9611ba2565b63ffffffff9099168952565b306020890152604088015260608701523691611c09565b608084015260a08301526133de61262a565b838301526133ea611bb1565b3481525f60208201526040518095819482937fc7c7f5b30000000000000000000000000000000000000000000000000000000084523391600485016145ec565b039134905af19283156116cd576127ae7f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a9373ffffffffffffffffffffffffffffffffffffffff9389966134ef575b5061349561348a6060890151613f1f565b935163ffffffff1690565b966134b361012060c0830151935192015167ffffffffffffffff1690565b604080516001815263ffffffff909a1660208b0152890192909252606088015267ffffffffffffffff16608087015291169390819060a0820190565b6135109060c03d60c011613516575b6135088183611b51565b810190614596565b50613479565b503d6134fe565b613536915060a03d60a0116116c6576116b88183611b51565b5f613238565b7fb9190bb3000000000000000000000000000000000000000000000000000000005f5263ffffffff9081166004521660245260445ffd5b61359691925060203d60201161359d575b61358e8183611b51565b810190614559565b905f6132d9565b503d613584565b6135bb9150823d8411611767576117598183611b51565b5f61329b565b7ff902523f000000000000000000000000000000000000000000000000000000005f5273ffffffffffffffffffffffffffffffffffffffff8084166004521660245260445ffd5b6131ba919350613626602091823d8411611767576117598183611b51565b93915061319e565b613647915060403d6040116117c8576117ba8183611b51565b5f61313b565b5f9081936040519383389360208451940192f1923d60243d1161367f575b806020918452805f8386013e830101604052565b5060019250602461366b565b805190602001fd5b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b959290949167ffffffffffffffff8091169516908181105f146138165750925b5f9385811061375c575b50505061374b9250612689565b81811115613757575090565b905090565b90919293508403938411611153578015801590816137b1575b5093613799929161374b955f146137aa575050805b8082116137a2575b50846146e3565b905f808061373e565b90505f613792565b029061378a565b90509190916137e957907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8190048411613799613775565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b905092613734565b8060401c9067ffffffffffffffff16f35b90600c116101d3576008013560e01c90565b90604c116101d357602c013590565b90602c116101d357600c013590565b91909182604c116101d357604c01917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb40190565b519061028582611936565b519061028582610668565b81601f820112156101d3578051906138c082611bcf565b926138ce6040519485611b51565b828452602083830101116101d357815f9260208093018386015e8301015290565b6138f76129b0565b5080518101906020818303126101d35760208101519067ffffffffffffffff82116101d3570190610200828203126101d357613931611b92565b9161393e60208201613893565b835261394c60408201611fdc565b602084015261395d60608201611fdc565b60408401526080810151606084015260a0810151608084015260c081015160a084015260e081015160c084015261010081015160e08401526101208101516101008401526139ae610140820161389e565b6101208401526139c1610160820161389e565b6101408401526139d4610180820161389e565b6101608401526101a08101516101808401526101c08101516101a08401526139ff6101e0820161389e565b6101c08401526102008101519167ffffffffffffffff83116101d357612c7e9260208092019201016138a9565b906101e08201613bc2613bb86126d36101c0613a72613a5d613a58613a5288515161266d565b60051c90565b61267b565b90604051828193825260010160051b01604052565b60208082015296613a99613a90613a8a835160ff1690565b60ff1690565b60408a01528890565b50613aba613ab16122f5602084015163ffffffff1690565b60608a01528890565b50613adb613ad26122f5604084015163ffffffff1690565b60808a01528890565b50606081015160a0890152608081015160c089015260a081015160e089015260c081015161010089015260e0810151610120890152610100810151610140890152613b42613b386126d361012084015167ffffffffffffffff1690565b6101608a01528890565b50613b69613b5f6126d361014084015167ffffffffffffffff1690565b6101808a01528890565b50613b90613b866126d361016084015167ffffffffffffffff1690565b6101a08a01528890565b506101808101516101c08901526101a08101516101e0890152015167ffffffffffffffff1690565b6102008501528390565b5061020061022084015280515161024084015251805180613c07575b5050815160051b602083012061028590928051604051828260010160051b011490151060061b52565b602082016020826102608701940101905b818110613c87575050601f16908115613bde5760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80613bde565b8051845260209384019301613c18565b91613ca9825f525f60205260405f2090565b805490600260a083901c60ff16613cbf816107ef565b14613dde5791613d43917f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a93613cf487613e0b565b91613d1f613d0560808a0151613f1f565b9173ffffffffffffffffffffffffffffffffffffffff1690565b61010089015180861015613dd8575084905b613d3b8287611fba565b968794612283565b73ffffffffffffffffffffffffffffffffffffffff811699868b15613da557505090613d7091858a613f6b565b81613d93575b5050505b60408051918252602082019290925290819081016127ae565b613d9d9287613f6b565b5f8281613d76565b9250929050613dd3949350613dcc6101c06101e085015194015167ffffffffffffffff1690565b9389612696565b613d7a565b90613d31565b7fb196a44a000000000000000000000000000000000000000000000000000000005f52600484905260245ffd5b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa80156116cd576020613ed09160c0935f91611eeb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b91015173ffffffffffffffffffffffffffffffffffffffff82168103613ef4575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8060a01c613f405773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9083156140a857604051925f80602086017fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84169687602482015288604482015260448152613fd2606482611b51565b519082865af1613fe061263e565b81614070575b50614069578161404e7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9361281d73ffffffffffffffffffffffffffffffffffffffff9473ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b614059878254612689565b90556040519586521693602090a4565b5050505050565b8051801592508215614085575b50505f613fe6565b81925090602091810103126101d357602001516140a1816101c9565b5f8061407d565b50505050565b905f9160248151146140bd5750565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff00000000000000000000000000000000000000000000000000000000602460208401519301519216146141135750565b73ffffffffffffffffffffffffffffffffffffffff169150565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815263ffffffff8216600482015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169060a081602481855afa9081156116cd575f916142f8575b50604080517f320005c500000000000000000000000000000000000000000000000000000000815263ffffffff851660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015292839060449082905afa9182156116cd575f926142cf575b5051155b908115614292575b811561426f575b5061423e5750565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6020015173ffffffffffffffffffffffffffffffffffffffff161590505f614236565b905073ffffffffffffffffffffffffffffffffffffffff6142c7825173ffffffffffffffffffffffffffffffffffffffff1690565b16159061422f565b6142279192506142f06115889160403d6040116117c8576117ba8183611b51565b929150614223565b614311915060a03d60a0116116c6576116b88183611b51565b5f6141ae565b99979593919896949290986040519960208b019b7f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b8d5260ff1660408c015263ffffffff1660608b015260808a015260a089015260c088015267ffffffffffffffff1660e087015261010086015261012085015261014084015261016083015267ffffffffffffffff1661018082015261018081526143b86101a082611b51565b51902090565b95939061446893614449611ae4999794604073ffffffffffffffffffffffffffffffffffffffff946144118c82516020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b6020818101518d84015291015160608c0152815173ffffffffffffffffffffffffffffffffffffffff1660808c0152015160a08a0152565b1660c087015260e0860152610140610100860152610140850190611158565b92610120818503910152611f4f565b95949198939098979296976144ae6040519761449460408a611b51565b73ffffffffffffffffffffffffffffffffffffffff168852565b60208701526144bb611bc0565b958652602086015260408501526144d0611bb1565b3081529460208601526e22d473030f116ddee9f6b43ac78ba33b156101d3575f956145299360405198899788977f137c29fe000000000000000000000000000000000000000000000000000000008952600489016143be565b0381836e22d473030f116ddee9f6b43ac78ba35af180156116cd5761454b5750565b80611a2f5f61028593611b51565b908160209103126101d35751611ae481610614565b91908260409103126101d35760405161458681611b35565b6020808294805184520151910152565b919082810360c081126101d3576080136101d357611ae49060806040516145bc81611b14565b8551815260208601516145ce81610668565b60208201526145e0836040880161456e565b6040820152940161456e565b6060906146c76146b2610285959796946080845263ffffffff8151166080850152602081015160a0850152604081015160c08501528481015160e085015260c061467f61464a608084015160e0610100890152610160880190611158565b60a08401517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8088830301610120890152611158565b9101517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8085830301610140860152611158565b96602083019060208091805184520151910152565b019073ffffffffffffffffffffffffffffffffffffffff169052565b81810291670de0b6b3a76400008183850414831517021561470e575050670de0b6b3a7640000900490565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff670de0b6b3a76400009284098481108501900392099080670de0b6b3a7640000111561478957828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690290565b63ae47f7025f526004601cfd5b9190601452806034526f095ea7b30000000000000000000000005f5260205f6044601082865af18060015f511416156147d3575b5050505f603452565b3d833b151710156147e5575b806147ca565b5f6034526f095ea7b30000000000000000000000005f525f386044601083865af15060345260205f6044601082855af1908160015f5114166147df573b153d171015614832575f806147df565b633e3f8f735f526004601cfd5b611ae46101e09161495760405193849260208085015261486560408501825160ff169052565b602081015163ffffffff166060850152604081015163ffffffff166080850152606081015160a0850152608081015160c085015260a081015160e085015260c081015161010085015260e08101516101208501526101008101516101408501526148e261012082015161016086019067ffffffffffffffff169052565b61014081015167ffffffffffffffff1661018085015261016081015167ffffffffffffffff166101a08501526101808101516101c08501526101a0810151828501526149416101c082015161020086019067ffffffffffffffff169052565b0151610200610220840152610240830190611158565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282611b5156fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC04a\x01IW`\x1FaK\x068\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01MW\x80\x84\x92`\x80\x94`@R\x839\x81\x01\x03\x12a\x01IWa\0G\x81a\x01aV[a\0S` \x83\x01a\x01aV[\x91```@\x82\x01Q\x91\x01Q\x92`\xFF\x84\x16\x84\x03a\x01IWg\r\xE0\xB6\xB3\xA7d\0\0\x82\x11a\x016W`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3`\x02U`\x01`\x01`\xA0\x1B\x03\x16`\x80R`\xA0R`@QaI\x90\x90\x81a\x01v\x829`\x80Q\x81\x81\x81a\x0F\\\x01R\x81\x81a\x13\xF2\x01R\x81\x81a\x1Eo\x01R\x81\x81a*\x9F\x01R\x81\x81a1\x01\x01R\x81\x81a>h\x01RaAv\x01R`\xA0Q\x81\x81\x81a\x0Cs\x01R\x81\x81a\x13\xB4\x01R\x81\x81a\x1EA\x01R\x81\x81a*`\x01R\x81\x81a0\xC1\x01R\x81\x81a>:\x01RaA\xE4\x01R\xF3[Pc\xADk\xB6\xD1`\xE0\x1B_R`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01IWV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x16\xC3\x8B<\x14a\x01\xC4W\x80c\x1E\x83@\x9A\x14a\x01\xBFW\x80c%i)b\x14a\x01\xBAW\x80c1\xEE\xE4M\x14a\x01\xB5W\x80c9\xC32\x15\x14a\x01\xB0W\x80cJk\xE4h\x14a\x01\xABW\x80cT\xD1\xF1=\x14a\x01\xA6W\x80cWxG*\x14a\x01\xA1W\x80cYu\xE7\x90\x14a\x01\x9CW\x80c\\\x97Z\xBB\x14a\x01\x97W\x80ca|S|\x14a\x01\x92W\x80cj\xFD\xD8P\x14a\x01\x8DW\x80cqP\x18\xA6\x14a\x01\x88W\x80csv\xF1\xC0\x14a\x01\x83W\x80cwo\xF3\xC7\x14a\x01~W\x80cyP,U\x14a\x01yW\x80c\x85\xC1x0\x14a\x01tW\x80c\x8C\xDA\x96\xDE\x14a\x01oW\x80c\x8D\xA5\xCB[\x14a\x01jW\x80c\x97\xC3k\xAE\x14a\x01eW\x80c\xAC\x96P\xD8\x14a\x01`W\x80c\xD0\xA1\x02`\x14a\x01[W\x80c\xD4W\x0C\x1C\x14a\x01VW\x80c\xF0N(>\x14a\x01QW\x80c\xF2\xFD\xE3\x8B\x14a\x01LW\x80c\xF3\x99\\g\x14a\x01GWc\xFE\xE8\x1C\xF4\x14a\x01BW_\x80\xFD[a\x1A5V[a\x19LV[a\x18\xD9V[a\x18hV[a\x17\xCFV[a\x12\xD9V[a\x11\x9BV[a\x10\xA4V[a\x104V[a\x0F\xBBV[a\x0F\x80V[a\x0F\x12V[a\x0C\x97V[a\x0C<V[a\x0B\x9FV[a\x0BXV[a\t\xFBV[a\t\xB8V[a\t\x1BV[a\x08lV[a\x07\x8DV[a\x06\xAEV[a\x05\xD9V[a\x04uV[a\x03\xFFV[a\x02\x87V[a\x01\xD7V[\x80\x15\x15\x03a\x01\xD3WV[_\x80\xFD[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x02\x12\x81a\x01\xC9V[a\x02\x1Aa mV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92\x15\x15`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xD3WV[a\x01$5\x90a\x02\x85\x82a\x02YV[V[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x02\xC2\x81a\x02YV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R`\x01` Ra\x03\x11\x81`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T\x80\x15a\x03\xC0Wa\x03\xBC\x913_R`\x01` R_a\x03P\x82`@\x83 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[Ua\x03\\\x823\x83a \xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83` 3\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[_\x91\x03\x12a\x01\xD3WV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[\x90\x81a\x02\0\x91\x03\x12a\x01\xD3W\x90V[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x04\xC4\x906\x90`\x04\x01a\x04fV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\x01\xC0a\x05ha\x05a\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x05\x92a\x03\xBC\x95a\x05(3\x82a\"\xCCV[\x98\x93\x99\x92\x91\x8A\x98\x82\x96a\x05=\x8403\x88a$\xF3V[a\x05Ka\x01\xE0\x82\x01\x82a\x1A\x89V[\x96\x90\x91\x015\x95a\x05Z\x87a\x06hV[6\x91a\x1C\tV[\x92\x89a&\x96V[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@QbLK@\x81R\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xD3WV[`\x045\x90a\x02\x85\x82a\x06\x14V[5\x90a\x02\x85\x82a\x06\x14V[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xD3W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xD3W` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xD3WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xD3WV[`\xA45\x90a\x02\x85\x82a\x06hV[5\x90a\x02\x85\x82a\x06hV[\x90\x81`@\x91\x03\x12a\x01\xD3W\x90V[\x90\x81``\x91\x03\x12a\x01\xD3W\x90V[a\x01`\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wa\x06\xE1a\x06\"V[`$5\x90`D5`d5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x07\x0B\x906\x90`\x04\x01a\x06:V[a\x07\x13a\x06zV[`\xC45\x91`\xE45\x93a\x01\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x07<\x906\x90`\x04\x01a\x06\x92V[\x95a\x07Ea\x02wV[\x97a\x01D5\x9Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x11a\x01\xD3Wa\x07ka\x07q\x9C6\x90`\x04\x01a\x06\xA0V[\x9Aa\x1C?V[`@\x80Q\x92\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R\x90\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[`\x03\x11\x15a\x07\xF9WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x91`@d\xFF\xFF\xFF\xFF\xFF\x81``\x84\x01\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R` \x81\x01Qa\x08_\x81a\x07\xEFV[` \x86\x01R\x01Q\x16\x91\x01RV[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045_`@\x80Qa\x08\xAC\x81a\x1B\x14V[\x82\x81R\x82` \x82\x01R\x01R_R_` Ra\x03\xBC`@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x08\xD7\x83a\x1B\x14V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16a\t\x01\x81a\x07\xEFV[` \x84\x01R`\xA8\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82a\x08&V[a\x01 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\tR\x81a\x06\x14V[`$5\x90`D5`d5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\t|\x906\x90`\x04\x01a\x06:V[a\t\x84a\x06zV[\x91`\xC45\x93`\xE45\x95a\x01\x045\x98g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x11a\x01\xD3Wa\t\xB2a\x07q\x9A6\x90`\x04\x01a\x06\x92V[\x98a\x1D\xA5V[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\n6\x81a\x02YV[`$5\x90a\nC\x82a\x02YV[`D5`d5\x92`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\nj\x906\x90`\x04\x01a\x06:V[\x90\x94`\xA45\x9303\x03a\x0B0Wa\x0B!\x96a\x0B\x1C\x93\x82a\n\x8Ea\n\xF0\x94\x88\x8Ba \xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x99\x8A\x96\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x89\x01R`$\x88\x01R\x16`D\x86\x01R`d\x85\x01R`\x80`\x84\x85\x01R`\xA4\x84\x01\x91a\x1FOV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x85R\x84a\x1BQV[a6MV[\x91\x90P\x15a\x0B+W\0[a6\x8BV[\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wa\x0B\xD0a mV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\0[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xD3W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x0C\xE6\x906\x90`\x04\x01a\x04fV[`$5a\x0C\xF2\x81a\x02YV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\r\x12\x906\x90`\x04\x01a\x06\xA0V[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\rE\x82\x84a\"\xCCV[\x95\x92\x91\x84\x97\x91\x94\x95\x85a\x01\0\x84\x015\x825\x92` \x81\x015`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\r\x9C``\x82a\x1BQV[Q\x90 \x90`@Q\x92a\r\xAF`\xA0\x85a\x1BQV[`j\x84R` \x84\x01\x7FFillAuthorization witness)FillAu\x90R`@\x84\x01\x7Fthorization(bytes32 orderId)Toke\x90R``\x84\x01\x7FnPermissions(address token,uint2\x90R`\x80\x84\x01\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90R`@\x81\x01a\x0E\\\x91a\x1A\x89V[\x95a\x0Ej\x97\x91\x95\x8C\x8CaDwV[\x84a\x0Eya\x01\xE0\x84\x01\x84a\x1A\x89V[\x93a\x01\xC0\x01a\x0E\x87\x90a\x1A\xDAV[\x936\x90a\x0E\x93\x92a\x1C\tV[\x91a\x0E\x9E\x94\x88a&\x96V[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `\x02T`@Q\x90\x81R\xF3[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x0F\xF5a mV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x10\tW`\x02U\0[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x01\xD3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x10\xF3\x906\x90`\x04\x01a\x04fV[a\x117`$5a\x01\0\x83\x015\x92a\x01@\x81\x015\x91a\x11\x10\x83a\x06hV[a\x01`\x82\x015\x90a\x11 \x82a\x06hV[a\x01\x80\x83\x015\x91a\x01\xA0`\x02T\x94\x015\x94\x87a7\x14V[\x90\x81\x81\x03\x90\x81\x11a\x11SW`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[a\x1F\x8DV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3W6`#\x82\x01\x12\x15a\x01\xD3W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3W\x80`\x05\x1B6`$\x82\x85\x01\x01\x11a\x01\xD3W4a\x01\xD3W`@Q\x92\x83\x92` \x84R\x80\x84` \x01R`@\x84\x81\x01\x93\x80`$\x85\x01\x867\x85\x01\x01\x92\x83\x91a\x12DW[PPP\x80`@R\x03`@\x1B\x17a8\x1EV[\x91\x93P\x91[\x80\x91_`D\x82Q\x87\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x12\xD0W` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?_\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8A\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x93=\x01\x01R\x84\x83\x82\x10\x15a\x12\xC3WP\x90a\x12IV[\x93PP\x90P_\x80\x80a\x123V[\x85=_\x82>=\x90\xFD[`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x13\x0F\x81a\x02YV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x13/\x906\x90`\x04\x01a\x06:V[a\x13:`d5a\x02YV[`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x13Z\x906\x90`\x04\x01a\x06:V[PPh\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xB1W`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94\x91\x81`D\x81\x88Z\xFA\x90\x81\x15a\x16\xCDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x14^\x91_\x91a\x17\xA0W[PQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x80\x15a\x17nW`@Q\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x16\xCDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91_\x91a\x17?W[P\x163\x03a\x17\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x16\xD2WPa\x14\xE6\x81\x83a8/V[a\x15h`\xA0a\x15\x14a\x15\x0Fa\x05Za\x14\xFE\x87\x89a8AV[\x96a\x15\t\x81\x8Aa8PV[\x98a8_V[a8\xEFV[\x95a\x15&` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x95\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\x16\xCDW_\x91a\x16\x9EW[Pa\x15\x8Ca\x15\x88\x82Q\x15\x15\x90V[\x15\x90V[\x90\x81\x15a\x16\x86W[Pa\x16UWPa\x15\xB80[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x81\x03a\x16*WP`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x91F\x83\x90\x03a\x15\xF5Wa\x15\xE7\x92Pa\x15\xE1\x81a:,V[\x90a<\x97V[_h\x80\0\0\0\0\xAB\x14<\x06]\0[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x83\x16`\x04R`$_\xFD[_\xFD[\x7F%\xEA#\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xE7=#\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[`@\x01Qc\xFF\xFF\xFF\xFF\x83\x81\x16\x91\x16\x14\x15\x90P_a\x15\x94V[a\x16\xC0\x91P`\xA0=`\xA0\x11a\x16\xC6W[a\x16\xB8\x81\x83a\x1BQV[\x81\x01\x90a\x1F\xE7V[_a\x15zV[P=a\x16\xAEV[a\x1FDV[\x7F\x89#:r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[\x7F9\xE7\xE9K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[a\x17a\x91P` =` \x11a\x17gW[a\x17Y\x81\x83a\x1BQV[\x81\x01\x90a\x1F\xC7V[_a\x14\xB7V[P=a\x17OV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_RFc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[a\x17\xC2\x91P`@=`@\x11a\x17\xC8W[a\x17\xBA\x81\x83a\x1BQV[\x81\x01\x90a\x1F\nV[_a\x14CV[P=a\x17\xB0V[4a\x01\xD3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` a\x18_`\x045a\x18\x0F\x81a\x02YV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91a\x181\x83a\x02YV[\x16_R`\x01\x83R`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x18\x9E\x81a\x02YV[a\x18\xA6a mV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x90\x81TB\x11a\x18\xCCW_a\x18\xCA\x92Ua6\x93V[\0[co^\x88\x18_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x19\x0F\x81a\x02YV[a\x19\x17a mV[\x80``\x1B\x15a\x19)Wa\x18\xCA\x90a6\x93V[ctH\xFB\xAE_R`\x04`\x1C\xFD[`\xFF\x81\x16\x03a\x01\xD3WV[5\x90a\x02\x85\x82a\x196V[4a\x01\xD3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x19\x87\x81a\x02YV[`$5`D5`d5\x92a\x19\x9A\x84a\x196V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91`\xA45\x90`\x845\x84;\x15a\x01\xD3W_\x94`\xE4\x93\x86\x92`\xFF`@Q\x99\x8A\x98\x89\x97\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R3`\x04\x8A\x01R0`$\x8A\x01R`D\x89\x01R`d\x88\x01R\x16`\x84\x86\x01R`\xA4\x85\x01R`\xC4\x84\x01RZ\xF1a\x1A!W\0[\x80a\x1A/_a\x18\xCA\x93a\x1BQV[\x80a\x03\xF5V[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x1Ap\x81a\x02YV[c8\x9Au\xE1`\x0CR_R` \x80`\x0C T`@Q\x90\x81R\xF3[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xD3W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xD3W` \x01\x91\x816\x03\x83\x13a\x01\xD3WV[5a\x1A\xE4\x81a\x06hV[\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1B0W`@RV[a\x1A\xE7V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1B0W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1B0W`@RV[`@Q\x90a\x02\x85a\x02\0\x83a\x1BQV[`@Q\x90a\x02\x85`\xE0\x83a\x1BQV[`@Q\x90a\x02\x85`@\x83a\x1BQV[`@Q\x90a\x02\x85``\x83a\x1BQV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B0W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a\x1C\x15\x82a\x1B\xCFV[\x91a\x1C#`@Q\x93\x84a\x1BQV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xD3W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x93\x97\x92\x9B\x9A\x96\x91\x9B\x99\x95\x90\x94\x98\x99`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\x1Cba)YV[\x9Ca\x1Co\x96\x8E\x96\x89a*!V[\x94\x805\x90a\x1C|\x82a\x06hV[a\x01\xC0\x87\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x81\x01a\x1C\x9B\x91a\x1A\x89V[a\x01\xE0\x88\x01\x92\x91a\x1C\xAD\x916\x91a\x1C\tV[\x82Ra\x1C\xB8\x87a,\xD0V[a\x1C\xC36\x87\x87a\x1C\tV[\x80Q` \x90\x91\x01 \x87Q`\xFF\x16\x91`@\x89\x01Qa\x1C\xE3\x90c\xFF\xFF\xFF\xFF\x16\x90V[\x91`\x80\x8A\x01Q\x94\x8A`\xE0\x81\x01\x96\x87Qa\x01\0\x83\x01Q\x90a\x01`\x84\x01Qa\x1D\x10\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01@\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D)\x91a.\xCDV[\x92a\x01\x80\x85\x01Q\x94a\x01\xA0\x01Q\x95Q\x80Q\x90` \x01 \x97Qa\x1DR\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x98a\x1D\\\x9AaC\x17V[\x92`\xA0\x87\x01Qa\x1Dk\x90a?\x1FV[\x91Q\x93` \x82\x015\x825\x86a\x1D~a.\xEFV[\x94`@\x81\x01a\x1D\x8C\x91a\x1A\x89V[\x97a\x1D\x98\x99\x91\x97aDwV[a\x1D\xA1\x92a0\x85V[\x91\x90V[\x90\x99\x98\x94\x96\x93\x95\x92\x91\x97`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\x1E\x03\x94a\x05Z\x94a\x1D\xDA\x94\x8Ba\x1D\xD1a)YV[\x9E\x8F\x963a*!V[\x92a\x1D\xF9a\x1D\xE7\x82a\x1A\xDAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x86\x01RV[` \x81\x01\x90a\x1A\x89V[a\x01\xE0\x82\x01Ra\x1E\x12\x81a,\xD0V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x94\x85\x15a\x16\xCDWa\x1E\xDD` a\x1D\xA1\x97a\x1E\xE6\x94_\x91a\x1E\xEBW[P\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[0\x903\x90a$\xF3V[a0\x85V[a\x1F\x04\x91P`@=`@\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[_a\x1E\xC1V[\x90\x81`@\x91\x03\x12a\x01\xD3W` `@Q\x91a\x1F$\x83a\x1B5V[\x80Qa\x1F/\x81a\x02YV[\x83R\x01Qa\x1F<\x81a\x02YV[` \x82\x01R\x90V[`@Q=_\x82>=\x90\xFD[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x03\x91\x82\x11a\x11SWV[\x90\x81` \x91\x03\x12a\x01\xD3WQa\x1A\xE4\x81a\x02YV[Q\x90a\x02\x85\x82a\x06\x14V[\x90\x81`\xA0\x91\x03\x12a\x01\xD3W`@Q\x90`\xA0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x1B0W`\x80\x91`@R\x80Qa \x1F\x81a\x01\xC9V[\x83R` \x81\x01Qa /\x81a\x06\x14V[` \x84\x01R`@\x81\x01Qa B\x81a\x06\x14V[`@\x84\x01R``\x81\x01Qa U\x81a\x02YV[``\x84\x01R\x01Qa e\x81a\x02YV[`\x80\x82\x01R\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a \x97WV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a \xE0W[PP_`4RV[;\x15=\x17\x10\x15a \xF1W_\x80a \xD8V[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5a\x1A\xE4\x81a\x196V[5a\x1A\xE4\x81a\x06\x14V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xD3W\x81` a\x1A\xE4\x935\x91\x01a\x1C\tV[\x91\x90\x91a\x02\0\x81\x84\x03\x12a\x01\xD3Wa!Ca\x1B\x92V[\x92a!M\x82a\x19AV[\x84Ra![` \x83\x01a\x06/V[` \x85\x01Ra!l`@\x83\x01a\x06/V[`@\x85\x01R``\x82\x015``\x85\x01R`\x80\x82\x015`\x80\x85\x01R`\xA0\x82\x015`\xA0\x85\x01R`\xC0\x82\x015`\xC0\x85\x01R`\xE0\x82\x015`\xE0\x85\x01Ra\x01\0\x82\x015a\x01\0\x85\x01Ra!\xBCa\x01 \x83\x01a\x06\x87V[a\x01 \x85\x01Ra!\xCFa\x01@\x83\x01a\x06\x87V[a\x01@\x85\x01Ra!\xE2a\x01`\x83\x01a\x06\x87V[a\x01`\x85\x01Ra\x01\x80\x82\x015a\x01\x80\x85\x01Ra\x01\xA0\x82\x015a\x01\xA0\x85\x01Ra\"\ra\x01\xC0\x83\x01a\x06\x87V[a\x01\xC0\x85\x01Ra\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\"2\x92\x01a!\x12V[a\x01\xE0\x83\x01RV[a\"D`\x01a\x07\xEFV[t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[a\"\x8D`\x02a\x07\xEFV[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[\x91\x90\x91a\"\xD8\x81a \xFEV[`\xFF\x80`\x01\x16\x91\x16\x03a$\xB4W`@\x81\x01a\"\xFEa\"\xF5\x82a!\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[F\x03a$wWPa#\x17a#\x126\x83a!-V[a:,V[\x92a#)\x84_R_` R`@_ \x90V[\x91a#9\x83T`\xFF\x90`\xA0\x1C\x16\x90V[a#B\x81a\x07\xEFV[a$JWa#Xa#S6\x83a!-V[a>\x0BV[\x93a\x02\x85a#i`\x80\x84\x015a?\x1FV[\x94a#\xF8a#\xB6a#\xAFa\x01\0\x87\x015\x96a#\x87a\x01@\x82\x01a\x1A\xDAV[\x90a#\x95a\x01`\x82\x01a\x1A\xDAV[\x91a\x01\x80\x82\x015\x90a\x01\xA0`\x02T\x93\x015\x93B\x91\x8Ca7\x14V[\x80\x96a\x1F\xBAV[\x95\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a$\x01\x81a\":V[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B`\xA8\x1By\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x90UV[\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$_\xFD[a$\x83a\x16'\x91a!\x08V[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$\x90V[a$\xC0a\x16'\x91a \xFEV[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`\xFF\x16`$R`D\x90V[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a%<W[PP_``R`@RV[;\x15=\x17\x10\x15a%MW_\x80a%1V[cy9\xF4$_R`\x04`\x1C\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\xC3P\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11SWV[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11SWV[\x94\x90\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa%\xEE\x94\x81`\xA0\x98\x9B\x9A\x9B\x16\x89R\x16` \x88\x01R`@\x87\x01R``\x86\x01R`\xC0`\x80\x86\x01R`\xC0\x85\x01\x90a\x11XV[\x94\x16\x91\x01RV[\x90_` \x83\x01\x92a&\x05\x82a\x07\xEFV[RV[\x90`\x02` \x83\x01\x92a&\x05\x82a\x07\xEFV[\x90`\x01` \x83\x01\x92a&\x05\x82a\x07\xEFV[`@Q\x90a&9` \x83a\x1BQV[_\x82RV[=\x15a&hW=\x90a&O\x82a\x1B\xCFV[\x91a&]`@Q\x93\x84a\x1BQV[\x82R=_` \x84\x01>V[``\x90V[\x90`\x1F\x82\x01\x80\x92\x11a\x11SWV[`\x12\x01\x90\x81`\x12\x11a\x11SWV[\x91\x90\x82\x01\x80\x92\x11a\x11SWV[\x93\x94\x90\x91\x94\x83\x15a)QW\x80Q\x15\x80\x15a)HW[a);WZa&\xE0a&\xD3a&\xCE`\x05\x86\x90\x1Cg\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a%{V[a%ZV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x11a(\xFEW0;\x15a\x01\xD3Wa'*\x91_\x91`@Q\x93\x84\x92\x83\x92\x7Fa|S|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x89\x89\x8C\x8A`\x04\x88\x01a%\x9DV[\x03\x81\x830Z\xF1\x90\x81a(\xEAW[Pa(\xA2Wa'La'Ga&>V[a@\xAEV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x94\x85\x15\x15\x80a(\x98W[\x15a'\xB3WP\x90a'\x80\x92\x91\x84a?kV[\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a'\xAE\x81a&\x19V[\x03\x90\xA3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x96P\x82\x93P\x81a(@\x87\x94a(\x1D\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[a(K\x87\x82Ta&\x89V[\x90U\x16\x95\x86\x93a(g`@Q\x92\x83\x92\x16\x96\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA4\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a'\xAE\x81a&\x08V[P0\x86\x14\x15a'nV[PP\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x93\x16\x92\x80a'\xAE\x81a%\xF5V[\x80a\x1A/_a(\xF8\x93a\x1BQV[_a'7V[a\x16'\x82Z\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$R`D\x90V[PPa\x02\x85\x93\x91\x92a?kV[P\x85;\x15a&\xABV[PPPPPPV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x11SWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[a)\xB8a\x1B\x92V[\x90_\x82R_` \x83\x01R_`@\x83\x01R_``\x83\x01R_`\x80\x83\x01R_`\xA0\x83\x01R_`\xC0\x83\x01R_`\xE0\x83\x01R_a\x01\0\x83\x01R_a\x01 \x83\x01R_a\x01@\x83\x01R_a\x01`\x83\x01R_a\x01\x80\x83\x01R_a\x01\xA0\x83\x01R_a\x01\xC0\x83\x01R``a\x01\xE0\x83\x01RV[\x94\x93\x91\x97\x92\x90\x97a*0a)\xB0V[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x81\x16`$\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x98\x92\x90\x91\x90\x81`D\x81\x8CZ\xFA\x91\x82\x15a\x16\xCDWa+\0` a+L\x94`@\x94_\x91a,\xB9WP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x99\x8D\x83Q\x80\x95\x81\x94\x82\x93\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\xFF` \x91c\xFF\xFF\xFF\xFF`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x16\xCDWa+\x80\x91` \x91_\x91a\x1E\xEBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x15a\x17nWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a,\x87Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x97g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x97\x92\x82\x16\x93\x91\x16\x91a+\xEF\x90\x88a%{V[\x97a+\xF8a\x1B\x92V[`\x01\x81R\x9Bc\xFF\xFF\xFF\xFFF\x16` \x8E\x01Rc\xFF\xFF\xFF\xFF\x16`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01R_a\x01\xC0\x82\x01Ra,~a&*V[a\x01\xE0\x82\x01R\x90V[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04R`$_\xFD[a\x1F\x04\x91P\x85=\x87\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16F\x81\x03a.\x9CWPa,\xFCa,\xF7`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[aA-V[a\x01`\x81\x01Qa\x01@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x80\x82\x11\x15a.aWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a.TW[a.\x1EWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a-\xF0WPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa-j`\x80\x83\x01Qa?\x1FV[\x16\x15a-\xC8Wa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a-\x8CWPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x91\x90\x91R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a-0V[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11SWV[`@Q\x90a.\xFFa\x01`\x83a\x1BQV[a\x01)\x82R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x83\x7FOrderIntent witness)OrderIntent(` \x82\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x82\x01R\x7FId,bytes32 recipient,uint256 inp``\x82\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x82\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x82\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x82\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x82\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x82\x01R\x7FPermissions(address token,uint25a\x01 \x82\x01R\x01RV[\x90\x92\x91\x92a0\x92\x82a:,V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x91\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x90\x92\x90\x91\x81`D\x81\x87Z\xFA\x90\x81\x15a\x16\xCDW_\x91a6.W[Pa1`a\x15\x9Fa\x15\x9F\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x91\x7F\xFC\x0CTj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` \x83`\x04\x81\x85Z\xFA\x92\x83\x15a\x16\xCDW_\x93a6\x08W[P` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x03a5\xC1WP`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x89Z\xFA\x90\x81\x15a\x16\xCDWa2D\x91`@\x91_\x91a5\x1DW[P\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x90\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x86Z\xFA\x91\x82\x15a\x16\xCDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92` \x91_\x91a5\xA4W[P`\x04`@Q\x80\x95\x81\x93\x7FAn\xCE\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x91\x82\x15a\x16\xCDW_\x92a5sW[Pc\xFF\xFF\xFF\xFF\x81\x16c\xFF\xFF\xFF\xFF\x83\x16\x03a5<WPP\x90a3]\x92\x91a3\x06`\xE0\x88\x01\x92\x82\x84Q\x91aG\x96V[`\xA0`@\x88\x01\x96a3\x1B\x88Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x98\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x92\x83\x15a\x16\xCDWa3\xCCa3\x89`@`\xC0\x96a4*\x98_\x91a5\x1DWP\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x93Q\x91a\x01\0\x8A\x01\x97\x88Qa3\x9D\x8CaH?V[\x94a3\xB5a3\xA9a\x1B\xA2V[c\xFF\xFF\xFF\xFF\x90\x99\x16\x89RV[0` \x89\x01R`@\x88\x01R``\x87\x01R6\x91a\x1C\tV[`\x80\x84\x01R`\xA0\x83\x01Ra3\xDEa&*V[\x83\x83\x01Ra3\xEAa\x1B\xB1V[4\x81R_` \x82\x01R`@Q\x80\x95\x81\x94\x82\x93\x7F\xC7\xC7\xF5\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R3\x91`\x04\x85\x01aE\xECV[\x03\x914\x90Z\xF1\x92\x83\x15a\x16\xCDWa'\xAE\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x89\x96a4\xEFW[Pa4\x95a4\x8A``\x89\x01Qa?\x1FV[\x93Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96a4\xB3a\x01 `\xC0\x83\x01Q\x93Q\x92\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@\x80Q`\x01\x81Rc\xFF\xFF\xFF\xFF\x90\x9A\x16` \x8B\x01R\x89\x01\x92\x90\x92R``\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01R\x91\x16\x93\x90\x81\x90`\xA0\x82\x01\x90V[a5\x10\x90`\xC0=`\xC0\x11a5\x16W[a5\x08\x81\x83a\x1BQV[\x81\x01\x90aE\x96V[Pa4yV[P=a4\xFEV[a56\x91P`\xA0=`\xA0\x11a\x16\xC6Wa\x16\xB8\x81\x83a\x1BQV[_a28V[\x7F\xB9\x19\x0B\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[a5\x96\x91\x92P` =` \x11a5\x9DW[a5\x8E\x81\x83a\x1BQV[\x81\x01\x90aEYV[\x90_a2\xD9V[P=a5\x84V[a5\xBB\x91P\x82=\x84\x11a\x17gWa\x17Y\x81\x83a\x1BQV[_a2\x9BV[\x7F\xF9\x02R?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16`\x04R\x16`$R`D_\xFD[a1\xBA\x91\x93Pa6&` \x91\x82=\x84\x11a\x17gWa\x17Y\x81\x83a\x1BQV[\x93\x91Pa1\x9EV[a6G\x91P`@=`@\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[_a1;V[_\x90\x81\x93`@Q\x93\x838\x93` \x84Q\x94\x01\x92\xF1\x92=`$=\x11a6\x7FW[\x80` \x91\x84R\x80_\x83\x86\x01>\x83\x01\x01`@RV[P`\x01\x92P`$a6kV[\x80Q\x90` \x01\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x95\x92\x90\x94\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x95\x16\x90\x81\x81\x10_\x14a8\x16WP\x92[_\x93\x85\x81\x10a7\\W[PPPa7K\x92Pa&\x89V[\x81\x81\x11\x15a7WWP\x90V[\x90P\x90V[\x90\x91\x92\x93P\x84\x03\x93\x84\x11a\x11SW\x80\x15\x80\x15\x90\x81a7\xB1W[P\x93a7\x99\x92\x91a7K\x95_\x14a7\xAAWPP\x80[\x80\x82\x11a7\xA2W[P\x84aF\xE3V[\x90_\x80\x80a7>V[\x90P_a7\x92V[\x02\x90a7\x8AV[\x90P\x91\x90\x91a7\xE9W\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x90\x04\x84\x11a7\x99a7uV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90P\x92a74V[\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[\x90`\x0C\x11a\x01\xD3W`\x08\x015`\xE0\x1C\x90V[\x90`L\x11a\x01\xD3W`,\x015\x90V[\x90`,\x11a\x01\xD3W`\x0C\x015\x90V[\x91\x90\x91\x82`L\x11a\x01\xD3W`L\x01\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xB4\x01\x90V[Q\x90a\x02\x85\x82a\x196V[Q\x90a\x02\x85\x82a\x06hV[\x81`\x1F\x82\x01\x12\x15a\x01\xD3W\x80Q\x90a8\xC0\x82a\x1B\xCFV[\x92a8\xCE`@Q\x94\x85a\x1BQV[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xD3W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[a8\xF7a)\xB0V[P\x80Q\x81\x01\x90` \x81\x83\x03\x12a\x01\xD3W` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xD3W\x01\x90a\x02\0\x82\x82\x03\x12a\x01\xD3Wa91a\x1B\x92V[\x91a9>` \x82\x01a8\x93V[\x83Ra9L`@\x82\x01a\x1F\xDCV[` \x84\x01Ra9]``\x82\x01a\x1F\xDCV[`@\x84\x01R`\x80\x81\x01Q``\x84\x01R`\xA0\x81\x01Q`\x80\x84\x01R`\xC0\x81\x01Q`\xA0\x84\x01R`\xE0\x81\x01Q`\xC0\x84\x01Ra\x01\0\x81\x01Q`\xE0\x84\x01Ra\x01 \x81\x01Qa\x01\0\x84\x01Ra9\xAEa\x01@\x82\x01a8\x9EV[a\x01 \x84\x01Ra9\xC1a\x01`\x82\x01a8\x9EV[a\x01@\x84\x01Ra9\xD4a\x01\x80\x82\x01a8\x9EV[a\x01`\x84\x01Ra\x01\xA0\x81\x01Qa\x01\x80\x84\x01Ra\x01\xC0\x81\x01Qa\x01\xA0\x84\x01Ra9\xFFa\x01\xE0\x82\x01a8\x9EV[a\x01\xC0\x84\x01Ra\x02\0\x81\x01Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xD3Wa,~\x92` \x80\x92\x01\x92\x01\x01a8\xA9V[\x90a\x01\xE0\x82\x01a;\xC2a;\xB8a&\xD3a\x01\xC0a:ra:]a:Xa:R\x88QQa&mV[`\x05\x1C\x90V[a&{V[\x90`@Q\x82\x81\x93\x82R`\x01\x01`\x05\x1B\x01`@RV[` \x80\x82\x01R\x96a:\x99a:\x90a:\x8A\x83Q`\xFF\x16\x90V[`\xFF\x16\x90V[`@\x8A\x01R\x88\x90V[Pa:\xBAa:\xB1a\"\xF5` \x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[``\x8A\x01R\x88\x90V[Pa:\xDBa:\xD2a\"\xF5`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[`\x80\x8A\x01R\x88\x90V[P``\x81\x01Q`\xA0\x89\x01R`\x80\x81\x01Q`\xC0\x89\x01R`\xA0\x81\x01Q`\xE0\x89\x01R`\xC0\x81\x01Qa\x01\0\x89\x01R`\xE0\x81\x01Qa\x01 \x89\x01Ra\x01\0\x81\x01Qa\x01@\x89\x01Ra;Ba;8a&\xD3a\x01 \x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01`\x8A\x01R\x88\x90V[Pa;ia;_a&\xD3a\x01@\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\x80\x8A\x01R\x88\x90V[Pa;\x90a;\x86a&\xD3a\x01`\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\xA0\x8A\x01R\x88\x90V[Pa\x01\x80\x81\x01Qa\x01\xC0\x89\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x89\x01R\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\0\x85\x01R\x83\x90V[Pa\x02\0a\x02 \x84\x01R\x80QQa\x02@\x84\x01RQ\x80Q\x80a<\x07W[PP\x81Q`\x05\x1B` \x83\x01 a\x02\x85\x90\x92\x80Q`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x87\x01\x94\x01\x01\x90[\x81\x81\x10a<\x87WPP`\x1F\x16\x90\x81\x15a;\xDEW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a;\xDEV[\x80Q\x84R` \x93\x84\x01\x93\x01a<\x18V[\x91a<\xA9\x82_R_` R`@_ \x90V[\x80T\x90`\x02`\xA0\x83\x90\x1C`\xFF\x16a<\xBF\x81a\x07\xEFV[\x14a=\xDEW\x91a=C\x91\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x93a<\xF4\x87a>\x0BV[\x91a=\x1Fa=\x05`\x80\x8A\x01Qa?\x1FV[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\0\x89\x01Q\x80\x86\x10\x15a=\xD8WP\x84\x90[a=;\x82\x87a\x1F\xBAV[\x96\x87\x94a\"\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x99\x86\x8B\x15a=\xA5WPP\x90a=p\x91\x85\x8Aa?kV[\x81a=\x93W[PPP[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01a'\xAEV[a=\x9D\x92\x87a?kV[_\x82\x81a=vV[\x92P\x92\x90Pa=\xD3\x94\x93Pa=\xCCa\x01\xC0a\x01\xE0\x85\x01Q\x94\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x93\x89a&\x96V[a=zV[\x90a=1V[\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x84\x90R`$_\xFD[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x80\x15a\x16\xCDW` a>\xD0\x91`\xC0\x93_\x91a\x1E\xEBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x03a>\xF4WP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80`\xA0\x1Ca?@Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x83\x15a@\xA8W`@Q\x92_\x80` \x86\x01\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x96\x87`$\x82\x01R\x88`D\x82\x01R`D\x81Ra?\xD2`d\x82a\x1BQV[Q\x90\x82\x86Z\xF1a?\xE0a&>V[\x81a@pW[Pa@iW\x81a@N\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x93a(\x1Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[a@Y\x87\x82Ta&\x89V[\x90U`@Q\x95\x86R\x16\x93` \x90\xA4V[PPPPPV[\x80Q\x80\x15\x92P\x82\x15a@\x85W[PP_a?\xE6V[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x01\xD3W` \x01Qa@\xA1\x81a\x01\xC9V[_\x80a@}V[PPPPV[\x90_\x91`$\x81Q\x14a@\xBDWPV[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x14aA\x13WPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90`\xA0\x81`$\x81\x85Z\xFA\x90\x81\x15a\x16\xCDW_\x91aB\xF8W[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x92\x83\x90`D\x90\x82\x90Z\xFA\x91\x82\x15a\x16\xCDW_\x92aB\xCFW[PQ\x15[\x90\x81\x15aB\x92W[\x81\x15aBoW[PaB>WPV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90P_aB6V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaB\xC7\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15\x90aB/V[aB'\x91\x92PaB\xF0a\x15\x88\x91`@=`@\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[\x92\x91PaB#V[aC\x11\x91P`\xA0=`\xA0\x11a\x16\xC6Wa\x16\xB8\x81\x83a\x1BQV[_aA\xAEV[\x99\x97\x95\x93\x91\x98\x96\x94\x92\x90\x98`@Q\x99` \x8B\x01\x9B\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x8DR`\xFF\x16`@\x8C\x01Rc\xFF\xFF\xFF\xFF\x16``\x8B\x01R`\x80\x8A\x01R`\xA0\x89\x01R`\xC0\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x82\x01Ra\x01\x80\x81RaC\xB8a\x01\xA0\x82a\x1BQV[Q\x90 \x90V[\x95\x93\x90aDh\x93aDIa\x1A\xE4\x99\x97\x94`@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94aD\x11\x8C\x82Q` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[` \x81\x81\x01Q\x8D\x84\x01R\x91\x01Q``\x8C\x01R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8C\x01R\x01Q`\xA0\x8A\x01RV[\x16`\xC0\x87\x01R`\xE0\x86\x01Ra\x01@a\x01\0\x86\x01Ra\x01@\x85\x01\x90a\x11XV[\x92a\x01 \x81\x85\x03\x91\x01Ra\x1FOV[\x95\x94\x91\x98\x93\x90\x98\x97\x92\x96\x97aD\xAE`@Q\x97aD\x94`@\x8Aa\x1BQV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88RV[` \x87\x01RaD\xBBa\x1B\xC0V[\x95\x86R` \x86\x01R`@\x85\x01RaD\xD0a\x1B\xB1V[0\x81R\x94` \x86\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x01\xD3W_\x95aE)\x93`@Q\x98\x89\x97\x88\x97\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\x04\x89\x01aC\xBEV[\x03\x81\x83n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3Z\xF1\x80\x15a\x16\xCDWaEKWPV[\x80a\x1A/_a\x02\x85\x93a\x1BQV[\x90\x81` \x91\x03\x12a\x01\xD3WQa\x1A\xE4\x81a\x06\x14V[\x91\x90\x82`@\x91\x03\x12a\x01\xD3W`@QaE\x86\x81a\x1B5V[` \x80\x82\x94\x80Q\x84R\x01Q\x91\x01RV[\x91\x90\x82\x81\x03`\xC0\x81\x12a\x01\xD3W`\x80\x13a\x01\xD3Wa\x1A\xE4\x90`\x80`@QaE\xBC\x81a\x1B\x14V[\x85Q\x81R` \x86\x01QaE\xCE\x81a\x06hV[` \x82\x01RaE\xE0\x83`@\x88\x01aEnV[`@\x82\x01R\x94\x01aEnV[``\x90aF\xC7aF\xB2a\x02\x85\x95\x97\x96\x94`\x80\x84Rc\xFF\xFF\xFF\xFF\x81Q\x16`\x80\x85\x01R` \x81\x01Q`\xA0\x85\x01R`@\x81\x01Q`\xC0\x85\x01R\x84\x81\x01Q`\xE0\x85\x01R`\xC0aF\x7FaFJ`\x80\x84\x01Q`\xE0a\x01\0\x89\x01Ra\x01`\x88\x01\x90a\x11XV[`\xA0\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x83\x03\x01a\x01 \x89\x01Ra\x11XV[\x91\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x83\x03\x01a\x01@\x86\x01Ra\x11XV[\x96` \x83\x01\x90` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x81\x81\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x81\x83\x85\x04\x14\x83\x15\x17\x02\x15aG\x0EWPPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFg\r\xE0\xB6\xB3\xA7d\0\0\x92\x84\t\x84\x81\x10\x85\x01\x90\x03\x92\t\x90\x80g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15aG\x89W\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x90V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x91\x90`\x14R\x80`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x86Z\xF1\x80`\x01_Q\x14\x16\x15aG\xD3W[PPP_`4RV[=\x83;\x15\x17\x10\x15aG\xE5W[\x80aG\xCAV[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x86Z\xF1P`4R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16aG\xDFW;\x15=\x17\x10\x15aH2W_\x80aG\xDFV[c>?\x8Fs_R`\x04`\x1C\xFD[a\x1A\xE4a\x01\xE0\x91aIW`@Q\x93\x84\x92` \x80\x85\x01RaHe`@\x85\x01\x82Q`\xFF\x16\x90RV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16``\x85\x01R`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\x80\x85\x01R``\x81\x01Q`\xA0\x85\x01R`\x80\x81\x01Q`\xC0\x85\x01R`\xA0\x81\x01Q`\xE0\x85\x01R`\xC0\x81\x01Qa\x01\0\x85\x01R`\xE0\x81\x01Qa\x01 \x85\x01Ra\x01\0\x81\x01Qa\x01@\x85\x01RaH\xE2a\x01 \x82\x01Qa\x01`\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a\x01@\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x85\x01Ra\x01`\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01RaIAa\x01\xC0\x82\x01Qa\x02\0\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a\x11XV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x1BQV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806316c38b3c146101c45780631e83409a146101bf57806325692962146101ba57806331eee44d146101b557806339c33215146101b05780634a6be468146101ab57806354d1f13d146101a65780635778472a146101a15780635975e7901461019c5780635c975abb14610197578063617c537c146101925780636afdd8501461018d578063715018a6146101885780637376f1c014610183578063776ff3c71461017e57806379502c551461017957806385c17830146101745780638cda96de1461016f5780638da5cb5b1461016a57806397c36bae14610165578063ac9650d814610160578063d0a102601461015b578063d4570c1c14610156578063f04e283e14610151578063f2fde38b1461014c578063f3995c67146101475763fee81cf414610142575f80fd5b611a35565b61194c565b6118d9565b611868565b6117cf565b6112d9565b61119b565b6110a4565b611034565b610fbb565b610f80565b610f12565b610c97565b610c3c565b610b9f565b610b58565b6109fb565b6109b8565b61091b565b61086c565b61078d565b6106ae565b6105d9565b610475565b6103ff565b610287565b6101d7565b801515036101d357565b5f80fd5b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435610212816101c9565b61021a61206d565b7fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff68ff000000000000000060035492151560401b169116176003555f80f35b73ffffffffffffffffffffffffffffffffffffffff8116036101d357565b610124359061028582610259565b565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576004356102c281610259565b688000000000ab143c065c6103e8576001688000000000ab143c065d335f5260016020526103118160405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5480156103c0576103bc91335f5260016020525f61035082604083209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b5561035c8233836120a4565b73ffffffffffffffffffffffffffffffffffffffff6040519183835216907ff7a40077ff7a04c7e61f6f26fb13774259ddf1b6bce9ecf26a8276cdd399268360203392a35f688000000000ab143c065d6040519081529081906020820190565b0390f35b7f969bf728000000000000000000000000000000000000000000000000000000005f5260045ffd5b63ab143c065f526004601cfd5b5f9103126101d357565b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35763389a75e1600c52335f526202a30042016020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f80a2005b90816102009103126101d35790565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d3576104c4903690600401610466565b688000000000ab143c065c6103e8576001688000000000ab143c065d60ff60035460401c166105b1576101c06105686105617fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d6105926103bc9561052833826122cc565b98939992918a98829661053d843033886124f3565b61054b6101e0820182611a89565b96909101359561055a87610668565b3691611c09565b9289612696565b6040805191825260208201959095524264ffffffffff169481019490945233939081906060820190565b0390a35f688000000000ab143c065d6040519081529081906020820190565b7f9e87fac8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576020604051624c4b408152f35b63ffffffff8116036101d357565b6004359061028582610614565b359061028582610614565b9181601f840112156101d35782359167ffffffffffffffff83116101d357602083818601950101116101d357565b67ffffffffffffffff8116036101d357565b60a4359061028582610668565b359061028582610668565b908160409103126101d35790565b908160609103126101d35790565b6101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576106e1610622565b6024359060443560643560843567ffffffffffffffff81116101d35761070b90369060040161063a565b61071361067a565b60c4359160e435936101043567ffffffffffffffff81116101d35761073c903690600401610692565b95610745610277565b97610144359a67ffffffffffffffff8c116101d35761076b6107719c36906004016106a0565b9a611c3f565b6040805192835267ffffffffffffffff91909116602083015290f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35763389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f80a2005b600311156107f957565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b919091604064ffffffffff81606084019573ffffffffffffffffffffffffffffffffffffffff8151168552602081015161085f816107ef565b6020860152015116910152565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576004355f604080516108ac81611b14565b82815282602082015201525f525f6020526103bc60405f2064ffffffffff604051916108d783611b14565b5473ffffffffffffffffffffffffffffffffffffffff8116835260ff8160a01c16610901816107ef565b602084015260a81c16604082015260405191829182610826565b6101207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561095281610614565b6024359060443560643560843567ffffffffffffffff81116101d35761097c90369060040161063a565b61098461067a565b9160c4359360e43595610104359867ffffffffffffffff8a116101d3576109b26107719a3690600401610692565b98611da5565b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602060ff60035460401c166040519015158152f35b346101d35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435610a3681610259565b60243590610a4382610259565b6044356064359260843567ffffffffffffffff81116101d357610a6a90369060040161063a565b909460a43593303303610b3057610b2196610b1c9382610a8e610af094888b6120a4565b73ffffffffffffffffffffffffffffffffffffffff604051998a967f3866c1dc000000000000000000000000000000000000000000000000000000006020890152602488015216604486015260648501526080608485015260a4840191611f4f565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101855284611b51565b61364d565b91905015610b2b57005b61368b565b7f14d4a4e8000000000000000000000000000000000000000000000000000000005f5260045ffd5b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760206040516e22d473030f116ddee9f6b43ac78ba38152f35b5f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357610bd061206d565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a35f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755005b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101d35760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d357610ce6903690600401610466565b602435610cf281610259565b60443567ffffffffffffffff81116101d357610d129036906004016106a0565b688000000000ab143c065c6103e8576001688000000000ab143c065d60ff60035460401c166105b157610d4582846122cc565b959291849791949585610100840135823592602081013560405160208101907fb333a9ae2c4c3677d1efa59a8cdee570700c1b20baf81ce2406192e5155d165682528c604082015260408152610d9c606082611b51565b5190209060405192610daf60a085611b51565b606a8452602084017f46696c6c417574686f72697a6174696f6e207769746e6573732946696c6c41759052604084017f74686f72697a6174696f6e2862797465733332206f72646572496429546f6b659052606084017f6e5065726d697373696f6e73286164647265737320746f6b656e2c75696e74329052608084017f353620616d6f756e742900000000000000000000000000000000000000000000905260408101610e5c91611a89565b95610e6a9791958c8c614477565b84610e796101e0840184611a89565b936101c001610e8790611ada565b933690610e9392611c09565b91610e9e9488612696565b60408051928352602083019490945264ffffffffff42169382019390935273ffffffffffffffffffffffffffffffffffffffff909216917fb67a0b8b144469e404c22c77c4e86b9745a9bd928a4e86a79933e7b16966b78d90606090a35f688000000000ab143c065d604051908152602090f35b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d3576020600254604051908152f35b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435610ff561206d565b670de0b6b3a7640000811161100957600255005b7fad6bb6d1000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b346101d3575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760207fffffffffffffffffffffffffffffffffffffffffffffffffffffffff748739275473ffffffffffffffffffffffffffffffffffffffff60405191168152f35b346101d35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d3576110f3903690600401610466565b611137602435610100830135926101408101359161111083610668565b6101608201359061112082610668565b610180830135916101a06002549401359487613714565b9081810390811161115357604080519182526020820192909252f35b611f8d565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f602080948051918291828752018686015e5f8582860101520116010190565b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043567ffffffffffffffff81116101d357366023820112156101d357806004013567ffffffffffffffff81116101d3578060051b36602482850101116101d357346101d3576040519283926020845280846020015260408481019380602485018637850101928391611244575b505050806040520360401b1761381e565b919350915b80915f6044825187016024810135918291018537389084305af4156112d057602067ffffffffffffffe0603f5f937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08a87030181528301943d81523d858583013e3d010116933d01015284838210156112c3575090611249565b93505090505f8080611233565b853d5f823e3d90fd5b60a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561130f81610259565b60443567ffffffffffffffff81116101d35761132f90369060040161063a565b61133a606435610259565b60843567ffffffffffffffff81116101d35761135a90369060040161063a565b5050688000000000ab143c065c6103e8576001688000000000ab143c065d60ff60035460401c166105b157604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016949181604481885afa9081156116cd5773ffffffffffffffffffffffffffffffffffffffff9161145e915f916117a0575b505173ffffffffffffffffffffffffffffffffffffffff1690565b16801561176e576040517f5e280f11000000000000000000000000000000000000000000000000000000008152602081600481855afa80156116cd5773ffffffffffffffffffffffffffffffffffffffff915f9161173f575b501633036117135773ffffffffffffffffffffffffffffffffffffffff8216036116d257506114e6818361382f565b61156860a061151461150f61055a6114fe8789613841565b96611509818a613850565b9861385f565b6138ef565b95611526602088015163ffffffff1690565b9060405180809581947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa9081156116cd575f9161169e575b5061158c6115888251151590565b1590565b908115611686575b5061165557506115b8305b73ffffffffffffffffffffffffffffffffffffffff1690565b810361162a5750604082015163ffffffff1691468390036115f5576115e792506115e181613a2c565b90613c97565b5f688000000000ab143c065d005b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5263ffffffff831660045260245ffd5b5ffd5b7f25ea23d9000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fe73d23c3000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6040015163ffffffff8381169116141590505f611594565b6116c0915060a03d60a0116116c6575b6116b88183611b51565b810190611fe7565b5f61157a565b503d6116ae565b611f44565b7f89233a72000000000000000000000000000000000000000000000000000000005f5273ffffffffffffffffffffffffffffffffffffffff1660045260245ffd5b7f39e7e94b000000000000000000000000000000000000000000000000000000005f523360045260245ffd5b611761915060203d602011611767575b6117598183611b51565b810190611fc7565b5f6114b7565b503d61174f565b7fb825dd76000000000000000000000000000000000000000000000000000000005f524663ffffffff1660045260245ffd5b6117c2915060403d6040116117c8575b6117ba8183611b51565b810190611f0a565b5f611443565b503d6117b0565b346101d35760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357602061185f60043561180f81610259565b73ffffffffffffffffffffffffffffffffffffffff6024359161183183610259565b165f526001835260405f209073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b54604051908152f35b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561189e81610259565b6118a661206d565b63389a75e1600c52805f526020600c2090815442116118cc575f6118ca9255613693565b005b636f5e88185f526004601cfd5b60207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561190f81610259565b61191761206d565b8060601b15611929576118ca90613693565b637448fbae5f526004601cfd5b60ff8116036101d357565b359061028582611936565b346101d35760c07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d35760043561198781610259565b6024356044356064359261199a84611936565b73ffffffffffffffffffffffffffffffffffffffff169160a43590608435843b156101d3575f9460e493869260ff604051998a9889977fd505accf0000000000000000000000000000000000000000000000000000000089523360048a01523060248a01526044890152606488015216608486015260a485015260c48401525af1611a2157005b80611a2f5f6118ca93611b51565b806103f5565b346101d35760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101d357600435611a7081610259565b63389a75e1600c525f52602080600c2054604051908152f35b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1813603018212156101d3570180359067ffffffffffffffff82116101d3576020019181360383136101d357565b35611ae481610668565b90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6060810190811067ffffffffffffffff821117611b3057604052565b611ae7565b6040810190811067ffffffffffffffff821117611b3057604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117611b3057604052565b6040519061028561020083611b51565b6040519061028560e083611b51565b60405190610285604083611b51565b60405190610285606083611b51565b67ffffffffffffffff8111611b3057601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b929192611c1582611bcf565b91611c236040519384611b51565b8294818452818301116101d3578281602093845f960137010152565b9397929b9a96919b99959094989960ff60035460401c166105b157611c62612959565b9c611c6f968e9689612a21565b94803590611c7c82610668565b6101c087019167ffffffffffffffff16825260208101611c9b91611a89565b6101e088019291611cad913691611c09565b8252611cb887612cd0565b611cc3368787611c09565b8051602090910120875160ff16916040890151611ce39063ffffffff1690565b9160808a0151948a60e0810196875161010083015190610160840151611d109067ffffffffffffffff1690565b61014085015167ffffffffffffffff16611d2991612ecd565b92610180850151946101a001519551805190602001209751611d529067ffffffffffffffff1690565b98611d5c9a614317565b9260a0870151611d6b90613f1f565b9151936020820135823586611d7e612eef565b9460408101611d8c91611a89565b97611d98999197614477565b611da192613085565b9190565b9099989496939592919760ff60035460401c166105b157611e039461055a94611dda948b611dd1612959565b9e8f9633612a21565b92611df9611de782611ada565b67ffffffffffffffff166101c0860152565b6020810190611a89565b6101e0820152611e1281612cd0565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa9485156116cd57611edd6020611da197611ee6945f91611eeb575b50015173ffffffffffffffffffffffffffffffffffffffff1690565b309033906124f3565b613085565b611f04915060403d6040116117c8576117ba8183611b51565b5f611ec1565b908160409103126101d357602060405191611f2483611b35565b8051611f2f81610259565b83520151611f3c81610259565b602082015290565b6040513d5f823e3d90fd5b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093818652868601375f8582860101520116010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9190820391821161115357565b908160209103126101d35751611ae481610259565b519061028582610614565b908160a09103126101d3576040519060a082019082821067ffffffffffffffff831117611b3057608091604052805161201f816101c9565b8352602081015161202f81610614565b6020840152604081015161204281610614565b6040840152606081015161205581610259565b6060840152015161206581610259565b608082015290565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392754330361209757565b6382b429005f526004601cfd5b91906014526034526fa9059cbb0000000000000000000000005f5260205f6044601082855af1908160015f511416156120e0575b50505f603452565b3b153d1710156120f1575f806120d8565b6390b8ec185f526004601cfd5b35611ae481611936565b35611ae481610614565b9080601f830112156101d357816020611ae493359101611c09565b919091610200818403126101d357612143611b92565b9261214d82611941565b845261215b6020830161062f565b602085015261216c6040830161062f565b6040850152606082013560608501526080820135608085015260a082013560a085015260c082013560c085015260e082013560e08501526101008201356101008501526121bc6101208301610687565b6101208501526121cf6101408301610687565b6101408501526121e26101608301610687565b6101608501526101808201356101808501526101a08201356101a085015261220d6101c08301610687565b6101c08501526101e082013567ffffffffffffffff81116101d3576122329201612112565b6101e0830152565b61224460016107ef565b740100000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b61228d60026107ef565b740200000000000000000000000000000000000000007fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff825416179055565b9190916122d8816120fe565b60ff806001169116036124b457604081016122fe6122f582612108565b63ffffffff1690565b46036124775750612317612312368361212d565b613a2c565b92612329845f525f60205260405f2090565b91612339835460ff9060a01c1690565b612342816107ef565b61244a57612358612353368361212d565b613e0b565b936102856123696080840135613f1f565b946123f86123b66123af610100870135966123876101408201611ada565b906123956101608201611ada565b91610180820135906101a06002549301359342918c613714565b8096611fba565b95829073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b6124018161223a565b80547fffffffffffff0000000000ffffffffffffffffffffffffffffffffffffffffff164260a81b79ffffffffff00000000000000000000000000000000000000000016179055565b7f343e211e000000000000000000000000000000000000000000000000000000005f52600485905260245ffd5b61248361162791612108565b7f8dae2d2b000000000000000000000000000000000000000000000000000000005f5263ffffffff16600452602490565b6124c0611627916120fe565b7fb2c3b6fd000000000000000000000000000000000000000000000000000000005f52600160045260ff16602452604490565b916040519360605260405260601b602c526f23b872dd000000000000000000000000600c5260205f6064601c82855af1908160015f5114161561253c575b50505f606052604052565b3b153d17101561254d575f80612531565b637939f4245f526004601cfd5b67ffffffffffffffff61c3509116019067ffffffffffffffff821161115357565b9067ffffffffffffffff8091169116019067ffffffffffffffff821161115357565b94909367ffffffffffffffff9373ffffffffffffffffffffffffffffffffffffffff6125ee948160a0989b9a9b1689521660208801526040870152606086015260c0608086015260c0850190611158565b9416910152565b905f6020830192612605826107ef565b52565b9060026020830192612605826107ef565b9060016020830192612605826107ef565b60405190612639602083611b51565b5f8252565b3d15612668573d9061264f82611bcf565b9161265d6040519384611b51565b82523d5f602084013e565b606090565b90601f820180921161115357565b601201908160121161115357565b9190820180921161115357565b93949091948315612951578051158015612948575b61293b575a6126e06126d36126ce600586901c6707ffffffffffffff168661257b565b61255a565b67ffffffffffffffff1690565b116128fe57303b156101d35761272a915f9160405193849283927f617c537c00000000000000000000000000000000000000000000000000000000845289898c8a6004880161259d565b038183305af190816128ea575b506128a25761274c61274761263e565b6140ae565b9073ffffffffffffffffffffffffffffffffffffffff82169485151580612898575b156127b3575090612780929184613f6b565b7f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9604051806127ae81612619565b0390a3565b73ffffffffffffffffffffffffffffffffffffffff80965082935081612840879461281d7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9573ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b9073ffffffffffffffffffffffffffffffffffffffff165f5260205260405f2090565b61284b878254612689565b9055169586936128676040519283921696829190602083019252565b0390a47f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a9604051806127ae81612608565b503086141561276e565b50507f9e3d9d3bbfdd46df14557c427c78a0ca7e2950aa7154abf713f5b7099eba12a973ffffffffffffffffffffffffffffffffffffffff604051931692806127ae816125f5565b80611a2f5f6128f893611b51565b5f612737565b611627825a7f588700c7000000000000000000000000000000000000000000000000000000005f5260045267ffffffffffffffff16602452604490565b5050610285939192613f6b565b50853b156126ab565b505050505050565b6003549067ffffffffffffffff8216916001830167ffffffffffffffff81116111535767ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000009116911617600355565b6129b8611b92565b905f82525f60208301525f60408301525f60608301525f60808301525f60a08301525f60c08301525f60e08301525f6101008301525f6101208301525f6101408301525f6101608301525f6101808301525f6101a08301525f6101c083015260606101e0830152565b94939197929097612a306129b0565b50604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff8116602483015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169892909190816044818c5afa9182156116cd57612b006020612b4c946040945f91612cb95750015173ffffffffffffffffffffffffffffffffffffffff1690565b998d83518095819482937f320005c50000000000000000000000000000000000000000000000000000000084526004840190929160ff60209163ffffffff604085019616845216910152565b03915afa9081156116cd57612b80916020915f91611eeb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b9073ffffffffffffffffffffffffffffffffffffffff88161561176e5773ffffffffffffffffffffffffffffffffffffffff821615612c875773ffffffffffffffffffffffffffffffffffffffff9081169767ffffffffffffffff42169792821693911691612bef908861257b565b97612bf8611b92565b600181529b63ffffffff461660208e015263ffffffff1660408d015260608c015260808b015260a08a015260c089015260e088015261010087015267ffffffffffffffff1661012086015267ffffffffffffffff1661014085015267ffffffffffffffff166101608401526101808301526101a08201525f6101c0820152612c7e61262a565b6101e082015290565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff8b1660045260245ffd5b611f049150853d87116117c8576117ba8183611b51565b602081015163ffffffff16468103612e9c5750612cfc612cf7604083015163ffffffff1690565b61412d565b61016081015161014082015167ffffffffffffffff918216911680821115612e615750506101008101805180158015612e54575b612e1e57506101a082015190519081811015612df057505073ffffffffffffffffffffffffffffffffffffffff612d6a6080830151613f1f565b1615612dc8576101c0015167ffffffffffffffff16624c4b408111612d8c5750565b7f25ad8594000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff16600452624c4b4060245260445ffd5b7fd27b4443000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f8d00b91b000000000000000000000000000000000000000000000000000000005f5260045260245260445ffd5b60e08301517f8dd09d91000000000000000000000000000000000000000000000000000000005f5260049190915260245260445ffd5b5060e08301518111612d30565b7f2802dd9e000000000000000000000000000000000000000000000000000000005f5267ffffffffffffffff9081166004521660245260445ffd5b7f1b2f5167000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b9067ffffffffffffffff8091169116039067ffffffffffffffff821161115357565b60405190612eff61016083611b51565b61012982527f3620616d6f756e74290000000000000000000000000000000000000000000000610140837f4f72646572496e74656e74207769746e657373294f72646572496e74656e742860208201527f75696e743820627269646765547970652c75696e74333220647374436861696e60408201527f49642c6279746573333220726563697069656e742c75696e7432353620696e7060608201527f7574416d6f756e742c75696e74323536206f7574707574416d6f756e742c756960808201527f6e7436342064656c697665727957696e646f772c75696e74323536206469736360a08201527f6f756e74526174652c75696e7432353620626173654665652c6279746573333260c08201527f20627269646765506172616d732c6279746573333220686f6f6b44617461486160e08201527f73682c75696e7436342063616c6c6261636b4761734c696d697429546f6b656e6101008201527f5065726d697373696f6e73286164647265737320746f6b656e2c75696e7432356101208201520152565b9092919261309282613a2c565b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff166024820152919573ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016939092909181604481875afa9081156116cd575f9161362e575b5061316061159f61159f835173ffffffffffffffffffffffffffffffffffffffff1690565b604051917ffc0c546a000000000000000000000000000000000000000000000000000000008352602083600481855afa9283156116cd575f93613608575b506020015173ffffffffffffffffffffffffffffffffffffffff165b73ffffffffffffffffffffffffffffffffffffffff811673ffffffffffffffffffffffffffffffffffffffff8416036135c157506040517f0a70b05600000000000000000000000000000000000000000000000000000000815246600482015260a081602481895afa9081156116cd57613244916040915f9161351d575b50015163ffffffff1690565b604051907f5e280f11000000000000000000000000000000000000000000000000000000008252602082600481865afa9182156116cd5773ffffffffffffffffffffffffffffffffffffffff926020915f916135a4575b506004604051809581937f416ecebf000000000000000000000000000000000000000000000000000000008352165afa9182156116cd575f92613573575b5063ffffffff811663ffffffff83160361353c5750509061335d929161330660e088019282845191614796565b60a0604088019661331b885163ffffffff1690565b9060405180809881947f0a70b0560000000000000000000000000000000000000000000000000000000083526004830191909163ffffffff6020820193169052565b03915afa9283156116cd576133cc613389604060c09661342a985f9161351d5750015163ffffffff1690565b9351916101008a0197885161339d8c61483f565b946133b56133a9611ba2565b63ffffffff9099168952565b306020890152604088015260608701523691611c09565b608084015260a08301526133de61262a565b838301526133ea611bb1565b3481525f60208201526040518095819482937fc7c7f5b30000000000000000000000000000000000000000000000000000000084523391600485016145ec565b039134905af19283156116cd576127ae7f2955a0c7a1fbafcea94e0ad6d2c4844a1f179f29634410b75de3a5a2977fea1a9373ffffffffffffffffffffffffffffffffffffffff9389966134ef575b5061349561348a6060890151613f1f565b935163ffffffff1690565b966134b361012060c0830151935192015167ffffffffffffffff1690565b604080516001815263ffffffff909a1660208b0152890192909252606088015267ffffffffffffffff16608087015291169390819060a0820190565b6135109060c03d60c011613516575b6135088183611b51565b810190614596565b50613479565b503d6134fe565b613536915060a03d60a0116116c6576116b88183611b51565b5f613238565b7fb9190bb3000000000000000000000000000000000000000000000000000000005f5263ffffffff9081166004521660245260445ffd5b61359691925060203d60201161359d575b61358e8183611b51565b810190614559565b905f6132d9565b503d613584565b6135bb9150823d8411611767576117598183611b51565b5f61329b565b7ff902523f000000000000000000000000000000000000000000000000000000005f5273ffffffffffffffffffffffffffffffffffffffff8084166004521660245260445ffd5b6131ba919350613626602091823d8411611767576117598183611b51565b93915061319e565b613647915060403d6040116117c8576117ba8183611b51565b5f61313b565b5f9081936040519383389360208451940192f1923d60243d1161367f575b806020918452805f8386013e830101604052565b5060019250602461366b565b805190602001fd5b73ffffffffffffffffffffffffffffffffffffffff16807fffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927547f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a37fffffffffffffffffffffffffffffffffffffffffffffffffffffffff7487392755565b959290949167ffffffffffffffff8091169516908181105f146138165750925b5f9385811061375c575b50505061374b9250612689565b81811115613757575090565b905090565b90919293508403938411611153578015801590816137b1575b5093613799929161374b955f146137aa575050805b8082116137a2575b50846146e3565b905f808061373e565b90505f613792565b029061378a565b90509190916137e957907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8190048411613799613775565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b905092613734565b8060401c9067ffffffffffffffff16f35b90600c116101d3576008013560e01c90565b90604c116101d357602c013590565b90602c116101d357600c013590565b91909182604c116101d357604c01917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb40190565b519061028582611936565b519061028582610668565b81601f820112156101d3578051906138c082611bcf565b926138ce6040519485611b51565b828452602083830101116101d357815f9260208093018386015e8301015290565b6138f76129b0565b5080518101906020818303126101d35760208101519067ffffffffffffffff82116101d3570190610200828203126101d357613931611b92565b9161393e60208201613893565b835261394c60408201611fdc565b602084015261395d60608201611fdc565b60408401526080810151606084015260a0810151608084015260c081015160a084015260e081015160c084015261010081015160e08401526101208101516101008401526139ae610140820161389e565b6101208401526139c1610160820161389e565b6101408401526139d4610180820161389e565b6101608401526101a08101516101808401526101c08101516101a08401526139ff6101e0820161389e565b6101c08401526102008101519167ffffffffffffffff83116101d357612c7e9260208092019201016138a9565b906101e08201613bc2613bb86126d36101c0613a72613a5d613a58613a5288515161266d565b60051c90565b61267b565b90604051828193825260010160051b01604052565b60208082015296613a99613a90613a8a835160ff1690565b60ff1690565b60408a01528890565b50613aba613ab16122f5602084015163ffffffff1690565b60608a01528890565b50613adb613ad26122f5604084015163ffffffff1690565b60808a01528890565b50606081015160a0890152608081015160c089015260a081015160e089015260c081015161010089015260e0810151610120890152610100810151610140890152613b42613b386126d361012084015167ffffffffffffffff1690565b6101608a01528890565b50613b69613b5f6126d361014084015167ffffffffffffffff1690565b6101808a01528890565b50613b90613b866126d361016084015167ffffffffffffffff1690565b6101a08a01528890565b506101808101516101c08901526101a08101516101e0890152015167ffffffffffffffff1690565b6102008501528390565b5061020061022084015280515161024084015251805180613c07575b5050815160051b602083012061028590928051604051828260010160051b011490151060061b52565b602082016020826102608701940101905b818110613c87575050601f16908115613bde5760017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92019260200360031b1b011981511690525f80613bde565b8051845260209384019301613c18565b91613ca9825f525f60205260405f2090565b805490600260a083901c60ff16613cbf816107ef565b14613dde5791613d43917f26ebbca293ad62a56cd6aba32cbd10c11c3ced6cd738dccba811d8edd7991a9a93613cf487613e0b565b91613d1f613d0560808a0151613f1f565b9173ffffffffffffffffffffffffffffffffffffffff1690565b61010089015180861015613dd8575084905b613d3b8287611fba565b968794612283565b73ffffffffffffffffffffffffffffffffffffffff811699868b15613da557505090613d7091858a613f6b565b81613d93575b5050505b60408051918252602082019290925290819081016127ae565b613d9d9287613f6b565b5f8281613d76565b9250929050613dd3949350613dcc6101c06101e085015194015167ffffffffffffffff1690565b9389612696565b613d7a565b90613d31565b7fb196a44a000000000000000000000000000000000000000000000000000000005f52600484905260245ffd5b604080517f320005c50000000000000000000000000000000000000000000000000000000081524660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015290816044817f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff165afa80156116cd576020613ed09160c0935f91611eeb5750015173ffffffffffffffffffffffffffffffffffffffff1690565b91015173ffffffffffffffffffffffffffffffffffffffff82168103613ef4575090565b7f2e775c7c000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b8060a01c613f405773ffffffffffffffffffffffffffffffffffffffff1690565b7f2bf95065000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b9083156140a857604051925f80602086017fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84169687602482015288604482015260448152613fd2606482611b51565b519082865af1613fe061263e565b81614070575b50614069578161404e7f9428757c1737778a519319e79c210670c49393bfbefb8410a0adb9670dcb659f9361281d73ffffffffffffffffffffffffffffffffffffffff9473ffffffffffffffffffffffffffffffffffffffff165f52600160205260405f2090565b614059878254612689565b90556040519586521693602090a4565b5050505050565b8051801592508215614085575b50505f613fe6565b81925090602091810103126101d357602001516140a1816101c9565b5f8061407d565b50505050565b905f9160248151146140bd5750565b7ff7c3b366000000000000000000000000000000000000000000000000000000007fffffffff00000000000000000000000000000000000000000000000000000000602460208401519301519216146141135750565b73ffffffffffffffffffffffffffffffffffffffff169150565b6040517f0a70b05600000000000000000000000000000000000000000000000000000000815263ffffffff8216600482015273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169060a081602481855afa9081156116cd575f916142f8575b50604080517f320005c500000000000000000000000000000000000000000000000000000000815263ffffffff851660048201527f000000000000000000000000000000000000000000000000000000000000000060ff16602482015292839060449082905afa9182156116cd575f926142cf575b5051155b908115614292575b811561426f575b5061423e5750565b7fb825dd76000000000000000000000000000000000000000000000000000000005f5263ffffffff1660045260245ffd5b6020015173ffffffffffffffffffffffffffffffffffffffff161590505f614236565b905073ffffffffffffffffffffffffffffffffffffffff6142c7825173ffffffffffffffffffffffffffffffffffffffff1690565b16159061422f565b6142279192506142f06115889160403d6040116117c8576117ba8183611b51565b929150614223565b614311915060a03d60a0116116c6576116b88183611b51565b5f6141ae565b99979593919896949290986040519960208b019b7f937e713d48c0ce14a0ca67eed9a5a7296eb40cda72ecbc56d28804c2976fc36b8d5260ff1660408c015263ffffffff1660608b015260808a015260a089015260c088015267ffffffffffffffff1660e087015261010086015261012085015261014084015261016083015267ffffffffffffffff1661018082015261018081526143b86101a082611b51565b51902090565b95939061446893614449611ae4999794604073ffffffffffffffffffffffffffffffffffffffff946144118c82516020809173ffffffffffffffffffffffffffffffffffffffff81511684520151910152565b6020818101518d84015291015160608c0152815173ffffffffffffffffffffffffffffffffffffffff1660808c0152015160a08a0152565b1660c087015260e0860152610140610100860152610140850190611158565b92610120818503910152611f4f565b95949198939098979296976144ae6040519761449460408a611b51565b73ffffffffffffffffffffffffffffffffffffffff168852565b60208701526144bb611bc0565b958652602086015260408501526144d0611bb1565b3081529460208601526e22d473030f116ddee9f6b43ac78ba33b156101d3575f956145299360405198899788977f137c29fe000000000000000000000000000000000000000000000000000000008952600489016143be565b0381836e22d473030f116ddee9f6b43ac78ba35af180156116cd5761454b5750565b80611a2f5f61028593611b51565b908160209103126101d35751611ae481610614565b91908260409103126101d35760405161458681611b35565b6020808294805184520151910152565b919082810360c081126101d3576080136101d357611ae49060806040516145bc81611b14565b8551815260208601516145ce81610668565b60208201526145e0836040880161456e565b6040820152940161456e565b6060906146c76146b2610285959796946080845263ffffffff8151166080850152602081015160a0850152604081015160c08501528481015160e085015260c061467f61464a608084015160e0610100890152610160880190611158565b60a08401517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8088830301610120890152611158565b9101517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8085830301610140860152611158565b96602083019060208091805184520151910152565b019073ffffffffffffffffffffffffffffffffffffffff169052565b81810291670de0b6b3a76400008183850414831517021561470e575050670de0b6b3a7640000900490565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff670de0b6b3a76400009284098481108501900392099080670de0b6b3a7640000111561478957828211900360ee1b910360121c177faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac106690290565b63ae47f7025f526004601cfd5b9190601452806034526f095ea7b30000000000000000000000005f5260205f6044601082865af18060015f511416156147d3575b5050505f603452565b3d833b151710156147e5575b806147ca565b5f6034526f095ea7b30000000000000000000000005f525f386044601083865af15060345260205f6044601082855af1908160015f5114166147df573b153d171015614832575f806147df565b633e3f8f735f526004601cfd5b611ae46101e09161495760405193849260208085015261486560408501825160ff169052565b602081015163ffffffff166060850152604081015163ffffffff166080850152606081015160a0850152608081015160c085015260a081015160e085015260c081015161010085015260e08101516101208501526101008101516101408501526148e261012082015161016086019067ffffffffffffffff169052565b61014081015167ffffffffffffffff1661018085015261016081015167ffffffffffffffff166101a08501526101808101516101c08501526101a0810151828501526149416101c082015161020086019067ffffffffffffffff169052565b0151610200610220840152610240830190611158565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282611b5156fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x16\xC3\x8B<\x14a\x01\xC4W\x80c\x1E\x83@\x9A\x14a\x01\xBFW\x80c%i)b\x14a\x01\xBAW\x80c1\xEE\xE4M\x14a\x01\xB5W\x80c9\xC32\x15\x14a\x01\xB0W\x80cJk\xE4h\x14a\x01\xABW\x80cT\xD1\xF1=\x14a\x01\xA6W\x80cWxG*\x14a\x01\xA1W\x80cYu\xE7\x90\x14a\x01\x9CW\x80c\\\x97Z\xBB\x14a\x01\x97W\x80ca|S|\x14a\x01\x92W\x80cj\xFD\xD8P\x14a\x01\x8DW\x80cqP\x18\xA6\x14a\x01\x88W\x80csv\xF1\xC0\x14a\x01\x83W\x80cwo\xF3\xC7\x14a\x01~W\x80cyP,U\x14a\x01yW\x80c\x85\xC1x0\x14a\x01tW\x80c\x8C\xDA\x96\xDE\x14a\x01oW\x80c\x8D\xA5\xCB[\x14a\x01jW\x80c\x97\xC3k\xAE\x14a\x01eW\x80c\xAC\x96P\xD8\x14a\x01`W\x80c\xD0\xA1\x02`\x14a\x01[W\x80c\xD4W\x0C\x1C\x14a\x01VW\x80c\xF0N(>\x14a\x01QW\x80c\xF2\xFD\xE3\x8B\x14a\x01LW\x80c\xF3\x99\\g\x14a\x01GWc\xFE\xE8\x1C\xF4\x14a\x01BW_\x80\xFD[a\x1A5V[a\x19LV[a\x18\xD9V[a\x18hV[a\x17\xCFV[a\x12\xD9V[a\x11\x9BV[a\x10\xA4V[a\x104V[a\x0F\xBBV[a\x0F\x80V[a\x0F\x12V[a\x0C\x97V[a\x0C<V[a\x0B\x9FV[a\x0BXV[a\t\xFBV[a\t\xB8V[a\t\x1BV[a\x08lV[a\x07\x8DV[a\x06\xAEV[a\x05\xD9V[a\x04uV[a\x03\xFFV[a\x02\x87V[a\x01\xD7V[\x80\x15\x15\x03a\x01\xD3WV[_\x80\xFD[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x02\x12\x81a\x01\xC9V[a\x02\x1Aa mV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\xFF\0\0\0\0\0\0\0\0`\x03T\x92\x15\x15`@\x1B\x16\x91\x16\x17`\x03U_\x80\xF3[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xD3WV[a\x01$5\x90a\x02\x85\x82a\x02YV[V[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x02\xC2\x81a\x02YV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]3_R`\x01` Ra\x03\x11\x81`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T\x80\x15a\x03\xC0Wa\x03\xBC\x913_R`\x01` R_a\x03P\x82`@\x83 \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[Ua\x03\\\x823\x83a \xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x83\x83R\x16\x90\x7F\xF7\xA4\0w\xFFz\x04\xC7\xE6\x1Fo&\xFB\x13wBY\xDD\xF1\xB6\xBC\xE9\xEC\xF2j\x82v\xCD\xD3\x99&\x83` 3\x92\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x7F\x96\x9B\xF7(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[c\xAB\x14<\x06_R`\x04`\x1C\xFD[_\x91\x03\x12a\x01\xD3WV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wc8\x9Au\xE1`\x0CR3_Rb\x02\xA3\0B\x01` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D_\x80\xA2\0[\x90\x81a\x02\0\x91\x03\x12a\x01\xD3W\x90V[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x04\xC4\x906\x90`\x04\x01a\x04fV[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\x01\xC0a\x05ha\x05a\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8Da\x05\x92a\x03\xBC\x95a\x05(3\x82a\"\xCCV[\x98\x93\x99\x92\x91\x8A\x98\x82\x96a\x05=\x8403\x88a$\xF3V[a\x05Ka\x01\xE0\x82\x01\x82a\x1A\x89V[\x96\x90\x91\x015\x95a\x05Z\x87a\x06hV[6\x91a\x1C\tV[\x92\x89a&\x96V[`@\x80Q\x91\x82R` \x82\x01\x95\x90\x95RBd\xFF\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R3\x93\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x7F\x9E\x87\xFA\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@QbLK@\x81R\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xD3WV[`\x045\x90a\x02\x85\x82a\x06\x14V[5\x90a\x02\x85\x82a\x06\x14V[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xD3W\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xD3W` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xD3WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xD3WV[`\xA45\x90a\x02\x85\x82a\x06hV[5\x90a\x02\x85\x82a\x06hV[\x90\x81`@\x91\x03\x12a\x01\xD3W\x90V[\x90\x81``\x91\x03\x12a\x01\xD3W\x90V[a\x01`\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wa\x06\xE1a\x06\"V[`$5\x90`D5`d5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x07\x0B\x906\x90`\x04\x01a\x06:V[a\x07\x13a\x06zV[`\xC45\x91`\xE45\x93a\x01\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x07<\x906\x90`\x04\x01a\x06\x92V[\x95a\x07Ea\x02wV[\x97a\x01D5\x9Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x11a\x01\xD3Wa\x07ka\x07q\x9C6\x90`\x04\x01a\x06\xA0V[\x9Aa\x1C?V[`@\x80Q\x92\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R\x90\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wc8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92_\x80\xA2\0[`\x03\x11\x15a\x07\xF9WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x91`@d\xFF\xFF\xFF\xFF\xFF\x81``\x84\x01\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x85R` \x81\x01Qa\x08_\x81a\x07\xEFV[` \x86\x01R\x01Q\x16\x91\x01RV[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045_`@\x80Qa\x08\xAC\x81a\x1B\x14V[\x82\x81R\x82` \x82\x01R\x01R_R_` Ra\x03\xBC`@_ d\xFF\xFF\xFF\xFF\xFF`@Q\x91a\x08\xD7\x83a\x1B\x14V[Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83R`\xFF\x81`\xA0\x1C\x16a\t\x01\x81a\x07\xEFV[` \x84\x01R`\xA8\x1C\x16`@\x82\x01R`@Q\x91\x82\x91\x82a\x08&V[a\x01 \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\tR\x81a\x06\x14V[`$5\x90`D5`d5`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\t|\x906\x90`\x04\x01a\x06:V[a\t\x84a\x06zV[\x91`\xC45\x93`\xE45\x95a\x01\x045\x98g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x11a\x01\xD3Wa\t\xB2a\x07q\x9A6\x90`\x04\x01a\x06\x92V[\x98a\x1D\xA5V[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `\xFF`\x03T`@\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01\xD3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\n6\x81a\x02YV[`$5\x90a\nC\x82a\x02YV[`D5`d5\x92`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\nj\x906\x90`\x04\x01a\x06:V[\x90\x94`\xA45\x9303\x03a\x0B0Wa\x0B!\x96a\x0B\x1C\x93\x82a\n\x8Ea\n\xF0\x94\x88\x8Ba \xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x99\x8A\x96\x7F8f\xC1\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x89\x01R`$\x88\x01R\x16`D\x86\x01R`d\x85\x01R`\x80`\x84\x85\x01R`\xA4\x84\x01\x91a\x1FOV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x85R\x84a\x1BQV[a6MV[\x91\x90P\x15a\x0B+W\0[a6\x8BV[\x7F\x14\xD4\xA4\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@Qn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x81R\xF3[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3Wa\x0B\xD0a mV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'U\0[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xD3W``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x0C\xE6\x906\x90`\x04\x01a\x04fV[`$5a\x0C\xF2\x81a\x02YV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\r\x12\x906\x90`\x04\x01a\x06\xA0V[h\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\rE\x82\x84a\"\xCCV[\x95\x92\x91\x84\x97\x91\x94\x95\x85a\x01\0\x84\x015\x825\x92` \x81\x015`@Q` \x81\x01\x90\x7F\xB33\xA9\xAE,L6w\xD1\xEF\xA5\x9A\x8C\xDE\xE5pp\x0C\x1B \xBA\xF8\x1C\xE2@a\x92\xE5\x15]\x16V\x82R\x8C`@\x82\x01R`@\x81Ra\r\x9C``\x82a\x1BQV[Q\x90 \x90`@Q\x92a\r\xAF`\xA0\x85a\x1BQV[`j\x84R` \x84\x01\x7FFillAuthorization witness)FillAu\x90R`@\x84\x01\x7Fthorization(bytes32 orderId)Toke\x90R``\x84\x01\x7FnPermissions(address token,uint2\x90R`\x80\x84\x01\x7F56 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90R`@\x81\x01a\x0E\\\x91a\x1A\x89V[\x95a\x0Ej\x97\x91\x95\x8C\x8CaDwV[\x84a\x0Eya\x01\xE0\x84\x01\x84a\x1A\x89V[\x93a\x01\xC0\x01a\x0E\x87\x90a\x1A\xDAV[\x936\x90a\x0E\x93\x92a\x1C\tV[\x91a\x0E\x9E\x94\x88a&\x96V[`@\x80Q\x92\x83R` \x83\x01\x94\x90\x94Rd\xFF\xFF\xFF\xFF\xFFB\x16\x93\x82\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x7F\xB6z\x0B\x8B\x14Di\xE4\x04\xC2,w\xC4\xE8k\x97E\xA9\xBD\x92\x8AN\x86\xA7\x993\xE7\xB1if\xB7\x8D\x90``\x90\xA3_h\x80\0\0\0\0\xAB\x14<\x06]`@Q\x90\x81R` \x90\xF3[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` `\x02T`@Q\x90\x81R\xF3[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x0F\xF5a mV[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11a\x10\tW`\x02U\0[\x7F\xADk\xB6\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[4a\x01\xD3W_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x01\xD3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x10\xF3\x906\x90`\x04\x01a\x04fV[a\x117`$5a\x01\0\x83\x015\x92a\x01@\x81\x015\x91a\x11\x10\x83a\x06hV[a\x01`\x82\x015\x90a\x11 \x82a\x06hV[a\x01\x80\x83\x015\x91a\x01\xA0`\x02T\x94\x015\x94\x87a7\x14V[\x90\x81\x81\x03\x90\x81\x11a\x11SW`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[a\x1F\x8DV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3W6`#\x82\x01\x12\x15a\x01\xD3W\x80`\x04\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3W\x80`\x05\x1B6`$\x82\x85\x01\x01\x11a\x01\xD3W4a\x01\xD3W`@Q\x92\x83\x92` \x84R\x80\x84` \x01R`@\x84\x81\x01\x93\x80`$\x85\x01\x867\x85\x01\x01\x92\x83\x91a\x12DW[PPP\x80`@R\x03`@\x1B\x17a8\x1EV[\x91\x93P\x91[\x80\x91_`D\x82Q\x87\x01`$\x81\x015\x91\x82\x91\x01\x8578\x90\x840Z\xF4\x15a\x12\xD0W` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?_\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x8A\x87\x03\x01\x81R\x83\x01\x94=\x81R=\x85\x85\x83\x01>=\x01\x01\x16\x93=\x01\x01R\x84\x83\x82\x10\x15a\x12\xC3WP\x90a\x12IV[\x93PP\x90P_\x80\x80a\x123V[\x85=_\x82>=\x90\xFD[`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x13\x0F\x81a\x02YV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x13/\x906\x90`\x04\x01a\x06:V[a\x13:`d5a\x02YV[`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\x13Z\x906\x90`\x04\x01a\x06:V[PPh\x80\0\0\0\0\xAB\x14<\x06\\a\x03\xE8W`\x01h\x80\0\0\0\0\xAB\x14<\x06]`\xFF`\x03T`@\x1C\x16a\x05\xB1W`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x94\x91\x81`D\x81\x88Z\xFA\x90\x81\x15a\x16\xCDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x14^\x91_\x91a\x17\xA0W[PQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x80\x15a\x17nW`@Q\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x85Z\xFA\x80\x15a\x16\xCDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91_\x91a\x17?W[P\x163\x03a\x17\x13Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x03a\x16\xD2WPa\x14\xE6\x81\x83a8/V[a\x15h`\xA0a\x15\x14a\x15\x0Fa\x05Za\x14\xFE\x87\x89a8AV[\x96a\x15\t\x81\x8Aa8PV[\x98a8_V[a8\xEFV[\x95a\x15&` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x95\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\x16\xCDW_\x91a\x16\x9EW[Pa\x15\x8Ca\x15\x88\x82Q\x15\x15\x90V[\x15\x90V[\x90\x81\x15a\x16\x86W[Pa\x16UWPa\x15\xB80[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x81\x03a\x16*WP`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x91F\x83\x90\x03a\x15\xF5Wa\x15\xE7\x92Pa\x15\xE1\x81a:,V[\x90a<\x97V[_h\x80\0\0\0\0\xAB\x14<\x06]\0[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x83\x16`\x04R`$_\xFD[_\xFD[\x7F%\xEA#\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xE7=#\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[`@\x01Qc\xFF\xFF\xFF\xFF\x83\x81\x16\x91\x16\x14\x15\x90P_a\x15\x94V[a\x16\xC0\x91P`\xA0=`\xA0\x11a\x16\xC6W[a\x16\xB8\x81\x83a\x1BQV[\x81\x01\x90a\x1F\xE7V[_a\x15zV[P=a\x16\xAEV[a\x1FDV[\x7F\x89#:r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[\x7F9\xE7\xE9K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[a\x17a\x91P` =` \x11a\x17gW[a\x17Y\x81\x83a\x1BQV[\x81\x01\x90a\x1F\xC7V[_a\x14\xB7V[P=a\x17OV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_RFc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[a\x17\xC2\x91P`@=`@\x11a\x17\xC8W[a\x17\xBA\x81\x83a\x1BQV[\x81\x01\x90a\x1F\nV[_a\x14CV[P=a\x17\xB0V[4a\x01\xD3W`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W` a\x18_`\x045a\x18\x0F\x81a\x02YV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x91a\x181\x83a\x02YV[\x16_R`\x01\x83R`@_ \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x18\x9E\x81a\x02YV[a\x18\xA6a mV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x90\x81TB\x11a\x18\xCCW_a\x18\xCA\x92Ua6\x93V[\0[co^\x88\x18_R`\x04`\x1C\xFD[` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x19\x0F\x81a\x02YV[a\x19\x17a mV[\x80``\x1B\x15a\x19)Wa\x18\xCA\x90a6\x93V[ctH\xFB\xAE_R`\x04`\x1C\xFD[`\xFF\x81\x16\x03a\x01\xD3WV[5\x90a\x02\x85\x82a\x196V[4a\x01\xD3W`\xC0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x19\x87\x81a\x02YV[`$5`D5`d5\x92a\x19\x9A\x84a\x196V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91`\xA45\x90`\x845\x84;\x15a\x01\xD3W_\x94`\xE4\x93\x86\x92`\xFF`@Q\x99\x8A\x98\x89\x97\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R3`\x04\x8A\x01R0`$\x8A\x01R`D\x89\x01R`d\x88\x01R\x16`\x84\x86\x01R`\xA4\x85\x01R`\xC4\x84\x01RZ\xF1a\x1A!W\0[\x80a\x1A/_a\x18\xCA\x93a\x1BQV[\x80a\x03\xF5V[4a\x01\xD3W` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC6\x01\x12a\x01\xD3W`\x045a\x1Ap\x81a\x02YV[c8\x9Au\xE1`\x0CR_R` \x80`\x0C T`@Q\x90\x81R\xF3[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x01\xD3W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xD3W` \x01\x91\x816\x03\x83\x13a\x01\xD3WV[5a\x1A\xE4\x81a\x06hV[\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1B0W`@RV[a\x1A\xE7V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1B0W`@RV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x1B0W`@RV[`@Q\x90a\x02\x85a\x02\0\x83a\x1BQV[`@Q\x90a\x02\x85`\xE0\x83a\x1BQV[`@Q\x90a\x02\x85`@\x83a\x1BQV[`@Q\x90a\x02\x85``\x83a\x1BQV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B0W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[\x92\x91\x92a\x1C\x15\x82a\x1B\xCFV[\x91a\x1C#`@Q\x93\x84a\x1BQV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xD3W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x93\x97\x92\x9B\x9A\x96\x91\x9B\x99\x95\x90\x94\x98\x99`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\x1Cba)YV[\x9Ca\x1Co\x96\x8E\x96\x89a*!V[\x94\x805\x90a\x1C|\x82a\x06hV[a\x01\xC0\x87\x01\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x81\x01a\x1C\x9B\x91a\x1A\x89V[a\x01\xE0\x88\x01\x92\x91a\x1C\xAD\x916\x91a\x1C\tV[\x82Ra\x1C\xB8\x87a,\xD0V[a\x1C\xC36\x87\x87a\x1C\tV[\x80Q` \x90\x91\x01 \x87Q`\xFF\x16\x91`@\x89\x01Qa\x1C\xE3\x90c\xFF\xFF\xFF\xFF\x16\x90V[\x91`\x80\x8A\x01Q\x94\x8A`\xE0\x81\x01\x96\x87Qa\x01\0\x83\x01Q\x90a\x01`\x84\x01Qa\x1D\x10\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01@\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D)\x91a.\xCDV[\x92a\x01\x80\x85\x01Q\x94a\x01\xA0\x01Q\x95Q\x80Q\x90` \x01 \x97Qa\x1DR\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x98a\x1D\\\x9AaC\x17V[\x92`\xA0\x87\x01Qa\x1Dk\x90a?\x1FV[\x91Q\x93` \x82\x015\x825\x86a\x1D~a.\xEFV[\x94`@\x81\x01a\x1D\x8C\x91a\x1A\x89V[\x97a\x1D\x98\x99\x91\x97aDwV[a\x1D\xA1\x92a0\x85V[\x91\x90V[\x90\x99\x98\x94\x96\x93\x95\x92\x91\x97`\xFF`\x03T`@\x1C\x16a\x05\xB1Wa\x1E\x03\x94a\x05Z\x94a\x1D\xDA\x94\x8Ba\x1D\xD1a)YV[\x9E\x8F\x963a*!V[\x92a\x1D\xF9a\x1D\xE7\x82a\x1A\xDAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xC0\x86\x01RV[` \x81\x01\x90a\x1A\x89V[a\x01\xE0\x82\x01Ra\x1E\x12\x81a,\xD0V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x94\x85\x15a\x16\xCDWa\x1E\xDD` a\x1D\xA1\x97a\x1E\xE6\x94_\x91a\x1E\xEBW[P\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[0\x903\x90a$\xF3V[a0\x85V[a\x1F\x04\x91P`@=`@\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[_a\x1E\xC1V[\x90\x81`@\x91\x03\x12a\x01\xD3W` `@Q\x91a\x1F$\x83a\x1B5V[\x80Qa\x1F/\x81a\x02YV[\x83R\x01Qa\x1F<\x81a\x02YV[` \x82\x01R\x90V[`@Q=_\x82>=\x90\xFD[`\x1F\x82` \x94\x93\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x91\x90\x82\x03\x91\x82\x11a\x11SWV[\x90\x81` \x91\x03\x12a\x01\xD3WQa\x1A\xE4\x81a\x02YV[Q\x90a\x02\x85\x82a\x06\x14V[\x90\x81`\xA0\x91\x03\x12a\x01\xD3W`@Q\x90`\xA0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x1B0W`\x80\x91`@R\x80Qa \x1F\x81a\x01\xC9V[\x83R` \x81\x01Qa /\x81a\x06\x14V[` \x84\x01R`@\x81\x01Qa B\x81a\x06\x14V[`@\x84\x01R``\x81\x01Qa U\x81a\x02YV[``\x84\x01R\x01Qa e\x81a\x02YV[`\x80\x82\x01R\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T3\x03a \x97WV[c\x82\xB4)\0_R`\x04`\x1C\xFD[\x91\x90`\x14R`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a \xE0W[PP_`4RV[;\x15=\x17\x10\x15a \xF1W_\x80a \xD8V[c\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[5a\x1A\xE4\x81a\x196V[5a\x1A\xE4\x81a\x06\x14V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xD3W\x81` a\x1A\xE4\x935\x91\x01a\x1C\tV[\x91\x90\x91a\x02\0\x81\x84\x03\x12a\x01\xD3Wa!Ca\x1B\x92V[\x92a!M\x82a\x19AV[\x84Ra![` \x83\x01a\x06/V[` \x85\x01Ra!l`@\x83\x01a\x06/V[`@\x85\x01R``\x82\x015``\x85\x01R`\x80\x82\x015`\x80\x85\x01R`\xA0\x82\x015`\xA0\x85\x01R`\xC0\x82\x015`\xC0\x85\x01R`\xE0\x82\x015`\xE0\x85\x01Ra\x01\0\x82\x015a\x01\0\x85\x01Ra!\xBCa\x01 \x83\x01a\x06\x87V[a\x01 \x85\x01Ra!\xCFa\x01@\x83\x01a\x06\x87V[a\x01@\x85\x01Ra!\xE2a\x01`\x83\x01a\x06\x87V[a\x01`\x85\x01Ra\x01\x80\x82\x015a\x01\x80\x85\x01Ra\x01\xA0\x82\x015a\x01\xA0\x85\x01Ra\"\ra\x01\xC0\x83\x01a\x06\x87V[a\x01\xC0\x85\x01Ra\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xD3Wa\"2\x92\x01a!\x12V[a\x01\xE0\x83\x01RV[a\"D`\x01a\x07\xEFV[t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[a\"\x8D`\x02a\x07\xEFV[t\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82T\x16\x17\x90UV[\x91\x90\x91a\"\xD8\x81a \xFEV[`\xFF\x80`\x01\x16\x91\x16\x03a$\xB4W`@\x81\x01a\"\xFEa\"\xF5\x82a!\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[F\x03a$wWPa#\x17a#\x126\x83a!-V[a:,V[\x92a#)\x84_R_` R`@_ \x90V[\x91a#9\x83T`\xFF\x90`\xA0\x1C\x16\x90V[a#B\x81a\x07\xEFV[a$JWa#Xa#S6\x83a!-V[a>\x0BV[\x93a\x02\x85a#i`\x80\x84\x015a?\x1FV[\x94a#\xF8a#\xB6a#\xAFa\x01\0\x87\x015\x96a#\x87a\x01@\x82\x01a\x1A\xDAV[\x90a#\x95a\x01`\x82\x01a\x1A\xDAV[\x91a\x01\x80\x82\x015\x90a\x01\xA0`\x02T\x93\x015\x93B\x91\x8Ca7\x14V[\x80\x96a\x1F\xBAV[\x95\x82\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90UV[a$\x01\x81a\":V[\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B`\xA8\x1By\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x90UV[\x7F4>!\x1E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x85\x90R`$_\xFD[a$\x83a\x16'\x91a!\x08V[\x7F\x8D\xAE-+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$\x90V[a$\xC0a\x16'\x91a \xFEV[\x7F\xB2\xC3\xB6\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`\xFF\x16`$R`D\x90V[\x91`@Q\x93``R`@R``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16\x15a%<W[PP_``R`@RV[;\x15=\x17\x10\x15a%MW_\x80a%1V[cy9\xF4$_R`\x04`\x1C\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\xC3P\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11SWV[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11SWV[\x94\x90\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa%\xEE\x94\x81`\xA0\x98\x9B\x9A\x9B\x16\x89R\x16` \x88\x01R`@\x87\x01R``\x86\x01R`\xC0`\x80\x86\x01R`\xC0\x85\x01\x90a\x11XV[\x94\x16\x91\x01RV[\x90_` \x83\x01\x92a&\x05\x82a\x07\xEFV[RV[\x90`\x02` \x83\x01\x92a&\x05\x82a\x07\xEFV[\x90`\x01` \x83\x01\x92a&\x05\x82a\x07\xEFV[`@Q\x90a&9` \x83a\x1BQV[_\x82RV[=\x15a&hW=\x90a&O\x82a\x1B\xCFV[\x91a&]`@Q\x93\x84a\x1BQV[\x82R=_` \x84\x01>V[``\x90V[\x90`\x1F\x82\x01\x80\x92\x11a\x11SWV[`\x12\x01\x90\x81`\x12\x11a\x11SWV[\x91\x90\x82\x01\x80\x92\x11a\x11SWV[\x93\x94\x90\x91\x94\x83\x15a)QW\x80Q\x15\x80\x15a)HW[a);WZa&\xE0a&\xD3a&\xCE`\x05\x86\x90\x1Cg\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a%{V[a%ZV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x11a(\xFEW0;\x15a\x01\xD3Wa'*\x91_\x91`@Q\x93\x84\x92\x83\x92\x7Fa|S|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R\x89\x89\x8C\x8A`\x04\x88\x01a%\x9DV[\x03\x81\x830Z\xF1\x90\x81a(\xEAW[Pa(\xA2Wa'La'Ga&>V[a@\xAEV[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x94\x85\x15\x15\x80a(\x98W[\x15a'\xB3WP\x90a'\x80\x92\x91\x84a?kV[\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a'\xAE\x81a&\x19V[\x03\x90\xA3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x96P\x82\x93P\x81a(@\x87\x94a(\x1D\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R` R`@_ \x90V[a(K\x87\x82Ta&\x89V[\x90U\x16\x95\x86\x93a(g`@Q\x92\x83\x92\x16\x96\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA4\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9`@Q\x80a'\xAE\x81a&\x08V[P0\x86\x14\x15a'nV[PP\x7F\x9E=\x9D;\xBF\xDDF\xDF\x14U|B|x\xA0\xCA~)P\xAAqT\xAB\xF7\x13\xF5\xB7\t\x9E\xBA\x12\xA9s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x93\x16\x92\x80a'\xAE\x81a%\xF5V[\x80a\x1A/_a(\xF8\x93a\x1BQV[_a'7V[a\x16'\x82Z\x7FX\x87\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`$R`D\x90V[PPa\x02\x85\x93\x91\x92a?kV[P\x85;\x15a&\xABV[PPPPPPV[`\x03T\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`\x01\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x11SWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x16\x91\x16\x17`\x03UV[a)\xB8a\x1B\x92V[\x90_\x82R_` \x83\x01R_`@\x83\x01R_``\x83\x01R_`\x80\x83\x01R_`\xA0\x83\x01R_`\xC0\x83\x01R_`\xE0\x83\x01R_a\x01\0\x83\x01R_a\x01 \x83\x01R_a\x01@\x83\x01R_a\x01`\x83\x01R_a\x01\x80\x83\x01R_a\x01\xA0\x83\x01R_a\x01\xC0\x83\x01R``a\x01\xE0\x83\x01RV[\x94\x93\x91\x97\x92\x90\x97a*0a)\xB0V[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x81\x16`$\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x98\x92\x90\x91\x90\x81`D\x81\x8CZ\xFA\x91\x82\x15a\x16\xCDWa+\0` a+L\x94`@\x94_\x91a,\xB9WP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x99\x8D\x83Q\x80\x95\x81\x94\x82\x93\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01\x90\x92\x91`\xFF` \x91c\xFF\xFF\xFF\xFF`@\x85\x01\x96\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x16\xCDWa+\x80\x91` \x91_\x91a\x1E\xEBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x15a\x17nWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15a,\x87Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x97g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x97\x92\x82\x16\x93\x91\x16\x91a+\xEF\x90\x88a%{V[\x97a+\xF8a\x1B\x92V[`\x01\x81R\x9Bc\xFF\xFF\xFF\xFFF\x16` \x8E\x01Rc\xFF\xFF\xFF\xFF\x16`@\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\xA0\x8A\x01R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x86\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x84\x01Ra\x01\x80\x83\x01Ra\x01\xA0\x82\x01R_a\x01\xC0\x82\x01Ra,~a&*V[a\x01\xE0\x82\x01R\x90V[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04R`$_\xFD[a\x1F\x04\x91P\x85=\x87\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16F\x81\x03a.\x9CWPa,\xFCa,\xF7`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[aA-V[a\x01`\x81\x01Qa\x01@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x80\x82\x11\x15a.aWPPa\x01\0\x81\x01\x80Q\x80\x15\x80\x15a.TW[a.\x1EWPa\x01\xA0\x82\x01Q\x90Q\x90\x81\x81\x10\x15a-\xF0WPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa-j`\x80\x83\x01Qa?\x1FV[\x16\x15a-\xC8Wa\x01\xC0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16bLK@\x81\x11a-\x8CWPV[\x7F%\xAD\x85\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x04RbLK@`$R`D_\xFD[\x7F\xD2{DC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F\x8D\0\xB9\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[`\xE0\x83\x01Q\x7F\x8D\xD0\x9D\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x91\x90\x91R`$R`D_\xFD[P`\xE0\x83\x01Q\x81\x11a-0V[\x7F(\x02\xDD\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[\x7F\x1B/Qg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x03\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x11SWV[`@Q\x90a.\xFFa\x01`\x83a\x1BQV[a\x01)\x82R\x7F6 amount)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01@\x83\x7FOrderIntent witness)OrderIntent(` \x82\x01R\x7Fuint8 bridgeType,uint32 dstChain`@\x82\x01R\x7FId,bytes32 recipient,uint256 inp``\x82\x01R\x7FutAmount,uint256 outputAmount,ui`\x80\x82\x01R\x7Fnt64 deliveryWindow,uint256 disc`\xA0\x82\x01R\x7FountRate,uint256 baseFee,bytes32`\xC0\x82\x01R\x7F bridgeParams,bytes32 hookDataHa`\xE0\x82\x01R\x7Fsh,uint64 callbackGasLimit)Tokena\x01\0\x82\x01R\x7FPermissions(address token,uint25a\x01 \x82\x01R\x01RV[\x90\x92\x91\x92a0\x92\x82a:,V[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x91\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x90\x92\x90\x91\x81`D\x81\x87Z\xFA\x90\x81\x15a\x16\xCDW_\x91a6.W[Pa1`a\x15\x9Fa\x15\x9F\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x91\x7F\xFC\x0CTj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R` \x83`\x04\x81\x85Z\xFA\x92\x83\x15a\x16\xCDW_\x93a6\x08W[P` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x03a5\xC1WP`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R`\xA0\x81`$\x81\x89Z\xFA\x90\x81\x15a\x16\xCDWa2D\x91`@\x91_\x91a5\x1DW[P\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x90\x7F^(\x0F\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R` \x82`\x04\x81\x86Z\xFA\x91\x82\x15a\x16\xCDWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92` \x91_\x91a5\xA4W[P`\x04`@Q\x80\x95\x81\x93\x7FAn\xCE\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x16Z\xFA\x91\x82\x15a\x16\xCDW_\x92a5sW[Pc\xFF\xFF\xFF\xFF\x81\x16c\xFF\xFF\xFF\xFF\x83\x16\x03a5<WPP\x90a3]\x92\x91a3\x06`\xE0\x88\x01\x92\x82\x84Q\x91aG\x96V[`\xA0`@\x88\x01\x96a3\x1B\x88Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x80\x80\x98\x81\x94\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x04\x83\x01\x91\x90\x91c\xFF\xFF\xFF\xFF` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x92\x83\x15a\x16\xCDWa3\xCCa3\x89`@`\xC0\x96a4*\x98_\x91a5\x1DWP\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x93Q\x91a\x01\0\x8A\x01\x97\x88Qa3\x9D\x8CaH?V[\x94a3\xB5a3\xA9a\x1B\xA2V[c\xFF\xFF\xFF\xFF\x90\x99\x16\x89RV[0` \x89\x01R`@\x88\x01R``\x87\x01R6\x91a\x1C\tV[`\x80\x84\x01R`\xA0\x83\x01Ra3\xDEa&*V[\x83\x83\x01Ra3\xEAa\x1B\xB1V[4\x81R_` \x82\x01R`@Q\x80\x95\x81\x94\x82\x93\x7F\xC7\xC7\xF5\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R3\x91`\x04\x85\x01aE\xECV[\x03\x914\x90Z\xF1\x92\x83\x15a\x16\xCDWa'\xAE\x7F)U\xA0\xC7\xA1\xFB\xAF\xCE\xA9N\n\xD6\xD2\xC4\x84J\x1F\x17\x9F)cD\x10\xB7]\xE3\xA5\xA2\x97\x7F\xEA\x1A\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x89\x96a4\xEFW[Pa4\x95a4\x8A``\x89\x01Qa?\x1FV[\x93Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96a4\xB3a\x01 `\xC0\x83\x01Q\x93Q\x92\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@\x80Q`\x01\x81Rc\xFF\xFF\xFF\xFF\x90\x9A\x16` \x8B\x01R\x89\x01\x92\x90\x92R``\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01R\x91\x16\x93\x90\x81\x90`\xA0\x82\x01\x90V[a5\x10\x90`\xC0=`\xC0\x11a5\x16W[a5\x08\x81\x83a\x1BQV[\x81\x01\x90aE\x96V[Pa4yV[P=a4\xFEV[a56\x91P`\xA0=`\xA0\x11a\x16\xC6Wa\x16\xB8\x81\x83a\x1BQV[_a28V[\x7F\xB9\x19\x0B\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x90\x81\x16`\x04R\x16`$R`D_\xFD[a5\x96\x91\x92P` =` \x11a5\x9DW[a5\x8E\x81\x83a\x1BQV[\x81\x01\x90aEYV[\x90_a2\xD9V[P=a5\x84V[a5\xBB\x91P\x82=\x84\x11a\x17gWa\x17Y\x81\x83a\x1BQV[_a2\x9BV[\x7F\xF9\x02R?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16`\x04R\x16`$R`D_\xFD[a1\xBA\x91\x93Pa6&` \x91\x82=\x84\x11a\x17gWa\x17Y\x81\x83a\x1BQV[\x93\x91Pa1\x9EV[a6G\x91P`@=`@\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[_a1;V[_\x90\x81\x93`@Q\x93\x838\x93` \x84Q\x94\x01\x92\xF1\x92=`$=\x11a6\x7FW[\x80` \x91\x84R\x80_\x83\x86\x01>\x83\x01\x01`@RV[P`\x01\x92P`$a6kV[\x80Q\x90` \x01\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'T\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFt\x879'UV[\x95\x92\x90\x94\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x91\x16\x95\x16\x90\x81\x81\x10_\x14a8\x16WP\x92[_\x93\x85\x81\x10a7\\W[PPPa7K\x92Pa&\x89V[\x81\x81\x11\x15a7WWP\x90V[\x90P\x90V[\x90\x91\x92\x93P\x84\x03\x93\x84\x11a\x11SW\x80\x15\x80\x15\x90\x81a7\xB1W[P\x93a7\x99\x92\x91a7K\x95_\x14a7\xAAWPP\x80[\x80\x82\x11a7\xA2W[P\x84aF\xE3V[\x90_\x80\x80a7>V[\x90P_a7\x92V[\x02\x90a7\x8AV[\x90P\x91\x90\x91a7\xE9W\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x90\x04\x84\x11a7\x99a7uV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x90P\x92a74V[\x80`@\x1C\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\xF3[\x90`\x0C\x11a\x01\xD3W`\x08\x015`\xE0\x1C\x90V[\x90`L\x11a\x01\xD3W`,\x015\x90V[\x90`,\x11a\x01\xD3W`\x0C\x015\x90V[\x91\x90\x91\x82`L\x11a\x01\xD3W`L\x01\x91\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xB4\x01\x90V[Q\x90a\x02\x85\x82a\x196V[Q\x90a\x02\x85\x82a\x06hV[\x81`\x1F\x82\x01\x12\x15a\x01\xD3W\x80Q\x90a8\xC0\x82a\x1B\xCFV[\x92a8\xCE`@Q\x94\x85a\x1BQV[\x82\x84R` \x83\x83\x01\x01\x11a\x01\xD3W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x90V[a8\xF7a)\xB0V[P\x80Q\x81\x01\x90` \x81\x83\x03\x12a\x01\xD3W` \x81\x01Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\xD3W\x01\x90a\x02\0\x82\x82\x03\x12a\x01\xD3Wa91a\x1B\x92V[\x91a9>` \x82\x01a8\x93V[\x83Ra9L`@\x82\x01a\x1F\xDCV[` \x84\x01Ra9]``\x82\x01a\x1F\xDCV[`@\x84\x01R`\x80\x81\x01Q``\x84\x01R`\xA0\x81\x01Q`\x80\x84\x01R`\xC0\x81\x01Q`\xA0\x84\x01R`\xE0\x81\x01Q`\xC0\x84\x01Ra\x01\0\x81\x01Q`\xE0\x84\x01Ra\x01 \x81\x01Qa\x01\0\x84\x01Ra9\xAEa\x01@\x82\x01a8\x9EV[a\x01 \x84\x01Ra9\xC1a\x01`\x82\x01a8\x9EV[a\x01@\x84\x01Ra9\xD4a\x01\x80\x82\x01a8\x9EV[a\x01`\x84\x01Ra\x01\xA0\x81\x01Qa\x01\x80\x84\x01Ra\x01\xC0\x81\x01Qa\x01\xA0\x84\x01Ra9\xFFa\x01\xE0\x82\x01a8\x9EV[a\x01\xC0\x84\x01Ra\x02\0\x81\x01Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xD3Wa,~\x92` \x80\x92\x01\x92\x01\x01a8\xA9V[\x90a\x01\xE0\x82\x01a;\xC2a;\xB8a&\xD3a\x01\xC0a:ra:]a:Xa:R\x88QQa&mV[`\x05\x1C\x90V[a&{V[\x90`@Q\x82\x81\x93\x82R`\x01\x01`\x05\x1B\x01`@RV[` \x80\x82\x01R\x96a:\x99a:\x90a:\x8A\x83Q`\xFF\x16\x90V[`\xFF\x16\x90V[`@\x8A\x01R\x88\x90V[Pa:\xBAa:\xB1a\"\xF5` \x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[``\x8A\x01R\x88\x90V[Pa:\xDBa:\xD2a\"\xF5`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[`\x80\x8A\x01R\x88\x90V[P``\x81\x01Q`\xA0\x89\x01R`\x80\x81\x01Q`\xC0\x89\x01R`\xA0\x81\x01Q`\xE0\x89\x01R`\xC0\x81\x01Qa\x01\0\x89\x01R`\xE0\x81\x01Qa\x01 \x89\x01Ra\x01\0\x81\x01Qa\x01@\x89\x01Ra;Ba;8a&\xD3a\x01 \x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01`\x8A\x01R\x88\x90V[Pa;ia;_a&\xD3a\x01@\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\x80\x8A\x01R\x88\x90V[Pa;\x90a;\x86a&\xD3a\x01`\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\xA0\x8A\x01R\x88\x90V[Pa\x01\x80\x81\x01Qa\x01\xC0\x89\x01Ra\x01\xA0\x81\x01Qa\x01\xE0\x89\x01R\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x02\0\x85\x01R\x83\x90V[Pa\x02\0a\x02 \x84\x01R\x80QQa\x02@\x84\x01RQ\x80Q\x80a<\x07W[PP\x81Q`\x05\x1B` \x83\x01 a\x02\x85\x90\x92\x80Q`@Q\x82\x82`\x01\x01`\x05\x1B\x01\x14\x90\x15\x10`\x06\x1BRV[` \x82\x01` \x82a\x02`\x87\x01\x94\x01\x01\x90[\x81\x81\x10a<\x87WPP`\x1F\x16\x90\x81\x15a;\xDEW`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x01\x92` \x03`\x03\x1B\x1B\x01\x19\x81Q\x16\x90R_\x80a;\xDEV[\x80Q\x84R` \x93\x84\x01\x93\x01a<\x18V[\x91a<\xA9\x82_R_` R`@_ \x90V[\x80T\x90`\x02`\xA0\x83\x90\x1C`\xFF\x16a<\xBF\x81a\x07\xEFV[\x14a=\xDEW\x91a=C\x91\x7F&\xEB\xBC\xA2\x93\xADb\xA5l\xD6\xAB\xA3,\xBD\x10\xC1\x1C<\xEDl\xD78\xDC\xCB\xA8\x11\xD8\xED\xD7\x99\x1A\x9A\x93a<\xF4\x87a>\x0BV[\x91a=\x1Fa=\x05`\x80\x8A\x01Qa?\x1FV[\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x01\0\x89\x01Q\x80\x86\x10\x15a=\xD8WP\x84\x90[a=;\x82\x87a\x1F\xBAV[\x96\x87\x94a\"\x83V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x99\x86\x8B\x15a=\xA5WPP\x90a=p\x91\x85\x8Aa?kV[\x81a=\x93W[PPP[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90\x81\x01a'\xAEV[a=\x9D\x92\x87a?kV[_\x82\x81a=vV[\x92P\x92\x90Pa=\xD3\x94\x93Pa=\xCCa\x01\xC0a\x01\xE0\x85\x01Q\x94\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x93\x89a&\x96V[a=zV[\x90a=1V[\x7F\xB1\x96\xA4J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04\x84\x90R`$_\xFD[`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RF`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x90\x81`D\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x80\x15a\x16\xCDW` a>\xD0\x91`\xC0\x93_\x91a\x1E\xEBWP\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81\x03a>\xF4WP\x90V[\x7F.w\\|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80`\xA0\x1Ca?@Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x7F+\xF9Pe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x83\x15a@\xA8W`@Q\x92_\x80` \x86\x01\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x96\x87`$\x82\x01R\x88`D\x82\x01R`D\x81Ra?\xD2`d\x82a\x1BQV[Q\x90\x82\x86Z\xF1a?\xE0a&>V[\x81a@pW[Pa@iW\x81a@N\x7F\x94(u|\x177w\x8AQ\x93\x19\xE7\x9C!\x06p\xC4\x93\x93\xBF\xBE\xFB\x84\x10\xA0\xAD\xB9g\r\xCBe\x9F\x93a(\x1Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_R`\x01` R`@_ \x90V[a@Y\x87\x82Ta&\x89V[\x90U`@Q\x95\x86R\x16\x93` \x90\xA4V[PPPPPV[\x80Q\x80\x15\x92P\x82\x15a@\x85W[PP_a?\xE6V[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x01\xD3W` \x01Qa@\xA1\x81a\x01\xC9V[_\x80a@}V[PPPPV[\x90_\x91`$\x81Q\x14a@\xBDWPV[\x7F\xF7\xC3\xB3f\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$` \x84\x01Q\x93\x01Q\x92\x16\x14aA\x13WPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PV[`@Q\x7F\np\xB0V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90`\xA0\x81`$\x81\x85Z\xFA\x90\x81\x15a\x16\xCDW_\x91aB\xF8W[P`@\x80Q\x7F2\0\x05\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xFF\x16`$\x82\x01R\x92\x83\x90`D\x90\x82\x90Z\xFA\x91\x82\x15a\x16\xCDW_\x92aB\xCFW[PQ\x15[\x90\x81\x15aB\x92W[\x81\x15aBoW[PaB>WPV[\x7F\xB8%\xDDv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_Rc\xFF\xFF\xFF\xFF\x16`\x04R`$_\xFD[` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90P_aB6V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaB\xC7\x82Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x16\x15\x90aB/V[aB'\x91\x92PaB\xF0a\x15\x88\x91`@=`@\x11a\x17\xC8Wa\x17\xBA\x81\x83a\x1BQV[\x92\x91PaB#V[aC\x11\x91P`\xA0=`\xA0\x11a\x16\xC6Wa\x16\xB8\x81\x83a\x1BQV[_aA\xAEV[\x99\x97\x95\x93\x91\x98\x96\x94\x92\x90\x98`@Q\x99` \x8B\x01\x9B\x7F\x93~q=H\xC0\xCE\x14\xA0\xCAg\xEE\xD9\xA5\xA7)n\xB4\x0C\xDAr\xEC\xBCV\xD2\x88\x04\xC2\x97o\xC3k\x8DR`\xFF\x16`@\x8C\x01Rc\xFF\xFF\xFF\xFF\x16``\x8B\x01R`\x80\x8A\x01R`\xA0\x89\x01R`\xC0\x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x87\x01Ra\x01\0\x86\x01Ra\x01 \x85\x01Ra\x01@\x84\x01Ra\x01`\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x82\x01Ra\x01\x80\x81RaC\xB8a\x01\xA0\x82a\x1BQV[Q\x90 \x90V[\x95\x93\x90aDh\x93aDIa\x1A\xE4\x99\x97\x94`@s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94aD\x11\x8C\x82Q` \x80\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[` \x81\x81\x01Q\x8D\x84\x01R\x91\x01Q``\x8C\x01R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8C\x01R\x01Q`\xA0\x8A\x01RV[\x16`\xC0\x87\x01R`\xE0\x86\x01Ra\x01@a\x01\0\x86\x01Ra\x01@\x85\x01\x90a\x11XV[\x92a\x01 \x81\x85\x03\x91\x01Ra\x1FOV[\x95\x94\x91\x98\x93\x90\x98\x97\x92\x96\x97aD\xAE`@Q\x97aD\x94`@\x8Aa\x1BQV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88RV[` \x87\x01RaD\xBBa\x1B\xC0V[\x95\x86R` \x86\x01R`@\x85\x01RaD\xD0a\x1B\xB1V[0\x81R\x94` \x86\x01Rn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3;\x15a\x01\xD3W_\x95aE)\x93`@Q\x98\x89\x97\x88\x97\x7F\x13|)\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89R`\x04\x89\x01aC\xBEV[\x03\x81\x83n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3Z\xF1\x80\x15a\x16\xCDWaEKWPV[\x80a\x1A/_a\x02\x85\x93a\x1BQV[\x90\x81` \x91\x03\x12a\x01\xD3WQa\x1A\xE4\x81a\x06\x14V[\x91\x90\x82`@\x91\x03\x12a\x01\xD3W`@QaE\x86\x81a\x1B5V[` \x80\x82\x94\x80Q\x84R\x01Q\x91\x01RV[\x91\x90\x82\x81\x03`\xC0\x81\x12a\x01\xD3W`\x80\x13a\x01\xD3Wa\x1A\xE4\x90`\x80`@QaE\xBC\x81a\x1B\x14V[\x85Q\x81R` \x86\x01QaE\xCE\x81a\x06hV[` \x82\x01RaE\xE0\x83`@\x88\x01aEnV[`@\x82\x01R\x94\x01aEnV[``\x90aF\xC7aF\xB2a\x02\x85\x95\x97\x96\x94`\x80\x84Rc\xFF\xFF\xFF\xFF\x81Q\x16`\x80\x85\x01R` \x81\x01Q`\xA0\x85\x01R`@\x81\x01Q`\xC0\x85\x01R\x84\x81\x01Q`\xE0\x85\x01R`\xC0aF\x7FaFJ`\x80\x84\x01Q`\xE0a\x01\0\x89\x01Ra\x01`\x88\x01\x90a\x11XV[`\xA0\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x83\x03\x01a\x01 \x89\x01Ra\x11XV[\x91\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x83\x03\x01a\x01@\x86\x01Ra\x11XV[\x96` \x83\x01\x90` \x80\x91\x80Q\x84R\x01Q\x91\x01RV[\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x81\x81\x02\x91g\r\xE0\xB6\xB3\xA7d\0\0\x81\x83\x85\x04\x14\x83\x15\x17\x02\x15aG\x0EWPPg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFg\r\xE0\xB6\xB3\xA7d\0\0\x92\x84\t\x84\x81\x10\x85\x01\x90\x03\x92\t\x90\x80g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15aG\x89W\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x02\x90V[c\xAEG\xF7\x02_R`\x04`\x1C\xFD[\x91\x90`\x14R\x80`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10\x82\x86Z\xF1\x80`\x01_Q\x14\x16\x15aG\xD3W[PPP_`4RV[=\x83;\x15\x17\x10\x15aG\xE5W[\x80aG\xCAV[_`4Ro\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0_R_8`D`\x10\x83\x86Z\xF1P`4R` _`D`\x10\x82\x85Z\xF1\x90\x81`\x01_Q\x14\x16aG\xDFW;\x15=\x17\x10\x15aH2W_\x80aG\xDFV[c>?\x8Fs_R`\x04`\x1C\xFD[a\x1A\xE4a\x01\xE0\x91aIW`@Q\x93\x84\x92` \x80\x85\x01RaHe`@\x85\x01\x82Q`\xFF\x16\x90RV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16``\x85\x01R`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\x80\x85\x01R``\x81\x01Q`\xA0\x85\x01R`\x80\x81\x01Q`\xC0\x85\x01R`\xA0\x81\x01Q`\xE0\x85\x01R`\xC0\x81\x01Qa\x01\0\x85\x01R`\xE0\x81\x01Qa\x01 \x85\x01Ra\x01\0\x81\x01Qa\x01@\x85\x01RaH\xE2a\x01 \x82\x01Qa\x01`\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[a\x01@\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\x80\x85\x01Ra\x01`\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xA0\x85\x01Ra\x01\x80\x81\x01Qa\x01\xC0\x85\x01Ra\x01\xA0\x81\x01Q\x82\x85\x01RaIAa\x01\xC0\x82\x01Qa\x02\0\x86\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[\x01Qa\x02\0a\x02 \x84\x01Ra\x02@\x83\x01\x90a\x11XV[\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x1BQV\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
            [97u8, 124u8, 83u8, 124u8],
            [106u8, 253u8, 216u8, 80u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 118u8, 241u8, 192u8],
            [119u8, 111u8, 243u8, 199u8],
            [121u8, 80u8, 44u8, 85u8],
            [133u8, 193u8, 120u8, 48u8],
            [140u8, 218u8, 150u8, 222u8],
            [141u8, 165u8, 203u8, 91u8],
            [151u8, 195u8, 107u8, 174u8],
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
            ::core::stringify!(_executeDelivery),
            ::core::stringify!(PERMIT2),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(oftId),
            ::core::stringify!(fillFor),
            ::core::stringify!(config),
            ::core::stringify!(maxFeeRate),
            ::core::stringify!(setMaxFeeRate),
            ::core::stringify!(owner),
            ::core::stringify!(quoteFill),
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
            <_executeDeliveryCall as alloy_sol_types::SolCall>::SIGNATURE,
            <PERMIT2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <oftIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <fillForCall as alloy_sol_types::SolCall>::SIGNATURE,
            <configCall as alloy_sol_types::SolCall>::SIGNATURE,
            <maxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setMaxFeeRateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <quoteFillCall as alloy_sol_types::SolCall>::SIGNATURE,
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
