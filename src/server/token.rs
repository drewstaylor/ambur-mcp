pub use ambur_wl_token::{
    ExecuteMsg as ForesightExecuteMsg, Extension as ForesightExtension,
    QueryMsg as ForesightQueryMsg,
};
pub use archies_token::{
    ExecuteMsg as ArchiesExecuteMsg, Extension as ArchiesExtension, QueryMsg as ArchiesQueryMsg,
};
pub use cosmwasm_std::{CosmosMsg, QueryRequest, WasmMsg, WasmQuery, to_json_binary};
pub use derpies_token::{
    ExecuteMsg as DerpiesExecuteMsg, Extension as DerpiesExtension, QueryMsg as DerpiesQueryMsg,
};
pub use ghouls_token::{
    ExecuteMsg as GhoulsExecuteMsg, Extension as GhoulsExtension, QueryMsg as GhoulsQueryMsg,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub enum TokenQueryMsg {
//     ArchiesQueryMsg,
//     ForesightQueryMsg,
//     DerpiesQueryMsg,
//     GhoulsQueryMsg,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TokenExecuteMsg {
    ArchiesExecuteMsg,
    ForesightExecuteMsg,
    DerpiesExecuteMsg,
    GhoulsExecuteMsg,
}
