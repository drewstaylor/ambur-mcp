use cosmwasm_std::{CosmosMsg, QueryRequest, WasmMsg, WasmQuery, to_json_binary};
use rmcp::{
    Error, ServerHandler, model::CallToolResult, model::Content, model::Implementation,
    model::ProtocolVersion, model::ServerCapabilities, model::ServerInfo, tool,
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use ambur_wl_token::{
    ExecuteMsg as ForesightExecuteMsg, 
    Extension as ForesightExtension, 
    QueryMsg as ForesightQueryMsg,
};
use archies_token::{
    ExecuteMsg as ArchiesExecuteMsg, 
    Extension as ArchiesExtension,
    QueryMsg as ArchiesQueryMsg,
};
use derpies_token::{
    ExecuteMsg as DerpiesExecuteMsg, 
    Extension as DerpiesExtension,
    QueryMsg as DerpiesQueryMsg
};
use ghouls_token::{
    ExecuteMsg as GhoulsExecuteMsg, 
    Extension as GhoulsExtension,
    QueryMsg as GhoulsQueryMsg,
};

use crate::contract::*;
use crate::execute::*;
use crate::network::*;
// XXX TODO: token_instruction::*
use crate::query::ValidatedQuery;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    ArchiesQueryMsg,
    ForesightQueryMsg,
    DerpiesQueryMsg,
    GhoulsQueryMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    ArchiesExecuteMsg,
    ForesightExecuteMsg,
    DerpiesExecuteMsg,
    GhoulsExecuteMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburTokenMcp {
    collections: Vec<AmburCollection>,
}

#[tool(tool_box)]
impl AmburTokenMcp {
    pub fn new() -> Self {
        Self {
            collections: vec![
                AmburCollection {
                    name: "Archies".to_string(),
                    description: ARCHIES_DESCR.to_string(),
                    contract_addresses: CollectionContract {
                        token: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: TOKEN_ARCHIES_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: TOKEN_ARCHIES_CONSTANTINE.to_string(),
                            },
                        ],
                        minter: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: MINTER_ARCHIES_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: MINTER_ARCHIES_CONSTANTINE.to_string(),
                            },
                        ],
                    },
                },
                AmburCollection {
                    name: "The Foresight Ticket".to_string(),
                    description: FORESIGHT_DESCR.to_string(),
                    contract_addresses: CollectionContract {
                        token: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: TOKEN_FORESIGHT_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: TOKEN_FORESIGHT_CONSTANTINE.to_string(),
                            },
                        ],
                        minter: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: MINTER_FORESIGHT_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: MINTER_FORESIGHT_CONSTANTINE.to_string(),
                            },
                        ],
                    },
                },
                AmburCollection {
                    name: "Derpies".to_string(),
                    description: DERPIES_DESCR.to_string(),
                    contract_addresses: CollectionContract {
                        token: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: TOKEN_DERPIES_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: TOKEN_DERPIES_CONSTANTINE.to_string(),
                            },
                        ],
                        minter: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: MINTER_DERPIES_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: MINTER_DERPIES_CONSTANTINE.to_string(),
                            },
                        ],
                    },
                },
                AmburCollection {
                    name: "Ghouls".to_string(),
                    description: GHOULS_DESCR.to_string(),
                    contract_addresses: CollectionContract {
                        token: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: TOKEN_GHOULS_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: TOKEN_GHOULS_CONSTANTINE.to_string(),
                            },
                        ],
                        minter: [
                            AmburContract {
                                network: ArchwayNetwork::Mainnet,
                                contract_address: MINTER_GHOULS_MAINNET.to_string(),
                            },
                            AmburContract {
                                network: ArchwayNetwork::Constantine,
                                contract_address: MINTER_GHOULS_CONSTANTINE.to_string(),
                            },
                        ],
                    },
                },
            ],
        }
    }

    // Query entry point tools
    #[tool(description = "XXX TODO")]
    async fn list_query_entry_points(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
    ) -> Result<CallToolResult, Error> {
        let schema = match nft.to_lowercase().as_str() {
            "archies" => Some(schema_for!(ArchiesQueryMsg<ArchiesExtension>)),
            "derpies" => Some(schema_for!(DerpiesQueryMsg<DerpiesExtension>)),
            "ghouls" => Some(schema_for!(GhoulsQueryMsg<GhoulsExtension>)),
            "foresight" => Some(schema_for!(ForesightQueryMsg<ForesightExtension>)),
            "the foresight ticket" => Some(schema_for!(ForesightQueryMsg<ForesightExtension>)),
            _ => None,
        };
        if schema.is_none() {
            let err_msg = "Unrecognized NFT collection name ".to_string() + &nft; 
            return Ok(CallToolResult::error(vec![Content::text(&err_msg)]));
        }
        let schema = schema.unwrap();
        let serialized: String = serde_json::to_string(&schema).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    #[tool(description = "XXX TODO")]
    async fn build_query_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "contract address of cw721 token"
        )]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "JSON stringified QueryMsg variant needed for building the query as a Cosmos SDK QueryRequest"
        )]
        query_msg: String,
    ) -> Result<CallToolResult, Error> {
        let deserialized: QueryMsg = serde_json::from_str(query_msg.as_str()).unwrap();
        let query_req: QueryRequest<QueryMsg> = QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_json_binary(&deserialized).unwrap_or_default(),
        });
        let serialized_query_req = serde_json::to_string(&query_req);
        if serialized_query_req.is_err() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error wrapping QueryMsg as QueryRequest",
            )]));
        }
        let valid_query = ValidatedQuery {
            query_msg,
            query_request: serialized_query_req.unwrap_or_default(),
        };
        let serialized: String = serde_json::to_string(&valid_query).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    // Execute entry point tools
    #[tool(description = "XXX TODO")]
    async fn list_tx_entry_points(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
    ) -> Result<CallToolResult, Error> {
        let schema = match nft.to_lowercase().as_str() {
            "archies" => Some(schema_for!(ArchiesExecuteMsg)),
            "derpies" => Some(schema_for!(DerpiesExecuteMsg)),
            "ghouls" => Some(schema_for!(GhoulsExecuteMsg)),
            "foresight" => Some(schema_for!(ForesightExecuteMsg)),
            "the foresight ticket" => Some(schema_for!(ForesightExecuteMsg)),
            _ => None,
        };
        if schema.is_none() {
            let err_msg = "Unrecognized NFT collection name ".to_string() + &nft; 
            return Ok(CallToolResult::error(vec![Content::text(&err_msg)]));
        }
        let schema = schema.unwrap();
        let serialized: String = serde_json::to_string(&schema).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    #[tool(description = "XXX TODO")]
    async fn build_execute_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "XXX TODO"
        )]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "XXX TODO"
        )]
        execute_msg: String,
    ) -> Result<CallToolResult, Error> {
        let deserialized: ExecuteMsg = serde_json::from_str(execute_msg.as_str()).unwrap();
        let cosmos_msg: CosmosMsg = WasmMsg::Execute {
            contract_addr,
            msg: to_json_binary(&deserialized).unwrap_or_default(),
            funds: vec![],
        }
        .into();
        let serialized_cosmos_msg = serde_json::to_string(&cosmos_msg);
        if serialized_cosmos_msg.is_err() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error wrapping ExecuteMsg as CosmosMsg",
            )]));
        }
        let valid_execute = ValidatedExecute {
            execute_msg,
            cosmos_msg: serialized_cosmos_msg.unwrap_or_default(),
        };
        let serialized: String = serde_json::to_string(&valid_execute).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }
}

impl Default for AmburTokenMcp {
    fn default() -> Self {
        Self::new()
    }
}

#[tool(tool_box)]
impl ServerHandler for AmburTokenMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("XXX TODO: server system prompt goes here".to_string()),
        }
    }
}
