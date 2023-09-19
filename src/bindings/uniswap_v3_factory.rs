pub use uniswap_v3_factory_contract::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod uniswap_v3_factory_contract {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"indexed\":true,\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"}],\"name\":\"FeeAmountEnabled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oldOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"indexed\":false,\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\"}],\"name\":\"PoolCreated\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"}],\"name\":\"createPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"}],\"name\":\"enableFeeAmount\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\"}],\"name\":\"feeAmountTickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\"}],\"name\":\"getPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"parameters\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static UNISWAP_V3_FACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct uniswap_v3_factory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for uniswap_v3_factory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for uniswap_v3_factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for uniswap_v3_factory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for uniswap_v3_factory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(uniswap_v3_factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> uniswap_v3_factory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNISWAP_V3_FACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `createPool` (0xa1671295) function
        pub fn create_pool(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
            fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([161, 103, 18, 149], (token_a, token_b, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableFeeAmount` (0x8a7c195f) function
        pub fn enable_fee_amount(
            &self,
            fee: u32,
            tick_spacing: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 124, 25, 95], (fee, tick_spacing))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeAmountTickSpacing` (0x22afcccb) function
        pub fn fee_amount_tick_spacing(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([34, 175, 204, 203], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x1698ee82) function
        pub fn get_pool(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([22, 152, 238, 130], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parameters` (0x89035730) function
        pub fn parameters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                u32,
                i32,
            ),
        > {
            self.0
                .method_hash([137, 3, 87, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FeeAmountEnabled` event
        pub fn fee_amount_enabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeAmountEnabledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnerChanged` event
        pub fn owner_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnerChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PoolCreated` event
        pub fn pool_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PoolCreatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, uniswap_v3_factoryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for uniswap_v3_factory<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "FeeAmountEnabled", abi = "FeeAmountEnabled(uint24,int24)")]
    pub struct FeeAmountEnabledFilter {
        #[ethevent(indexed)]
        pub fee: u32,
        #[ethevent(indexed)]
        pub tick_spacing: i32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address,address)")]
    pub struct OwnerChangedFilter {
        #[ethevent(indexed)]
        pub old_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "PoolCreated",
        abi = "PoolCreated(address,address,uint24,int24,address)"
    )]
    pub struct PoolCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub fee: u32,
        pub tick_spacing: i32,
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum uniswap_v3_factoryEvents {
        FeeAmountEnabledFilter(FeeAmountEnabledFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        PoolCreatedFilter(PoolCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for uniswap_v3_factoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FeeAmountEnabledFilter::decode_log(log) {
                return Ok(uniswap_v3_factoryEvents::FeeAmountEnabledFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(uniswap_v3_factoryEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = PoolCreatedFilter::decode_log(log) {
                return Ok(uniswap_v3_factoryEvents::PoolCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for uniswap_v3_factoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeAmountEnabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeAmountEnabledFilter> for uniswap_v3_factoryEvents {
        fn from(value: FeeAmountEnabledFilter) -> Self {
            Self::FeeAmountEnabledFilter(value)
        }
    }
    impl ::core::convert::From<OwnerChangedFilter> for uniswap_v3_factoryEvents {
        fn from(value: OwnerChangedFilter) -> Self {
            Self::OwnerChangedFilter(value)
        }
    }
    impl ::core::convert::From<PoolCreatedFilter> for uniswap_v3_factoryEvents {
        fn from(value: PoolCreatedFilter) -> Self {
            Self::PoolCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(address,address,uint24)` and selector `0xa1671295`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "createPool", abi = "createPool(address,address,uint24)")]
    pub struct CreatePoolCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
        pub fee: u32,
    }
    ///Container type for all input parameters for the `enableFeeAmount` function with signature `enableFeeAmount(uint24,int24)` and selector `0x8a7c195f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "enableFeeAmount", abi = "enableFeeAmount(uint24,int24)")]
    pub struct EnableFeeAmountCall {
        pub fee: u32,
        pub tick_spacing: i32,
    }
    ///Container type for all input parameters for the `feeAmountTickSpacing` function with signature `feeAmountTickSpacing(uint24)` and selector `0x22afcccb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeAmountTickSpacing", abi = "feeAmountTickSpacing(uint24)")]
    pub struct FeeAmountTickSpacingCall(pub u32);
    ///Container type for all input parameters for the `getPool` function with signature `getPool(address,address,uint24)` and selector `0x1698ee82`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPool", abi = "getPool(address,address,uint24)")]
    pub struct GetPoolCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub u32,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `parameters` function with signature `parameters()` and selector `0x89035730`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "parameters", abi = "parameters()")]
    pub struct ParametersCall;
    ///Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `0x13af4035`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum uniswap_v3_factoryCalls {
        CreatePool(CreatePoolCall),
        EnableFeeAmount(EnableFeeAmountCall),
        FeeAmountTickSpacing(FeeAmountTickSpacingCall),
        GetPool(GetPoolCall),
        Owner(OwnerCall),
        Parameters(ParametersCall),
        SetOwner(SetOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for uniswap_v3_factoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded) =
                <EnableFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnableFeeAmount(decoded));
            }
            if let Ok(decoded) =
                <FeeAmountTickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeAmountTickSpacing(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ParametersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Parameters(decoded));
            }
            if let Ok(decoded) = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for uniswap_v3_factoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreatePool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnableFeeAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeAmountTickSpacing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Parameters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for uniswap_v3_factoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableFeeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeAmountTickSpacing(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Parameters(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreatePoolCall> for uniswap_v3_factoryCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<EnableFeeAmountCall> for uniswap_v3_factoryCalls {
        fn from(value: EnableFeeAmountCall) -> Self {
            Self::EnableFeeAmount(value)
        }
    }
    impl ::core::convert::From<FeeAmountTickSpacingCall> for uniswap_v3_factoryCalls {
        fn from(value: FeeAmountTickSpacingCall) -> Self {
            Self::FeeAmountTickSpacing(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for uniswap_v3_factoryCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for uniswap_v3_factoryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ParametersCall> for uniswap_v3_factoryCalls {
        fn from(value: ParametersCall) -> Self {
            Self::Parameters(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for uniswap_v3_factoryCalls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(address,address,uint24)` and selector `0xa1671295`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreatePoolReturn {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `feeAmountTickSpacing` function with signature `feeAmountTickSpacing(uint24)` and selector `0x22afcccb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeAmountTickSpacingReturn(pub i32);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address,uint24)` and selector `0x1698ee82`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `parameters` function with signature `parameters()` and selector `0x89035730`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ParametersReturn {
        pub factory: ::ethers::core::types::Address,
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_spacing: i32,
    }
}
