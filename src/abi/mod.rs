use ethers::prelude::*;
abigen!(
    Erc20,
    "src/sandwich/abi/ierc20.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    UniswapV2Pair,
    "src/sandwich/abi/iuniswap_v2_pool.json",
    event_derives(serde::Deserialize, serde::Serialize)
);
