pub use usdt_contract::*;
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
pub mod usdt_contract {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"constant\":true,\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"name\":\"\",\"type\":\"string\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_upgradedAddress\",\"type\":\"address\"}],\"name\":\"deprecate\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_spender\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"deprecated\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_evilUser\",\"type\":\"address\"}],\"name\":\"addBlackList\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_from\",\"type\":\"address\"},{\"name\":\"_to\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"upgradedAddress\",\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"address\"}],\"name\":\"balances\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"maximumFee\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"_totalSupply\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[],\"name\":\"unpause\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"_maker\",\"type\":\"address\"}],\"name\":\"getBlackListStatus\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"address\"},{\"name\":\"\",\"type\":\"address\"}],\"name\":\"allowed\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"who\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[],\"name\":\"pause\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"getOwner\",\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"name\":\"\",\"type\":\"string\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_to\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"newBasisPoints\",\"type\":\"uint256\"},{\"name\":\"newMaxFee\",\"type\":\"uint256\"}],\"name\":\"setParams\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"issue\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"redeem\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"_owner\",\"type\":\"address\"},{\"name\":\"_spender\",\"type\":\"address\"}],\"name\":\"allowance\",\"outputs\":[{\"name\":\"remaining\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"basisPointsRate\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"address\"}],\"name\":\"isBlackListed\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_clearedUser\",\"type\":\"address\"}],\"name\":\"removeBlackList\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"MAX_UINT\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_blackListedUser\",\"type\":\"address\"}],\"name\":\"destroyBlackFunds\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"name\":\"_initialSupply\",\"type\":\"uint256\"},{\"name\":\"_name\",\"type\":\"string\"},{\"name\":\"_symbol\",\"type\":\"string\"},{\"name\":\"_decimals\",\"type\":\"uint256\"}],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"Issue\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"Redeem\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"newAddress\",\"type\":\"address\"}],\"name\":\"Deprecate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"feeBasisPoints\",\"type\":\"uint256\"},{\"indexed\":false,\"name\":\"maxFee\",\"type\":\"uint256\"}],\"name\":\"Params\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"_blackListedUser\",\"type\":\"address\"},{\"indexed\":false,\"name\":\"_balance\",\"type\":\"uint256\"}],\"name\":\"DestroyedBlackFunds\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"_user\",\"type\":\"address\"}],\"name\":\"AddedBlackList\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"_user\",\"type\":\"address\"}],\"name\":\"RemovedBlackList\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"name\":\"spender\",\"type\":\"address\"},{\"indexed\":false,\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Pause\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Unpause\",\"type\":\"event\"}]";
    ///The parsed JSON ABI of the contract.
    pub static USDT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct usdt<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for usdt<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for usdt<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for usdt<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for usdt<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(usdt))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> usdt<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                USDT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `MAX_UINT` (0xe5b5019a) function
        pub fn max_uint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 181, 1, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_totalSupply` (0x3eaaf86b) function
        pub fn _total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 170, 248, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBlackList` (0x0ecb93c0) function
        pub fn add_black_list(
            &self,
            evil_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 203, 147, 192], evil_user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowed` (0x5c658165) function
        pub fn allowed(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 101, 129, 101], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            who: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], who)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0x27e235e3) function
        pub fn balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 226, 53, 227], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `basisPointsRate` (0xdd644f72) function
        pub fn basis_points_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 100, 79, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deprecate` (0x0753c30c) function
        pub fn deprecate(
            &self,
            upgraded_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 83, 195, 12], upgraded_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deprecated` (0x0e136b19) function
        pub fn deprecated(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 19, 107, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `destroyBlackFunds` (0xf3bdc228) function
        pub fn destroy_black_funds(
            &self,
            black_listed_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 189, 194, 40], black_listed_user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlackListStatus` (0x59bf1abe) function
        pub fn get_black_list_status(
            &self,
            maker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([89, 191, 26, 190], maker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwner` (0x893d20e8) function
        pub fn get_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([137, 61, 32, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBlackListed` (0xe47d6060) function
        pub fn is_black_listed(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 125, 96, 96], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `issue` (0xcc872b66) function
        pub fn issue(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 135, 43, 102], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maximumFee` (0x35390714) function
        pub fn maximum_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 57, 7, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xdb006a75) function
        pub fn redeem(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 0, 106, 117], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBlackList` (0xe4997dc5) function
        pub fn remove_black_list(
            &self,
            cleared_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 153, 125, 197], cleared_user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setParams` (0xc0324c77) function
        pub fn set_params(
            &self,
            new_basis_points: ::ethers::core::types::U256,
            new_max_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 50, 76, 119], (new_basis_points, new_max_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradedAddress` (0x26976e3f) function
        pub fn upgraded_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([38, 151, 110, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedBlackList` event
        pub fn added_black_list_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddedBlackListFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `Deprecate` event
        pub fn deprecate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeprecateFilter> {
            self.0.event()
        }
        ///Gets the contract's `DestroyedBlackFunds` event
        pub fn destroyed_black_funds_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DestroyedBlackFundsFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Issue` event
        pub fn issue_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IssueFilter> {
            self.0.event()
        }
        ///Gets the contract's `Params` event
        pub fn params_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ParamsFilter> {
            self.0.event()
        }
        ///Gets the contract's `Pause` event
        pub fn pause_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauseFilter> {
            self.0.event()
        }
        ///Gets the contract's `Redeem` event
        pub fn redeem_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RedeemFilter> {
            self.0.event()
        }
        ///Gets the contract's `RemovedBlackList` event
        pub fn removed_black_list_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedBlackListFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        ///Gets the contract's `Unpause` event
        pub fn unpause_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpauseFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, usdtEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for usdt<M> {
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
    #[ethevent(name = "AddedBlackList", abi = "AddedBlackList(address)")]
    pub struct AddedBlackListFilter {
        pub user: ::ethers::core::types::Address,
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deprecate", abi = "Deprecate(address)")]
    pub struct DeprecateFilter {
        pub new_address: ::ethers::core::types::Address,
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
        name = "DestroyedBlackFunds",
        abi = "DestroyedBlackFunds(address,uint256)"
    )]
    pub struct DestroyedBlackFundsFilter {
        pub black_listed_user: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
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
    #[ethevent(name = "Issue", abi = "Issue(uint256)")]
    pub struct IssueFilter {
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Params", abi = "Params(uint256,uint256)")]
    pub struct ParamsFilter {
        pub fee_basis_points: ::ethers::core::types::U256,
        pub max_fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "Pause", abi = "Pause()")]
    pub struct PauseFilter;
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
    #[ethevent(name = "Redeem", abi = "Redeem(uint256)")]
    pub struct RedeemFilter {
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedBlackList", abi = "RemovedBlackList(address)")]
    pub struct RemovedBlackListFilter {
        pub user: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Unpause", abi = "Unpause()")]
    pub struct UnpauseFilter;
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum usdtEvents {
        AddedBlackListFilter(AddedBlackListFilter),
        ApprovalFilter(ApprovalFilter),
        DeprecateFilter(DeprecateFilter),
        DestroyedBlackFundsFilter(DestroyedBlackFundsFilter),
        IssueFilter(IssueFilter),
        ParamsFilter(ParamsFilter),
        PauseFilter(PauseFilter),
        RedeemFilter(RedeemFilter),
        RemovedBlackListFilter(RemovedBlackListFilter),
        TransferFilter(TransferFilter),
        UnpauseFilter(UnpauseFilter),
    }
    impl ::ethers::contract::EthLogDecode for usdtEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedBlackListFilter::decode_log(log) {
                return Ok(usdtEvents::AddedBlackListFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(usdtEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DeprecateFilter::decode_log(log) {
                return Ok(usdtEvents::DeprecateFilter(decoded));
            }
            if let Ok(decoded) = DestroyedBlackFundsFilter::decode_log(log) {
                return Ok(usdtEvents::DestroyedBlackFundsFilter(decoded));
            }
            if let Ok(decoded) = IssueFilter::decode_log(log) {
                return Ok(usdtEvents::IssueFilter(decoded));
            }
            if let Ok(decoded) = ParamsFilter::decode_log(log) {
                return Ok(usdtEvents::ParamsFilter(decoded));
            }
            if let Ok(decoded) = PauseFilter::decode_log(log) {
                return Ok(usdtEvents::PauseFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(usdtEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RemovedBlackListFilter::decode_log(log) {
                return Ok(usdtEvents::RemovedBlackListFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(usdtEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpauseFilter::decode_log(log) {
                return Ok(usdtEvents::UnpauseFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for usdtEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedBlackListFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeprecateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DestroyedBlackFundsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IssueFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParamsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedBlackListFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpauseFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedBlackListFilter> for usdtEvents {
        fn from(value: AddedBlackListFilter) -> Self {
            Self::AddedBlackListFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for usdtEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DeprecateFilter> for usdtEvents {
        fn from(value: DeprecateFilter) -> Self {
            Self::DeprecateFilter(value)
        }
    }
    impl ::core::convert::From<DestroyedBlackFundsFilter> for usdtEvents {
        fn from(value: DestroyedBlackFundsFilter) -> Self {
            Self::DestroyedBlackFundsFilter(value)
        }
    }
    impl ::core::convert::From<IssueFilter> for usdtEvents {
        fn from(value: IssueFilter) -> Self {
            Self::IssueFilter(value)
        }
    }
    impl ::core::convert::From<ParamsFilter> for usdtEvents {
        fn from(value: ParamsFilter) -> Self {
            Self::ParamsFilter(value)
        }
    }
    impl ::core::convert::From<PauseFilter> for usdtEvents {
        fn from(value: PauseFilter) -> Self {
            Self::PauseFilter(value)
        }
    }
    impl ::core::convert::From<RedeemFilter> for usdtEvents {
        fn from(value: RedeemFilter) -> Self {
            Self::RedeemFilter(value)
        }
    }
    impl ::core::convert::From<RemovedBlackListFilter> for usdtEvents {
        fn from(value: RemovedBlackListFilter) -> Self {
            Self::RemovedBlackListFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for usdtEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UnpauseFilter> for usdtEvents {
        fn from(value: UnpauseFilter) -> Self {
            Self::UnpauseFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_UINT` function with signature `MAX_UINT()` and selector `0xe5b5019a`
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
    #[ethcall(name = "MAX_UINT", abi = "MAX_UINT()")]
    pub struct MaxUintCall;
    ///Container type for all input parameters for the `_totalSupply` function with signature `_totalSupply()` and selector `0x3eaaf86b`
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
    #[ethcall(name = "_totalSupply", abi = "_totalSupply()")]
    pub struct _TotalSupplyCall;
    ///Container type for all input parameters for the `addBlackList` function with signature `addBlackList(address)` and selector `0x0ecb93c0`
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
    #[ethcall(name = "addBlackList", abi = "addBlackList(address)")]
    pub struct AddBlackListCall {
        pub evil_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowed` function with signature `allowed(address,address)` and selector `0x5c658165`
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
    #[ethcall(name = "allowed", abi = "allowed(address,address)")]
    pub struct AllowedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub who: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    #[ethcall(name = "balances", abi = "balances(address)")]
    pub struct BalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `basisPointsRate` function with signature `basisPointsRate()` and selector `0xdd644f72`
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
    #[ethcall(name = "basisPointsRate", abi = "basisPointsRate()")]
    pub struct BasisPointsRateCall;
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `deprecate` function with signature `deprecate(address)` and selector `0x0753c30c`
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
    #[ethcall(name = "deprecate", abi = "deprecate(address)")]
    pub struct DeprecateCall {
        pub upgraded_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deprecated` function with signature `deprecated()` and selector `0x0e136b19`
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
    #[ethcall(name = "deprecated", abi = "deprecated()")]
    pub struct DeprecatedCall;
    ///Container type for all input parameters for the `destroyBlackFunds` function with signature `destroyBlackFunds(address)` and selector `0xf3bdc228`
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
    #[ethcall(name = "destroyBlackFunds", abi = "destroyBlackFunds(address)")]
    pub struct DestroyBlackFundsCall {
        pub black_listed_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getBlackListStatus` function with signature `getBlackListStatus(address)` and selector `0x59bf1abe`
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
    #[ethcall(name = "getBlackListStatus", abi = "getBlackListStatus(address)")]
    pub struct GetBlackListStatusCall {
        pub maker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOwner` function with signature `getOwner()` and selector `0x893d20e8`
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
    #[ethcall(name = "getOwner", abi = "getOwner()")]
    pub struct GetOwnerCall;
    ///Container type for all input parameters for the `isBlackListed` function with signature `isBlackListed(address)` and selector `0xe47d6060`
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
    #[ethcall(name = "isBlackListed", abi = "isBlackListed(address)")]
    pub struct IsBlackListedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `issue` function with signature `issue(uint256)` and selector `0xcc872b66`
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
    #[ethcall(name = "issue", abi = "issue(uint256)")]
    pub struct IssueCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `maximumFee` function with signature `maximumFee()` and selector `0x35390714`
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
    #[ethcall(name = "maximumFee", abi = "maximumFee()")]
    pub struct MaximumFeeCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256)` and selector `0xdb006a75`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256)")]
    pub struct RedeemCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeBlackList` function with signature `removeBlackList(address)` and selector `0xe4997dc5`
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
    #[ethcall(name = "removeBlackList", abi = "removeBlackList(address)")]
    pub struct RemoveBlackListCall {
        pub cleared_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setParams` function with signature `setParams(uint256,uint256)` and selector `0xc0324c77`
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
    #[ethcall(name = "setParams", abi = "setParams(uint256,uint256)")]
    pub struct SetParamsCall {
        pub new_basis_points: ::ethers::core::types::U256,
        pub new_max_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `upgradedAddress` function with signature `upgradedAddress()` and selector `0x26976e3f`
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
    #[ethcall(name = "upgradedAddress", abi = "upgradedAddress()")]
    pub struct UpgradedAddressCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum usdtCalls {
        MaxUint(MaxUintCall),
        _TotalSupply(_TotalSupplyCall),
        AddBlackList(AddBlackListCall),
        Allowance(AllowanceCall),
        Allowed(AllowedCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Balances(BalancesCall),
        BasisPointsRate(BasisPointsRateCall),
        Decimals(DecimalsCall),
        Deprecate(DeprecateCall),
        Deprecated(DeprecatedCall),
        DestroyBlackFunds(DestroyBlackFundsCall),
        GetBlackListStatus(GetBlackListStatusCall),
        GetOwner(GetOwnerCall),
        IsBlackListed(IsBlackListedCall),
        Issue(IssueCall),
        MaximumFee(MaximumFeeCall),
        Name(NameCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Redeem(RedeemCall),
        RemoveBlackList(RemoveBlackListCall),
        SetParams(SetParamsCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpgradedAddress(UpgradedAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for usdtCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxUintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxUint(decoded));
            }
            if let Ok(decoded) = <_TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::_TotalSupply(decoded));
            }
            if let Ok(decoded) = <AddBlackListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddBlackList(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <AllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowed(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded) =
                <BasisPointsRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BasisPointsRate(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DeprecateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deprecate(decoded));
            }
            if let Ok(decoded) = <DeprecatedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deprecated(decoded));
            }
            if let Ok(decoded) =
                <DestroyBlackFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DestroyBlackFunds(decoded));
            }
            if let Ok(decoded) =
                <GetBlackListStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBlackListStatus(decoded));
            }
            if let Ok(decoded) = <GetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOwner(decoded));
            }
            if let Ok(decoded) = <IsBlackListedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsBlackListed(decoded));
            }
            if let Ok(decoded) = <IssueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Issue(decoded));
            }
            if let Ok(decoded) = <MaximumFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaximumFee(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RemoveBlackListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveBlackList(decoded));
            }
            if let Ok(decoded) = <SetParamsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetParams(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpgradedAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpgradedAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for usdtCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::_TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddBlackList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Balances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BasisPointsRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deprecate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deprecated(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DestroyBlackFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBlackListStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsBlackListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Issue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaximumFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveBlackList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradedAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for usdtCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::_TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBlackList(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::BasisPointsRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deprecate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deprecated(element) => ::core::fmt::Display::fmt(element, f),
                Self::DestroyBlackFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlackListStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsBlackListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Issue(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximumFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBlackList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxUintCall> for usdtCalls {
        fn from(value: MaxUintCall) -> Self {
            Self::MaxUint(value)
        }
    }
    impl ::core::convert::From<_TotalSupplyCall> for usdtCalls {
        fn from(value: _TotalSupplyCall) -> Self {
            Self::_TotalSupply(value)
        }
    }
    impl ::core::convert::From<AddBlackListCall> for usdtCalls {
        fn from(value: AddBlackListCall) -> Self {
            Self::AddBlackList(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for usdtCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<AllowedCall> for usdtCalls {
        fn from(value: AllowedCall) -> Self {
            Self::Allowed(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for usdtCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for usdtCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for usdtCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<BasisPointsRateCall> for usdtCalls {
        fn from(value: BasisPointsRateCall) -> Self {
            Self::BasisPointsRate(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for usdtCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DeprecateCall> for usdtCalls {
        fn from(value: DeprecateCall) -> Self {
            Self::Deprecate(value)
        }
    }
    impl ::core::convert::From<DeprecatedCall> for usdtCalls {
        fn from(value: DeprecatedCall) -> Self {
            Self::Deprecated(value)
        }
    }
    impl ::core::convert::From<DestroyBlackFundsCall> for usdtCalls {
        fn from(value: DestroyBlackFundsCall) -> Self {
            Self::DestroyBlackFunds(value)
        }
    }
    impl ::core::convert::From<GetBlackListStatusCall> for usdtCalls {
        fn from(value: GetBlackListStatusCall) -> Self {
            Self::GetBlackListStatus(value)
        }
    }
    impl ::core::convert::From<GetOwnerCall> for usdtCalls {
        fn from(value: GetOwnerCall) -> Self {
            Self::GetOwner(value)
        }
    }
    impl ::core::convert::From<IsBlackListedCall> for usdtCalls {
        fn from(value: IsBlackListedCall) -> Self {
            Self::IsBlackListed(value)
        }
    }
    impl ::core::convert::From<IssueCall> for usdtCalls {
        fn from(value: IssueCall) -> Self {
            Self::Issue(value)
        }
    }
    impl ::core::convert::From<MaximumFeeCall> for usdtCalls {
        fn from(value: MaximumFeeCall) -> Self {
            Self::MaximumFee(value)
        }
    }
    impl ::core::convert::From<NameCall> for usdtCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for usdtCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for usdtCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for usdtCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for usdtCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RemoveBlackListCall> for usdtCalls {
        fn from(value: RemoveBlackListCall) -> Self {
            Self::RemoveBlackList(value)
        }
    }
    impl ::core::convert::From<SetParamsCall> for usdtCalls {
        fn from(value: SetParamsCall) -> Self {
            Self::SetParams(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for usdtCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for usdtCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for usdtCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for usdtCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for usdtCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for usdtCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpgradedAddressCall> for usdtCalls {
        fn from(value: UpgradedAddressCall) -> Self {
            Self::UpgradedAddress(value)
        }
    }
    ///Container type for all return fields from the `MAX_UINT` function with signature `MAX_UINT()` and selector `0xe5b5019a`
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
    pub struct MaxUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_totalSupply` function with signature `_totalSupply()` and selector `0x3eaaf86b`
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
    pub struct _TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn {
        pub remaining: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `allowed` function with signature `allowed(address,address)` and selector `0x5c658165`
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
    pub struct AllowedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balances` function with signature `balances(address)` and selector `0x27e235e3`
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
    pub struct BalancesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `basisPointsRate` function with signature `basisPointsRate()` and selector `0xdd644f72`
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
    pub struct BasisPointsRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deprecated` function with signature `deprecated()` and selector `0x0e136b19`
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
    pub struct DeprecatedReturn(pub bool);
    ///Container type for all return fields from the `getBlackListStatus` function with signature `getBlackListStatus(address)` and selector `0x59bf1abe`
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
    pub struct GetBlackListStatusReturn(pub bool);
    ///Container type for all return fields from the `getOwner` function with signature `getOwner()` and selector `0x893d20e8`
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
    pub struct GetOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isBlackListed` function with signature `isBlackListed(address)` and selector `0xe47d6060`
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
    pub struct IsBlackListedReturn(pub bool);
    ///Container type for all return fields from the `maximumFee` function with signature `maximumFee()` and selector `0x35390714`
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
    pub struct MaximumFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `upgradedAddress` function with signature `upgradedAddress()` and selector `0x26976e3f`
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
    pub struct UpgradedAddressReturn(pub ::ethers::core::types::Address);
}
