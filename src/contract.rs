use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::network::ArchwayNetwork;

// Contract addresses
pub static CONTRACT_MAINNET: &str =
    "archway13s2wjcgx4pxwq3kw66669jt8l4jm840hhhxk7ktpj7r98wc5hvnqraj6w3";
pub static CONTRACT_CONSTANTINE: &str =
    "archway1y9twsp7sf4ae8jl5huhduv0lwxa0l4g4n9lyvnvjkp648udpwh0q9vycz4";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburContract {
    pub network: ArchwayNetwork,
    pub contract_address: String,
}
