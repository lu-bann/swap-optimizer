pub use uniswap_universal_router_contract::*;
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
pub mod uniswap_universal_router_contract {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"permit2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"weth9\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"seaport\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"nftxZap\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"x2y2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"foundation\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"sudoswap\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"nft20Zap\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"cryptopunks\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"looksRare\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"routerRewardsDistributor\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"looksRareRewardsDistributor\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"looksRareToken\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"v2Factory\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"v3Factory\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"pairInitCodeHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"poolInitCodeHash\",\"type\":\"bytes32\"}],\"internalType\":\"struct RouterParameters\",\"name\":\"params\",\"type\":\"tuple\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"ContractLocked\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ETHNotAccepted\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commandIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\"}],\"name\":\"ExecutionFailed\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"FromAddressIsNotOwner\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InsufficientETH\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InsufficientToken\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidBips\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commandType\",\"type\":\"uint256\"}],\"name\":\"InvalidCommandType\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidOwnerERC1155\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidOwnerERC721\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidPath\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidReserves\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"LengthMismatch\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NoSlice\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SliceOutOfBounds\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SliceOverflow\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToAddressOutOfBounds\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToAddressOverflow\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToUint24OutOfBounds\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ToUint24Overflow\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"TransactionDeadlinePassed\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnableToClaim\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnsafeCast\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V2InvalidPath\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V2TooLittleReceived\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V2TooMuchRequested\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3InvalidAmountOut\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3InvalidCaller\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3InvalidSwap\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3TooLittleReceived\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"V3TooMuchRequested\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"RewardsSent\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"looksRareClaim\",\"type\":\"bytes\"}],\"name\":\"collectRewards\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"commands\",\"type\":\"bytes\"},{\"internalType\":\"bytes[]\",\"name\":\"inputs\",\"type\":\"bytes[]\"}],\"name\":\"execute\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"commands\",\"type\":\"bytes\"},{\"internalType\":\"bytes[]\",\"name\":\"inputs\",\"type\":\"bytes[]\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"}],\"name\":\"execute\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\"}],\"name\":\"onERC721Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0Delta\",\"type\":\"int256\"},{\"internalType\":\"int256\",\"name\":\"amount1Delta\",\"type\":\"int256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"uniswapV3SwapCallback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]";
    ///The parsed JSON ABI of the contract.
    pub static UNISWAP_UNIVERSAL_ROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct uniswap_universal_router<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for uniswap_universal_router<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for uniswap_universal_router<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for uniswap_universal_router<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for uniswap_universal_router<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(uniswap_universal_router))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> uniswap_universal_router<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNISWAP_UNIVERSAL_ROUTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `collectRewards` (0x709a1cc2) function
        pub fn collect_rewards(
            &self,
            looks_rare_claim: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 154, 28, 194], looks_rare_claim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x24856bc3) function
        pub fn execute(
            &self,
            commands: ::ethers::core::types::Bytes,
            inputs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 133, 107, 195], (commands, inputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x3593564c) function
        pub fn execute_with_commands_and_inputs(
            &self,
            commands: ::ethers::core::types::Bytes,
            inputs: ::std::vec::Vec<::ethers::core::types::Bytes>,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 147, 86, 76], (commands, inputs, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RewardsSent` event
        pub fn rewards_sent_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardsSentFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RewardsSentFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for uniswap_universal_router<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ContractLocked` with signature `ContractLocked()` and selector `0x6f5ffb7e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ContractLocked", abi = "ContractLocked()")]
    pub struct ContractLocked;
    ///Custom Error type `ETHNotAccepted` with signature `ETHNotAccepted()` and selector `0x1231ae40`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ETHNotAccepted", abi = "ETHNotAccepted()")]
    pub struct ETHNotAccepted;
    ///Custom Error type `ExecutionFailed` with signature `ExecutionFailed(uint256,bytes)` and selector `0x2c4029e9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ExecutionFailed", abi = "ExecutionFailed(uint256,bytes)")]
    pub struct ExecutionFailed {
        pub command_index: ::ethers::core::types::U256,
        pub message: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `FromAddressIsNotOwner` with signature `FromAddressIsNotOwner()` and selector `0xe7002877`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FromAddressIsNotOwner", abi = "FromAddressIsNotOwner()")]
    pub struct FromAddressIsNotOwner;
    ///Custom Error type `InsufficientETH` with signature `InsufficientETH()` and selector `0x6a12f104`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InsufficientETH", abi = "InsufficientETH()")]
    pub struct InsufficientETH;
    ///Custom Error type `InsufficientToken` with signature `InsufficientToken()` and selector `0x675cae38`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InsufficientToken", abi = "InsufficientToken()")]
    pub struct InsufficientToken;
    ///Custom Error type `InvalidBips` with signature `InvalidBips()` and selector `0xdeaa01e6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidBips", abi = "InvalidBips()")]
    pub struct InvalidBips;
    ///Custom Error type `InvalidCommandType` with signature `InvalidCommandType(uint256)` and selector `0xd76a1e9e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidCommandType", abi = "InvalidCommandType(uint256)")]
    pub struct InvalidCommandType {
        pub command_type: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidOwnerERC1155` with signature `InvalidOwnerERC1155()` and selector `0x483a6929`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidOwnerERC1155", abi = "InvalidOwnerERC1155()")]
    pub struct InvalidOwnerERC1155;
    ///Custom Error type `InvalidOwnerERC721` with signature `InvalidOwnerERC721()` and selector `0x7dbe7e89`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidOwnerERC721", abi = "InvalidOwnerERC721()")]
    pub struct InvalidOwnerERC721;
    ///Custom Error type `InvalidPath` with signature `InvalidPath()` and selector `0x20db8267`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidPath", abi = "InvalidPath()")]
    pub struct InvalidPath;
    ///Custom Error type `InvalidReserves` with signature `InvalidReserves()` and selector `0x7b9c8916`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidReserves", abi = "InvalidReserves()")]
    pub struct InvalidReserves;
    ///Custom Error type `LengthMismatch` with signature `LengthMismatch()` and selector `0xff633a38`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LengthMismatch", abi = "LengthMismatch()")]
    pub struct LengthMismatch;
    ///Custom Error type `NoSlice` with signature `NoSlice()` and selector `0xcc94a63a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoSlice", abi = "NoSlice()")]
    pub struct NoSlice;
    ///Custom Error type `SliceOutOfBounds` with signature `SliceOutOfBounds()` and selector `0x3b99b53d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SliceOutOfBounds", abi = "SliceOutOfBounds()")]
    pub struct SliceOutOfBounds;
    ///Custom Error type `SliceOverflow` with signature `SliceOverflow()` and selector `0x47aaf07a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SliceOverflow", abi = "SliceOverflow()")]
    pub struct SliceOverflow;
    ///Custom Error type `ToAddressOutOfBounds` with signature `ToAddressOutOfBounds()` and selector `0xa78aa27f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ToAddressOutOfBounds", abi = "ToAddressOutOfBounds()")]
    pub struct ToAddressOutOfBounds;
    ///Custom Error type `ToAddressOverflow` with signature `ToAddressOverflow()` and selector `0x77146e62`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ToAddressOverflow", abi = "ToAddressOverflow()")]
    pub struct ToAddressOverflow;
    ///Custom Error type `ToUint24OutOfBounds` with signature `ToUint24OutOfBounds()` and selector `0xd9096a3e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ToUint24OutOfBounds", abi = "ToUint24OutOfBounds()")]
    pub struct ToUint24OutOfBounds;
    ///Custom Error type `ToUint24Overflow` with signature `ToUint24Overflow()` and selector `0x855859b4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ToUint24Overflow", abi = "ToUint24Overflow()")]
    pub struct ToUint24Overflow;
    ///Custom Error type `TransactionDeadlinePassed` with signature `TransactionDeadlinePassed()` and selector `0x5bf6f916`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "TransactionDeadlinePassed",
        abi = "TransactionDeadlinePassed()"
    )]
    pub struct TransactionDeadlinePassed;
    ///Custom Error type `UnableToClaim` with signature `UnableToClaim()` and selector `0x7d529919`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "UnableToClaim", abi = "UnableToClaim()")]
    pub struct UnableToClaim;
    ///Custom Error type `UnsafeCast` with signature `UnsafeCast()` and selector `0xc4bd89a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "UnsafeCast", abi = "UnsafeCast()")]
    pub struct UnsafeCast;
    ///Custom Error type `V2InvalidPath` with signature `V2InvalidPath()` and selector `0xae52ad0c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V2InvalidPath", abi = "V2InvalidPath()")]
    pub struct V2InvalidPath;
    ///Custom Error type `V2TooLittleReceived` with signature `V2TooLittleReceived()` and selector `0x849eaf98`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V2TooLittleReceived", abi = "V2TooLittleReceived()")]
    pub struct V2TooLittleReceived;
    ///Custom Error type `V2TooMuchRequested` with signature `V2TooMuchRequested()` and selector `0x8ab0bc16`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V2TooMuchRequested", abi = "V2TooMuchRequested()")]
    pub struct V2TooMuchRequested;
    ///Custom Error type `V3InvalidAmountOut` with signature `V3InvalidAmountOut()` and selector `0xd4e0248e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V3InvalidAmountOut", abi = "V3InvalidAmountOut()")]
    pub struct V3InvalidAmountOut;
    ///Custom Error type `V3InvalidCaller` with signature `V3InvalidCaller()` and selector `0x32b13d91`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V3InvalidCaller", abi = "V3InvalidCaller()")]
    pub struct V3InvalidCaller;
    ///Custom Error type `V3InvalidSwap` with signature `V3InvalidSwap()` and selector `0x316cf0eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V3InvalidSwap", abi = "V3InvalidSwap()")]
    pub struct V3InvalidSwap;
    ///Custom Error type `V3TooLittleReceived` with signature `V3TooLittleReceived()` and selector `0x39d35496`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V3TooLittleReceived", abi = "V3TooLittleReceived()")]
    pub struct V3TooLittleReceived;
    ///Custom Error type `V3TooMuchRequested` with signature `V3TooMuchRequested()` and selector `0x739dbe52`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "V3TooMuchRequested", abi = "V3TooMuchRequested()")]
    pub struct V3TooMuchRequested;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum uniswap_universal_routerErrors {
        ContractLocked(ContractLocked),
        ETHNotAccepted(ETHNotAccepted),
        ExecutionFailed(ExecutionFailed),
        FromAddressIsNotOwner(FromAddressIsNotOwner),
        InsufficientETH(InsufficientETH),
        InsufficientToken(InsufficientToken),
        InvalidBips(InvalidBips),
        InvalidCommandType(InvalidCommandType),
        InvalidOwnerERC1155(InvalidOwnerERC1155),
        InvalidOwnerERC721(InvalidOwnerERC721),
        InvalidPath(InvalidPath),
        InvalidReserves(InvalidReserves),
        LengthMismatch(LengthMismatch),
        NoSlice(NoSlice),
        SliceOutOfBounds(SliceOutOfBounds),
        SliceOverflow(SliceOverflow),
        ToAddressOutOfBounds(ToAddressOutOfBounds),
        ToAddressOverflow(ToAddressOverflow),
        ToUint24OutOfBounds(ToUint24OutOfBounds),
        ToUint24Overflow(ToUint24Overflow),
        TransactionDeadlinePassed(TransactionDeadlinePassed),
        UnableToClaim(UnableToClaim),
        UnsafeCast(UnsafeCast),
        V2InvalidPath(V2InvalidPath),
        V2TooLittleReceived(V2TooLittleReceived),
        V2TooMuchRequested(V2TooMuchRequested),
        V3InvalidAmountOut(V3InvalidAmountOut),
        V3InvalidCaller(V3InvalidCaller),
        V3InvalidSwap(V3InvalidSwap),
        V3TooLittleReceived(V3TooLittleReceived),
        V3TooMuchRequested(V3TooMuchRequested),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for uniswap_universal_routerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ContractLocked as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ContractLocked(decoded));
            }
            if let Ok(decoded) = <ETHNotAccepted as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ETHNotAccepted(decoded));
            }
            if let Ok(decoded) = <ExecutionFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutionFailed(decoded));
            }
            if let Ok(decoded) =
                <FromAddressIsNotOwner as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FromAddressIsNotOwner(decoded));
            }
            if let Ok(decoded) = <InsufficientETH as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InsufficientETH(decoded));
            }
            if let Ok(decoded) = <InsufficientToken as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientToken(decoded));
            }
            if let Ok(decoded) = <InvalidBips as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidBips(decoded));
            }
            if let Ok(decoded) =
                <InvalidCommandType as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidCommandType(decoded));
            }
            if let Ok(decoded) =
                <InvalidOwnerERC1155 as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidOwnerERC1155(decoded));
            }
            if let Ok(decoded) =
                <InvalidOwnerERC721 as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidOwnerERC721(decoded));
            }
            if let Ok(decoded) = <InvalidPath as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPath(decoded));
            }
            if let Ok(decoded) = <InvalidReserves as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidReserves(decoded));
            }
            if let Ok(decoded) = <LengthMismatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LengthMismatch(decoded));
            }
            if let Ok(decoded) = <NoSlice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoSlice(decoded));
            }
            if let Ok(decoded) = <SliceOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SliceOutOfBounds(decoded));
            }
            if let Ok(decoded) = <SliceOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SliceOverflow(decoded));
            }
            if let Ok(decoded) =
                <ToAddressOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ToAddressOutOfBounds(decoded));
            }
            if let Ok(decoded) = <ToAddressOverflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ToAddressOverflow(decoded));
            }
            if let Ok(decoded) =
                <ToUint24OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ToUint24OutOfBounds(decoded));
            }
            if let Ok(decoded) = <ToUint24Overflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ToUint24Overflow(decoded));
            }
            if let Ok(decoded) =
                <TransactionDeadlinePassed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransactionDeadlinePassed(decoded));
            }
            if let Ok(decoded) = <UnableToClaim as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnableToClaim(decoded));
            }
            if let Ok(decoded) = <UnsafeCast as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnsafeCast(decoded));
            }
            if let Ok(decoded) = <V2InvalidPath as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::V2InvalidPath(decoded));
            }
            if let Ok(decoded) =
                <V2TooLittleReceived as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V2TooLittleReceived(decoded));
            }
            if let Ok(decoded) =
                <V2TooMuchRequested as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V2TooMuchRequested(decoded));
            }
            if let Ok(decoded) =
                <V3InvalidAmountOut as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V3InvalidAmountOut(decoded));
            }
            if let Ok(decoded) = <V3InvalidCaller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::V3InvalidCaller(decoded));
            }
            if let Ok(decoded) = <V3InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::V3InvalidSwap(decoded));
            }
            if let Ok(decoded) =
                <V3TooLittleReceived as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V3TooLittleReceived(decoded));
            }
            if let Ok(decoded) =
                <V3TooMuchRequested as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::V3TooMuchRequested(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for uniswap_universal_routerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ContractLocked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ETHNotAccepted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecutionFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FromAddressIsNotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidBips(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidCommandType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwnerERC1155(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwnerERC721(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPath(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidReserves(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LengthMismatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoSlice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SliceOutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SliceOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToAddressOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToAddressOverflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToUint24OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToUint24Overflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransactionDeadlinePassed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnableToClaim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsafeCast(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::V2InvalidPath(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::V2TooLittleReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V2TooMuchRequested(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V3InvalidAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V3InvalidCaller(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::V3InvalidSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::V3TooLittleReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::V3TooMuchRequested(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for uniswap_universal_routerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <ContractLocked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ETHNotAccepted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ExecutionFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FromAddressIsNotOwner as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InsufficientETH as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientToken as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidBips as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidCommandType as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidOwnerERC1155 as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidOwnerERC721 as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidPath as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <LengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NoSlice as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SliceOutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <SliceOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ToAddressOutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ToAddressOverflow as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ToUint24OutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <ToUint24Overflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransactionDeadlinePassed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <UnableToClaim as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <UnsafeCast as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <V2InvalidPath as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V2TooLittleReceived as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <V2TooMuchRequested as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <V3InvalidAmountOut as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <V3InvalidCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <V3InvalidSwap as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V3TooLittleReceived as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <V3TooMuchRequested as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for uniswap_universal_routerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ETHNotAccepted(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutionFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromAddressIsNotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBips(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCommandType(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidOwnerERC1155(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidOwnerERC721(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::LengthMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoSlice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SliceOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SliceOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToAddressOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToAddressOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToUint24OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToUint24Overflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactionDeadlinePassed(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnableToClaim(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeCast(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2InvalidPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2TooLittleReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2TooMuchRequested(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3InvalidAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3InvalidCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3TooLittleReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3TooMuchRequested(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for uniswap_universal_routerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ContractLocked> for uniswap_universal_routerErrors {
        fn from(value: ContractLocked) -> Self {
            Self::ContractLocked(value)
        }
    }
    impl ::core::convert::From<ETHNotAccepted> for uniswap_universal_routerErrors {
        fn from(value: ETHNotAccepted) -> Self {
            Self::ETHNotAccepted(value)
        }
    }
    impl ::core::convert::From<ExecutionFailed> for uniswap_universal_routerErrors {
        fn from(value: ExecutionFailed) -> Self {
            Self::ExecutionFailed(value)
        }
    }
    impl ::core::convert::From<FromAddressIsNotOwner> for uniswap_universal_routerErrors {
        fn from(value: FromAddressIsNotOwner) -> Self {
            Self::FromAddressIsNotOwner(value)
        }
    }
    impl ::core::convert::From<InsufficientETH> for uniswap_universal_routerErrors {
        fn from(value: InsufficientETH) -> Self {
            Self::InsufficientETH(value)
        }
    }
    impl ::core::convert::From<InsufficientToken> for uniswap_universal_routerErrors {
        fn from(value: InsufficientToken) -> Self {
            Self::InsufficientToken(value)
        }
    }
    impl ::core::convert::From<InvalidBips> for uniswap_universal_routerErrors {
        fn from(value: InvalidBips) -> Self {
            Self::InvalidBips(value)
        }
    }
    impl ::core::convert::From<InvalidCommandType> for uniswap_universal_routerErrors {
        fn from(value: InvalidCommandType) -> Self {
            Self::InvalidCommandType(value)
        }
    }
    impl ::core::convert::From<InvalidOwnerERC1155> for uniswap_universal_routerErrors {
        fn from(value: InvalidOwnerERC1155) -> Self {
            Self::InvalidOwnerERC1155(value)
        }
    }
    impl ::core::convert::From<InvalidOwnerERC721> for uniswap_universal_routerErrors {
        fn from(value: InvalidOwnerERC721) -> Self {
            Self::InvalidOwnerERC721(value)
        }
    }
    impl ::core::convert::From<InvalidPath> for uniswap_universal_routerErrors {
        fn from(value: InvalidPath) -> Self {
            Self::InvalidPath(value)
        }
    }
    impl ::core::convert::From<InvalidReserves> for uniswap_universal_routerErrors {
        fn from(value: InvalidReserves) -> Self {
            Self::InvalidReserves(value)
        }
    }
    impl ::core::convert::From<LengthMismatch> for uniswap_universal_routerErrors {
        fn from(value: LengthMismatch) -> Self {
            Self::LengthMismatch(value)
        }
    }
    impl ::core::convert::From<NoSlice> for uniswap_universal_routerErrors {
        fn from(value: NoSlice) -> Self {
            Self::NoSlice(value)
        }
    }
    impl ::core::convert::From<SliceOutOfBounds> for uniswap_universal_routerErrors {
        fn from(value: SliceOutOfBounds) -> Self {
            Self::SliceOutOfBounds(value)
        }
    }
    impl ::core::convert::From<SliceOverflow> for uniswap_universal_routerErrors {
        fn from(value: SliceOverflow) -> Self {
            Self::SliceOverflow(value)
        }
    }
    impl ::core::convert::From<ToAddressOutOfBounds> for uniswap_universal_routerErrors {
        fn from(value: ToAddressOutOfBounds) -> Self {
            Self::ToAddressOutOfBounds(value)
        }
    }
    impl ::core::convert::From<ToAddressOverflow> for uniswap_universal_routerErrors {
        fn from(value: ToAddressOverflow) -> Self {
            Self::ToAddressOverflow(value)
        }
    }
    impl ::core::convert::From<ToUint24OutOfBounds> for uniswap_universal_routerErrors {
        fn from(value: ToUint24OutOfBounds) -> Self {
            Self::ToUint24OutOfBounds(value)
        }
    }
    impl ::core::convert::From<ToUint24Overflow> for uniswap_universal_routerErrors {
        fn from(value: ToUint24Overflow) -> Self {
            Self::ToUint24Overflow(value)
        }
    }
    impl ::core::convert::From<TransactionDeadlinePassed> for uniswap_universal_routerErrors {
        fn from(value: TransactionDeadlinePassed) -> Self {
            Self::TransactionDeadlinePassed(value)
        }
    }
    impl ::core::convert::From<UnableToClaim> for uniswap_universal_routerErrors {
        fn from(value: UnableToClaim) -> Self {
            Self::UnableToClaim(value)
        }
    }
    impl ::core::convert::From<UnsafeCast> for uniswap_universal_routerErrors {
        fn from(value: UnsafeCast) -> Self {
            Self::UnsafeCast(value)
        }
    }
    impl ::core::convert::From<V2InvalidPath> for uniswap_universal_routerErrors {
        fn from(value: V2InvalidPath) -> Self {
            Self::V2InvalidPath(value)
        }
    }
    impl ::core::convert::From<V2TooLittleReceived> for uniswap_universal_routerErrors {
        fn from(value: V2TooLittleReceived) -> Self {
            Self::V2TooLittleReceived(value)
        }
    }
    impl ::core::convert::From<V2TooMuchRequested> for uniswap_universal_routerErrors {
        fn from(value: V2TooMuchRequested) -> Self {
            Self::V2TooMuchRequested(value)
        }
    }
    impl ::core::convert::From<V3InvalidAmountOut> for uniswap_universal_routerErrors {
        fn from(value: V3InvalidAmountOut) -> Self {
            Self::V3InvalidAmountOut(value)
        }
    }
    impl ::core::convert::From<V3InvalidCaller> for uniswap_universal_routerErrors {
        fn from(value: V3InvalidCaller) -> Self {
            Self::V3InvalidCaller(value)
        }
    }
    impl ::core::convert::From<V3InvalidSwap> for uniswap_universal_routerErrors {
        fn from(value: V3InvalidSwap) -> Self {
            Self::V3InvalidSwap(value)
        }
    }
    impl ::core::convert::From<V3TooLittleReceived> for uniswap_universal_routerErrors {
        fn from(value: V3TooLittleReceived) -> Self {
            Self::V3TooLittleReceived(value)
        }
    }
    impl ::core::convert::From<V3TooMuchRequested> for uniswap_universal_routerErrors {
        fn from(value: V3TooMuchRequested) -> Self {
            Self::V3TooMuchRequested(value)
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
    #[ethevent(name = "RewardsSent", abi = "RewardsSent(uint256)")]
    pub struct RewardsSentFilter {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `collectRewards` function with signature `collectRewards(bytes)` and selector `0x709a1cc2`
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
    #[ethcall(name = "collectRewards", abi = "collectRewards(bytes)")]
    pub struct CollectRewardsCall {
        pub looks_rare_claim: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes,bytes[])` and selector `0x24856bc3`
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
    #[ethcall(name = "execute", abi = "execute(bytes,bytes[])")]
    pub struct ExecuteCall {
        pub commands: ::ethers::core::types::Bytes,
        pub inputs: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes,bytes[],uint256)` and selector `0x3593564c`
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
    #[ethcall(name = "execute", abi = "execute(bytes,bytes[],uint256)")]
    pub struct ExecuteWithCommandsAndInputsCall {
        pub commands: ::ethers::core::types::Bytes,
        pub inputs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    #[ethcall(
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    #[ethcall(
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    #[ethcall(
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum uniswap_universal_routerCalls {
        CollectRewards(CollectRewardsCall),
        Execute(ExecuteCall),
        ExecuteWithCommandsAndInputs(ExecuteWithCommandsAndInputsCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        SupportsInterface(SupportsInterfaceCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for uniswap_universal_routerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CollectRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CollectRewards(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWithCommandsAndInputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteWithCommandsAndInputs(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for uniswap_universal_routerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectRewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteWithCommandsAndInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC721Received(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for uniswap_universal_routerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithCommandsAndInputs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155BatchReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CollectRewardsCall> for uniswap_universal_routerCalls {
        fn from(value: CollectRewardsCall) -> Self {
            Self::CollectRewards(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for uniswap_universal_routerCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteWithCommandsAndInputsCall> for uniswap_universal_routerCalls {
        fn from(value: ExecuteWithCommandsAndInputsCall) -> Self {
            Self::ExecuteWithCommandsAndInputs(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for uniswap_universal_routerCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for uniswap_universal_routerCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for uniswap_universal_routerCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for uniswap_universal_routerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for uniswap_universal_routerCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
