pub use uni_v3_swap_router_2::*;
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
pub mod uni_v3_swap_router_2 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_factoryV2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"factoryV3\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_positionManager\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_WETH9\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"approveMax\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"approveMaxMinusOne\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"approveZeroThenMax\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"approveZeroThenMaxMinusOne\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"callPositionManager\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"result\",\"type\":\"bytes\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"paths\",\"type\":\"bytes[]\"},{\"internalType\":\"uint128[]\",\"name\":\"amounts\",\"type\":\"uint128[]\"},{\"internalType\":\"uint24\",\"name\":\"maximumTickDivergence\",\"type\":\"uint24\"},{\"internalType\":\"uint32\",\"name\":\"secondsAgo\",\"type\":\"uint32\"}],\"name\":\"checkOracleSlippage\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"uint24\",\"name\":\"maximumTickDivergence\",\"type\":\"uint24\"},{\"internalType\":\"uint32\",\"name\":\"secondsAgo\",\"type\":\"uint32\"}],\"name\":\"checkOracleSlippage\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"}],\"internalType\":\"struct IV3SwapRouter.ExactInputParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\"}],\"internalType\":\"struct IV3SwapRouter.ExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\"}],\"internalType\":\"struct IV3SwapRouter.ExactOutputParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\"},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\"}],\"internalType\":\"struct IV3SwapRouter.ExactOutputSingleParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"exactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"factoryV2\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"getApprovalType\",\"outputs\":[{\"internalType\":\"enum IApproveAndCall.ApprovalType\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount0Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Min\",\"type\":\"uint256\"}],\"internalType\":\"struct IApproveAndCall.IncreaseLiquidityParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"increaseLiquidity\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"result\",\"type\":\"bytes\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\"},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\"},{\"internalType\":\"int24\",\"name\":\"tickLower\",\"type\":\"int24\"},{\"internalType\":\"int24\",\"name\":\"tickUpper\",\"type\":\"int24\"},{\"internalType\":\"uint256\",\"name\":\"amount0Min\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount1Min\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct IApproveAndCall.MintParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"result\",\"type\":\"bytes\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"previousBlockhash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\"}],\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"\",\"type\":\"bytes[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\"}],\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"\",\"type\":\"bytes[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\"}],\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"positionManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"pull\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"refundETH\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermitAllowed\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermitAllowedIfNecessary\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"selfPermitIfNecessary\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\"},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"name\":\"swapExactTokensForTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\"},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"name\":\"swapTokensForExactTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"sweepToken\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"}],\"name\":\"sweepToken\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\"}],\"name\":\"sweepTokenWithFee\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\"}],\"name\":\"sweepTokenWithFee\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0Delta\",\"type\":\"int256\"},{\"internalType\":\"int256\",\"name\":\"amount1Delta\",\"type\":\"int256\"},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\"}],\"name\":\"uniswapV3SwapCallback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"unwrapWETH9\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"}],\"name\":\"unwrapWETH9\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\"}],\"name\":\"unwrapWETH9WithFee\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\"}],\"name\":\"unwrapWETH9WithFee\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"wrapETH\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]";
    ///The parsed JSON ABI of the contract.
    pub static SWAPROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SwapRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SwapRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SwapRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SwapRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SwapRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SwapRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SwapRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SWAPROUTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveMax` (0x571ac8b0) function
        pub fn approve_max(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 26, 200, 176], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveMaxMinusOne` (0xcab372ce) function
        pub fn approve_max_minus_one(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 179, 114, 206], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveZeroThenMax` (0x639d71a9) function
        pub fn approve_zero_then_max(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 157, 113, 169], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveZeroThenMaxMinusOne` (0xab3fdd50) function
        pub fn approve_zero_then_max_minus_one(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 63, 221, 80], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callPositionManager` (0xb3a2af13) function
        pub fn call_position_manager(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([179, 162, 175, 19], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkOracleSlippage` (0xefdeed8e) function
        pub fn check_oracle_slippage_with_paths_and_amounts_and_maximum_tick_divergence(
            &self,
            paths: ::std::vec::Vec<::ethers::core::types::Bytes>,
            amounts: ::std::vec::Vec<u128>,
            maximum_tick_divergence: u32,
            seconds_ago: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [239, 222, 237, 142],
                    (paths, amounts, maximum_tick_divergence, seconds_ago),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkOracleSlippage` (0xf25801a7) function
        pub fn check_oracle_slippage(
            &self,
            path: ::ethers::core::types::Bytes,
            maximum_tick_divergence: u32,
            seconds_ago: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [242, 88, 1, 167],
                    (path, maximum_tick_divergence, seconds_ago),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInput` (0xb858183f) function
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 88, 24, 63], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInputSingle` (0x04e45aaf) function
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 228, 90, 175], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactOutput` (0x09b81346) function
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([9, 184, 19, 70], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactOutputSingle` (0x5023b4df) function
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 35, 180, 223], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factoryV2` (0x68e0d4e1) function
        pub fn factory_v2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([104, 224, 212, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApprovalType` (0xdee00f35) function
        pub fn get_approval_type(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([222, 224, 15, 53], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseLiquidity` (0xf100b205) function
        pub fn increase_liquidity(
            &self,
            params: IncreaseLiquidityParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([241, 0, 178, 5], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x11ed56c9) function
        pub fn mint(
            &self,
            params: MintParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([17, 237, 86, 201], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0x1f0464d1) function
        pub fn multicall_with_previous_blockhash(
            &self,
            previous_blockhash: [u8; 32],
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([31, 4, 100, 209], (previous_blockhash, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0x5ae401dc) function
        pub fn multicall_with_deadline(
            &self,
            deadline: ::ethers::core::types::U256,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([90, 228, 1, 220], (deadline, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positionManager` (0x791b98bc) function
        pub fn position_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([121, 27, 152, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pull` (0xf2d5d56b) function
        pub fn pull(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 213, 213, 107], (token, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundETH` (0x12210e8a) function
        pub fn refund_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermit` (0xf3995c67) function
        pub fn self_permit(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowed` (0x4659a494) function
        pub fn self_permit_allowed(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function
        pub fn self_permit_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokens` (0x472b43f3) function
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 43, 67, 243], (amount_in, amount_out_min, path, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokensForExactTokens` (0x42712a67) function
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ::ethers::core::types::U256,
            amount_in_max: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 113, 42, 103], (amount_out, amount_in_max, path, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepToken` (0xdf2ab5bb) function
        pub fn sweep_token_with_token_and_amount_minimum(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepToken` (0xe90a182f) function
        pub fn sweep_token(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 10, 24, 47], (token, amount_minimum))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepTokenWithFee` (0x3068c554) function
        pub fn sweep_token_with_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 104, 197, 84],
                    (token, amount_minimum, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepTokenWithFee` (0xe0e189a0) function
        pub fn sweep_token_with_fee_with_token_and_amount_minimum_and_recipient_and_fee_bips(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 225, 137, 160],
                    (token, amount_minimum, recipient, fee_bips, fee_recipient),
                )
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
        ///Calls the contract's `unwrapWETH9` (0x49404b7c) function
        pub fn unwrap_weth_9_with_recipient(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9` (0x49616997) function
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 97, 105, 151], amount_minimum)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9WithFee` (0x9b2c0a37) function
        pub fn unwrap_weth_9_with_fee_with_amount_minimum_and_recipient_and_fee_bips(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 44, 10, 55],
                    (amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9WithFee` (0xd4ef38de) function
        pub fn unwrap_weth9_with_fee(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            fee_bips: ::ethers::core::types::U256,
            fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [212, 239, 56, 222],
                    (amount_minimum, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrapETH` (0x1c58db4f) function
        pub fn wrap_eth(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 88, 219, 79], value)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for SwapRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    ///Container type for all input parameters for the `approveMax` function with signature `approveMax(address)` and selector `0x571ac8b0`
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
    #[ethcall(name = "approveMax", abi = "approveMax(address)")]
    pub struct ApproveMaxCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approveMaxMinusOne` function with signature `approveMaxMinusOne(address)` and selector `0xcab372ce`
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
    #[ethcall(name = "approveMaxMinusOne", abi = "approveMaxMinusOne(address)")]
    pub struct ApproveMaxMinusOneCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approveZeroThenMax` function with signature `approveZeroThenMax(address)` and selector `0x639d71a9`
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
    #[ethcall(name = "approveZeroThenMax", abi = "approveZeroThenMax(address)")]
    pub struct ApproveZeroThenMaxCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approveZeroThenMaxMinusOne` function with signature `approveZeroThenMaxMinusOne(address)` and selector `0xab3fdd50`
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
        name = "approveZeroThenMaxMinusOne",
        abi = "approveZeroThenMaxMinusOne(address)"
    )]
    pub struct ApproveZeroThenMaxMinusOneCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `callPositionManager` function with signature `callPositionManager(bytes)` and selector `0xb3a2af13`
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
    #[ethcall(name = "callPositionManager", abi = "callPositionManager(bytes)")]
    pub struct CallPositionManagerCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `checkOracleSlippage` function with signature `checkOracleSlippage(bytes[],uint128[],uint24,uint32)` and selector `0xefdeed8e`
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
        name = "checkOracleSlippage",
        abi = "checkOracleSlippage(bytes[],uint128[],uint24,uint32)"
    )]
    pub struct CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergenceCall {
        pub paths: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub amounts: ::std::vec::Vec<u128>,
        pub maximum_tick_divergence: u32,
        pub seconds_ago: u32,
    }
    ///Container type for all input parameters for the `checkOracleSlippage` function with signature `checkOracleSlippage(bytes,uint24,uint32)` and selector `0xf25801a7`
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
        name = "checkOracleSlippage",
        abi = "checkOracleSlippage(bytes,uint24,uint32)"
    )]
    pub struct CheckOracleSlippageCall {
        pub path: ::ethers::core::types::Bytes,
        pub maximum_tick_divergence: u32,
        pub seconds_ago: u32,
    }
    ///Container type for all input parameters for the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256))` and selector `0xb858183f`
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
        name = "exactInput",
        abi = "exactInput((bytes,address,uint256,uint256))"
    )]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    ///Container type for all input parameters for the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint160))` and selector `0x04e45aaf`
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
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    ///Container type for all input parameters for the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256))` and selector `0x09b81346`
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
        name = "exactOutput",
        abi = "exactOutput((bytes,address,uint256,uint256))"
    )]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }
    ///Container type for all input parameters for the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint160))` and selector `0x5023b4df`
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
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `factoryV2` function with signature `factoryV2()` and selector `0x68e0d4e1`
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
    #[ethcall(name = "factoryV2", abi = "factoryV2()")]
    pub struct FactoryV2Call;
    ///Container type for all input parameters for the `getApprovalType` function with signature `getApprovalType(address,uint256)` and selector `0xdee00f35`
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
    #[ethcall(name = "getApprovalType", abi = "getApprovalType(address,uint256)")]
    pub struct GetApprovalTypeCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `increaseLiquidity` function with signature `increaseLiquidity((address,address,uint256,uint256,uint256))` and selector `0xf100b205`
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
        name = "increaseLiquidity",
        abi = "increaseLiquidity((address,address,uint256,uint256,uint256))"
    )]
    pub struct IncreaseLiquidityCall {
        pub params: IncreaseLiquidityParams,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint((address,address,uint24,int24,int24,uint256,uint256,address))` and selector `0x11ed56c9`
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
        name = "mint",
        abi = "mint((address,address,uint24,int24,int24,uint256,uint256,address))"
    )]
    pub struct MintCall {
        pub params: MintParams,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes32,bytes[])` and selector `0x1f0464d1`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes32,bytes[])")]
    pub struct MulticallWithPreviousBlockhashCall {
        pub previous_blockhash: [u8; 32],
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(uint256,bytes[])` and selector `0x5ae401dc`
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
    #[ethcall(name = "multicall", abi = "multicall(uint256,bytes[])")]
    pub struct MulticallWithDeadlineCall {
        pub deadline: ::ethers::core::types::U256,
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `positionManager` function with signature `positionManager()` and selector `0x791b98bc`
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
    #[ethcall(name = "positionManager", abi = "positionManager()")]
    pub struct PositionManagerCall;
    ///Container type for all input parameters for the `pull` function with signature `pull(address,uint256)` and selector `0xf2d5d56b`
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
    #[ethcall(name = "pull", abi = "pull(address,uint256)")]
    pub struct PullCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `refundETH` function with signature `refundETH()` and selector `0x12210e8a`
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
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
    ///Container type for all input parameters for the `selfPermit` function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3995c67`
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
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowed` function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x4659a494`
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
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowedIfNecessary` function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xa4a78f0c`
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
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitIfNecessary` function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc2e3140a`
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
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address)` and selector `0x472b43f3`
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address)` and selector `0x42712a67`
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
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(uint256,uint256,address[],address)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_max: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sweepToken` function with signature `sweepToken(address,uint256,address)` and selector `0xdf2ab5bb`
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
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenWithTokenAndAmountMinimumCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sweepToken` function with signature `sweepToken(address,uint256)` and selector `0xe90a182f`
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
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256)")]
    pub struct SweepTokenCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sweepTokenWithFee` function with signature `sweepTokenWithFee(address,uint256,uint256,address)` and selector `0x3068c554`
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
        name = "sweepTokenWithFee",
        abi = "sweepTokenWithFee(address,uint256,uint256,address)"
    )]
    pub struct SweepTokenWithFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sweepTokenWithFee` function with signature `sweepTokenWithFee(address,uint256,address,uint256,address)` and selector `0xe0e189a0`
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
        name = "sweepTokenWithFee",
        abi = "sweepTokenWithFee(address,uint256,address,uint256,address)"
    )]
    pub struct SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBipsCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256,address)` and selector `0x49404b7c`
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
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWeth9WithRecipientCall {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256)` and selector `0x49616997`
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
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unwrapWETH9WithFee` function with signature `unwrapWETH9WithFee(uint256,address,uint256,address)` and selector `0x9b2c0a37`
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
        name = "unwrapWETH9WithFee",
        abi = "unwrapWETH9WithFee(uint256,address,uint256,address)"
    )]
    pub struct UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBipsCall {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH9WithFee` function with signature `unwrapWETH9WithFee(uint256,uint256,address)` and selector `0xd4ef38de`
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
        name = "unwrapWETH9WithFee",
        abi = "unwrapWETH9WithFee(uint256,uint256,address)"
    )]
    pub struct UnwrapWETH9WithFeeCall {
        pub amount_minimum: ::ethers::core::types::U256,
        pub fee_bips: ::ethers::core::types::U256,
        pub fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `wrapETH` function with signature `wrapETH(uint256)` and selector `0x1c58db4f`
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
    #[ethcall(name = "wrapETH", abi = "wrapETH(uint256)")]
    pub struct WrapETHCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SwapRouterCalls {
        Weth9(Weth9Call),
        ApproveMax(ApproveMaxCall),
        ApproveMaxMinusOne(ApproveMaxMinusOneCall),
        ApproveZeroThenMax(ApproveZeroThenMaxCall),
        ApproveZeroThenMaxMinusOne(ApproveZeroThenMaxMinusOneCall),
        CallPositionManager(CallPositionManagerCall),
        CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergence(
            CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergenceCall,
        ),
        CheckOracleSlippage(CheckOracleSlippageCall),
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        Factory(FactoryCall),
        FactoryV2(FactoryV2Call),
        GetApprovalType(GetApprovalTypeCall),
        IncreaseLiquidity(IncreaseLiquidityCall),
        Mint(MintCall),
        MulticallWithPreviousBlockhash(MulticallWithPreviousBlockhashCall),
        MulticallWithDeadline(MulticallWithDeadlineCall),
        Multicall(MulticallCall),
        PositionManager(PositionManagerCall),
        Pull(PullCall),
        RefundETH(RefundETHCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
        SweepTokenWithTokenAndAmountMinimum(SweepTokenWithTokenAndAmountMinimumCall),
        SweepToken(SweepTokenCall),
        SweepTokenWithFee(SweepTokenWithFeeCall),
        SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBips(
            SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBipsCall,
        ),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
        UnwrapWeth9WithRecipient(UnwrapWeth9WithRecipientCall),
        UnwrapWETH9(UnwrapWETH9Call),
        UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBips(
            UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBipsCall,
        ),
        UnwrapWETH9WithFee(UnwrapWETH9WithFeeCall),
        WrapETH(WrapETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for SwapRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <ApproveMaxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveMax(decoded));
            }
            if let Ok(decoded) =
                <ApproveMaxMinusOneCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproveMaxMinusOne(decoded));
            }
            if let Ok(decoded) =
                <ApproveZeroThenMaxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproveZeroThenMax(decoded));
            }
            if let Ok(decoded) =
                <ApproveZeroThenMaxMinusOneCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproveZeroThenMaxMinusOne(decoded));
            }
            if let Ok(decoded) =
                <CallPositionManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CallPositionManager(decoded));
            }
            if let Ok(decoded)
                = <CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergenceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergence(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <CheckOracleSlippageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckOracleSlippage(decoded));
            }
            if let Ok(decoded) = <ExactInputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExactInput(decoded));
            }
            if let Ok(decoded) =
                <ExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExactInputSingle(decoded));
            }
            if let Ok(decoded) = <ExactOutputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExactOutput(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputSingleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExactOutputSingle(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <FactoryV2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FactoryV2(decoded));
            }
            if let Ok(decoded) =
                <GetApprovalTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetApprovalType(decoded));
            }
            if let Ok(decoded) =
                <IncreaseLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncreaseLiquidity(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) =
                <MulticallWithPreviousBlockhashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MulticallWithPreviousBlockhash(decoded));
            }
            if let Ok(decoded) =
                <MulticallWithDeadlineCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MulticallWithDeadline(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) =
                <PositionManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PositionManager(decoded));
            }
            if let Ok(decoded) = <PullCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pull(decoded));
            }
            if let Ok(decoded) = <RefundETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RefundETH(decoded));
            }
            if let Ok(decoded) = <SelfPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfPermit(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensForExactTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapTokensForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenWithTokenAndAmountMinimumCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SweepTokenWithTokenAndAmountMinimum(decoded));
            }
            if let Ok(decoded) = <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenWithFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SweepTokenWithFee(decoded));
            }
            if let Ok(decoded)
                = <SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBipsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBips(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWeth9WithRecipientCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnwrapWeth9WithRecipient(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            if let Ok(decoded)
                = <UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBipsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBips(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <UnwrapWETH9WithFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnwrapWETH9WithFee(decoded));
            }
            if let Ok(decoded) = <WrapETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrapETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveMax(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveMaxMinusOne(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveZeroThenMax(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveZeroThenMaxMinusOne(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallPositionManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergence(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckOracleSlippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExactInput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactInputSingle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactOutput(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactOutputSingle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FactoryV2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApprovalType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulticallWithPreviousBlockhash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulticallWithDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PositionManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pull(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermitAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokensForExactTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepTokenWithTokenAndAmountMinimum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepTokenWithFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBips(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWeth9WithRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBips(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9WithFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SwapRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveMaxMinusOne(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveZeroThenMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveZeroThenMaxMinusOne(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallPositionManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergence(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckOracleSlippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactOutputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::FactoryV2(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApprovalType(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulticallWithPreviousBlockhash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MulticallWithDeadline(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pull(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermitIfNecessary(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactTokensForTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapTokensForExactTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepTokenWithTokenAndAmountMinimum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepTokenWithFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBips(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UniswapV3SwapCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWeth9WithRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBips(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnwrapWETH9WithFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for SwapRouterCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<ApproveMaxCall> for SwapRouterCalls {
        fn from(value: ApproveMaxCall) -> Self {
            Self::ApproveMax(value)
        }
    }
    impl ::core::convert::From<ApproveMaxMinusOneCall> for SwapRouterCalls {
        fn from(value: ApproveMaxMinusOneCall) -> Self {
            Self::ApproveMaxMinusOne(value)
        }
    }
    impl ::core::convert::From<ApproveZeroThenMaxCall> for SwapRouterCalls {
        fn from(value: ApproveZeroThenMaxCall) -> Self {
            Self::ApproveZeroThenMax(value)
        }
    }
    impl ::core::convert::From<ApproveZeroThenMaxMinusOneCall> for SwapRouterCalls {
        fn from(value: ApproveZeroThenMaxMinusOneCall) -> Self {
            Self::ApproveZeroThenMaxMinusOne(value)
        }
    }
    impl ::core::convert::From<CallPositionManagerCall> for SwapRouterCalls {
        fn from(value: CallPositionManagerCall) -> Self {
            Self::CallPositionManager(value)
        }
    }
    impl ::core::convert::From<CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergenceCall>
        for SwapRouterCalls
    {
        fn from(value: CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergenceCall) -> Self {
            Self::CheckOracleSlippageWithPathsAndAmountsAndMaximumTickDivergence(value)
        }
    }
    impl ::core::convert::From<CheckOracleSlippageCall> for SwapRouterCalls {
        fn from(value: CheckOracleSlippageCall) -> Self {
            Self::CheckOracleSlippage(value)
        }
    }
    impl ::core::convert::From<ExactInputCall> for SwapRouterCalls {
        fn from(value: ExactInputCall) -> Self {
            Self::ExactInput(value)
        }
    }
    impl ::core::convert::From<ExactInputSingleCall> for SwapRouterCalls {
        fn from(value: ExactInputSingleCall) -> Self {
            Self::ExactInputSingle(value)
        }
    }
    impl ::core::convert::From<ExactOutputCall> for SwapRouterCalls {
        fn from(value: ExactOutputCall) -> Self {
            Self::ExactOutput(value)
        }
    }
    impl ::core::convert::From<ExactOutputSingleCall> for SwapRouterCalls {
        fn from(value: ExactOutputSingleCall) -> Self {
            Self::ExactOutputSingle(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for SwapRouterCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<FactoryV2Call> for SwapRouterCalls {
        fn from(value: FactoryV2Call) -> Self {
            Self::FactoryV2(value)
        }
    }
    impl ::core::convert::From<GetApprovalTypeCall> for SwapRouterCalls {
        fn from(value: GetApprovalTypeCall) -> Self {
            Self::GetApprovalType(value)
        }
    }
    impl ::core::convert::From<IncreaseLiquidityCall> for SwapRouterCalls {
        fn from(value: IncreaseLiquidityCall) -> Self {
            Self::IncreaseLiquidity(value)
        }
    }
    impl ::core::convert::From<MintCall> for SwapRouterCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MulticallWithPreviousBlockhashCall> for SwapRouterCalls {
        fn from(value: MulticallWithPreviousBlockhashCall) -> Self {
            Self::MulticallWithPreviousBlockhash(value)
        }
    }
    impl ::core::convert::From<MulticallWithDeadlineCall> for SwapRouterCalls {
        fn from(value: MulticallWithDeadlineCall) -> Self {
            Self::MulticallWithDeadline(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for SwapRouterCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<PositionManagerCall> for SwapRouterCalls {
        fn from(value: PositionManagerCall) -> Self {
            Self::PositionManager(value)
        }
    }
    impl ::core::convert::From<PullCall> for SwapRouterCalls {
        fn from(value: PullCall) -> Self {
            Self::Pull(value)
        }
    }
    impl ::core::convert::From<RefundETHCall> for SwapRouterCalls {
        fn from(value: RefundETHCall) -> Self {
            Self::RefundETH(value)
        }
    }
    impl ::core::convert::From<SelfPermitCall> for SwapRouterCalls {
        fn from(value: SelfPermitCall) -> Self {
            Self::SelfPermit(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedCall> for SwapRouterCalls {
        fn from(value: SelfPermitAllowedCall) -> Self {
            Self::SelfPermitAllowed(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedIfNecessaryCall> for SwapRouterCalls {
        fn from(value: SelfPermitAllowedIfNecessaryCall) -> Self {
            Self::SelfPermitAllowedIfNecessary(value)
        }
    }
    impl ::core::convert::From<SelfPermitIfNecessaryCall> for SwapRouterCalls {
        fn from(value: SelfPermitIfNecessaryCall) -> Self {
            Self::SelfPermitIfNecessary(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall> for SwapRouterCalls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<SwapTokensForExactTokensCall> for SwapRouterCalls {
        fn from(value: SwapTokensForExactTokensCall) -> Self {
            Self::SwapTokensForExactTokens(value)
        }
    }
    impl ::core::convert::From<SweepTokenWithTokenAndAmountMinimumCall> for SwapRouterCalls {
        fn from(value: SweepTokenWithTokenAndAmountMinimumCall) -> Self {
            Self::SweepTokenWithTokenAndAmountMinimum(value)
        }
    }
    impl ::core::convert::From<SweepTokenCall> for SwapRouterCalls {
        fn from(value: SweepTokenCall) -> Self {
            Self::SweepToken(value)
        }
    }
    impl ::core::convert::From<SweepTokenWithFeeCall> for SwapRouterCalls {
        fn from(value: SweepTokenWithFeeCall) -> Self {
            Self::SweepTokenWithFee(value)
        }
    }
    impl ::core::convert::From<SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBipsCall>
        for SwapRouterCalls
    {
        fn from(
            value: SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBipsCall,
        ) -> Self {
            Self::SweepTokenWithFeeWithTokenAndAmountMinimumAndRecipientAndFeeBips(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for SwapRouterCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    impl ::core::convert::From<UnwrapWeth9WithRecipientCall> for SwapRouterCalls {
        fn from(value: UnwrapWeth9WithRecipientCall) -> Self {
            Self::UnwrapWeth9WithRecipient(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for SwapRouterCalls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
        }
    }
    impl ::core::convert::From<UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBipsCall>
        for SwapRouterCalls
    {
        fn from(value: UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBipsCall) -> Self {
            Self::UnwrapWeth9WithFeeWithAmountMinimumAndRecipientAndFeeBips(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9WithFeeCall> for SwapRouterCalls {
        fn from(value: UnwrapWETH9WithFeeCall) -> Self {
            Self::UnwrapWETH9WithFee(value)
        }
    }
    impl ::core::convert::From<WrapETHCall> for SwapRouterCalls {
        fn from(value: WrapETHCall) -> Self {
            Self::WrapETH(value)
        }
    }
    ///Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    pub struct Weth9Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `callPositionManager` function with signature `callPositionManager(bytes)` and selector `0xb3a2af13`
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
    pub struct CallPositionManagerReturn {
        pub result: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256))` and selector `0xb858183f`
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
    pub struct ExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint160))` and selector `0x04e45aaf`
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
    pub struct ExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactOutput` function with signature `exactOutput((bytes,address,uint256,uint256))` and selector `0x09b81346`
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
    pub struct ExactOutputReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactOutputSingle` function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint160))` and selector `0x5023b4df`
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
    pub struct ExactOutputSingleReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `factoryV2` function with signature `factoryV2()` and selector `0x68e0d4e1`
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
    pub struct FactoryV2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getApprovalType` function with signature `getApprovalType(address,uint256)` and selector `0xdee00f35`
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
    pub struct GetApprovalTypeReturn(pub u8);
    ///Container type for all return fields from the `increaseLiquidity` function with signature `increaseLiquidity((address,address,uint256,uint256,uint256))` and selector `0xf100b205`
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
    pub struct IncreaseLiquidityReturn {
        pub result: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `mint` function with signature `mint((address,address,uint24,int24,int24,uint256,uint256,address))` and selector `0x11ed56c9`
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
    pub struct MintReturn {
        pub result: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes32,bytes[])` and selector `0x1f0464d1`
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
    pub struct MulticallWithPreviousBlockhashReturn(
        pub ::std::vec::Vec<::ethers::core::types::Bytes>,
    );
    ///Container type for all return fields from the `multicall` function with signature `multicall(uint256,bytes[])` and selector `0x5ae401dc`
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
    pub struct MulticallWithDeadlineReturn(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `positionManager` function with signature `positionManager()` and selector `0x791b98bc`
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
    pub struct PositionManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address)` and selector `0x472b43f3`
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
    pub struct SwapExactTokensForTokensReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapTokensForExactTokens` function with signature `swapTokensForExactTokens(uint256,uint256,address[],address)` and selector `0x42712a67`
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
    pub struct SwapTokensForExactTokensReturn {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///`IncreaseLiquidityParams(address,address,uint256,uint256,uint256)`
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
    pub struct IncreaseLiquidityParams {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub amount_0_min: ::ethers::core::types::U256,
        pub amount_1_min: ::ethers::core::types::U256,
    }
    ///`MintParams(address,address,uint24,int24,int24,uint256,uint256,address)`
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
    pub struct MintParams {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_min: ::ethers::core::types::U256,
        pub amount_1_min: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///`ExactInputParams(bytes,address,uint256,uint256)`
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
    pub struct ExactInputParams {
        pub path: ::ethers::core::types::Bytes,
        pub recipient: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_minimum: ::ethers::core::types::U256,
    }
    ///`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint160)`
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
    pub struct ExactInputSingleParams {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub recipient: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_minimum: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///`ExactOutputParams(bytes,address,uint256,uint256)`
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
    pub struct ExactOutputParams {
        pub path: ::ethers::core::types::Bytes,
        pub recipient: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_maximum: ::ethers::core::types::U256,
    }
    ///`ExactOutputSingleParams(address,address,uint24,address,uint256,uint256,uint160)`
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
    pub struct ExactOutputSingleParams {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub recipient: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub amount_in_maximum: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
}
