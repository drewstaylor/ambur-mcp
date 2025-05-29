use cosmwasm_std::{Coin, CosmosMsg, QueryRequest, Uint128, WasmMsg, WasmQuery, to_json_binary};
use philabs_cw721_marketplace::msg::{ExecuteMsg, QueryMsg};
use rmcp::{
    Error, ServerHandler, model::CallToolResult, model::Content, model::Implementation,
    model::ProtocolVersion, model::ServerCapabilities, model::ServerInfo, tool,
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::contract::*;
use crate::execute::*;
use crate::instruction::*;
use crate::network::*;
use crate::query::{AllResponse as AllQueryResponse, ValidatedQuery};
use crate::server::minter::*;
use crate::server::token::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburMcp {
    contracts: [AmburContract; 2],
    collections: Vec<AmburCollection>,
}
#[tool(tool_box)]
impl AmburMcp {
    pub fn new() -> Self {
        Self {
            contracts: [
                AmburContract {
                    network: ArchwayNetwork::Mainnet,
                    contract_address: CONTRACT_MAINNET.to_string(),
                },
                AmburContract {
                    network: ArchwayNetwork::Constantine,
                    contract_address: CONTRACT_CONSTANTINE.to_string(),
                },
            ],
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

    // Contracts and collections tools
    #[tool(description = LIST_CONTRACTS_DESCR)]
    async fn list_contract_deployments(&self) -> Result<CallToolResult, Error> {
        let serialized: String = serde_json::to_string(&self.contracts).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    #[tool(description = LIST_NFT_COLLECTIONS_DESCR)]
    async fn list_nft_collections(&self) -> Result<CallToolResult, Error> {
        let serialized: String = serde_json::to_string(&self.collections).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    // Query entry point tools
    #[tool(description = LIST_QUERY_ENTRY_POINTS_DESCR)]
    async fn list_query_entry_points(&self) -> Result<CallToolResult, Error> {
        let schema = schema_for!(QueryMsg);
        let serialized: String = serde_json::to_string(&schema).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    #[tool(description = LIST_QUERY_RESPONSE_DESCR)]
    async fn list_query_responses(&self) -> Result<CallToolResult, Error> {
        let schema = schema_for!(AllQueryResponse);
        let serialized: String = serde_json::to_string(&schema).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    #[tool(description = BUILD_QUERY_MSG_DESCR)]
    async fn build_query_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "contract address of Ambur marketplace (e.g. mainnet or testnet address)"
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
    #[tool(description = LIST_TX_ENTRY_POINTS_DESCR)]
    async fn list_tx_entry_points(&self) -> Result<CallToolResult, Error> {
        let schema = schema_for!(ExecuteMsg);
        let serialized: String = serde_json::to_string(&schema).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    #[tool(description = BUILD_EXECUTE_MSG_DESCR)]
    async fn build_execute_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "contract address of Ambur marketplace (e.g. mainnet or testnet address)"
        )]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "ExecuteMsg variant and its values needed for building the transaction as a Cosmos SDK CosmosMsg"
        )]
        execute_msg: String,
        #[tool(param)]
        #[schemars(
            description = "Optionally include native payment funds to be sent in the transaction (only required for 'finish' txs if payment_token is a native token)"
        )]
        payment: Option<String>,
        #[tool(param)]
        #[schemars(
            description = "Optionally include native payment denom for funds being sent in the transaction (required for any transactions that require native denom payments; e.g. not cw20 payments)"
        )]
        payment_denom: Option<String>,
    ) -> Result<CallToolResult, Error> {
        let funds: Vec<Coin> = if payment.is_some() && payment_denom.is_some() {
            let funds = Coin {
                denom: payment_denom.unwrap_or_default(),
                amount: Uint128::from_str(payment.unwrap_or_default().as_str()).unwrap_or_default(),
            };
            vec![funds]
        } else {
            vec![]
        };
        let deserialized: ExecuteMsg = serde_json::from_str(execute_msg.as_str()).unwrap();
        let cosmos_msg: CosmosMsg = WasmMsg::Execute {
            contract_addr,
            msg: to_json_binary(&deserialized).unwrap_or_default(),
            funds,
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

    // cw721 Query entry point tools
    #[tool(description = LIST_TOKEN_QUERY_ENTRY_POINTS_DESCR)]
    async fn list_token_query_entry_points(
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
            "foresight" | "the foresight ticket" => {
                Some(schema_for!(ForesightQueryMsg<ForesightExtension>))
            }
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

    #[tool(description = BUILD_TOKEN_QUERY_MSG_DESCR)]
    async fn build_token_query_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
        #[tool(param)]
        #[schemars(description = "contract address of cw721 token")]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "JSON stringified QueryMsg variant needed for building the query as a Cosmos SDK QueryRequest"
        )]
        query_msg: String,
    ) -> Result<CallToolResult, Error> {
        let query_req: Option<QueryRequest> = match nft.to_lowercase().as_str() {
            "archies" => {
                let deserialized: ArchiesQueryMsg<ArchiesExtension> =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            "derpies" => {
                let deserialized: DerpiesQueryMsg<DerpiesExtension> =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            "ghouls" => {
                let deserialized: GhoulsQueryMsg<GhoulsExtension> =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            "foresight" | "the foresight ticket" => {
                let deserialized: ForesightQueryMsg<ForesightExtension> =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            _ => None,
        };
        if query_req.is_none() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error deserializing token QueryMsg",
            )]));
        }
        let serialized_query_req = serde_json::to_string(&query_req);
        if serialized_query_req.is_err() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error wrapping token QueryMsg as QueryRequest",
            )]));
        }
        let valid_query = ValidatedQuery {
            query_msg,
            query_request: serialized_query_req.unwrap_or_default(),
        };
        let serialized: String = serde_json::to_string(&valid_query).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    // cw721 Execute entry point tools
    #[tool(description = LIST_TOKEN_TX_ENTRY_POINTS_DESCR)]
    async fn list_token_tx_entry_points(
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
            "foresight" | "the foresight ticket" => Some(schema_for!(ForesightExecuteMsg)),
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

    #[tool(description = BUILD_TOKEN_EXECUTE_MSG_DESCR)]
    async fn build_token_execute_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
        #[tool(param)]
        #[schemars(description = "contract address of cw721 token")]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "ExecuteMsg variant and its values needed for building the transaction as a Cosmos SDK CosmosMsg"
        )]
        execute_msg: String,
    ) -> Result<CallToolResult, Error> {
        let cosmos_msg: Option<CosmosMsg> = match nft.to_lowercase().as_str() {
            "archies" => {
                let deserialized: ArchiesExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds: vec![],
                }
                .into();
                Some(cosmos_msg)
            }
            "derpies" => {
                let deserialized: DerpiesExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds: vec![],
                }
                .into();
                Some(cosmos_msg)
            }
            "ghouls" => {
                let deserialized: GhoulsExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds: vec![],
                }
                .into();
                Some(cosmos_msg)
            }
            "foresight" | "the foresight ticket" => {
                let deserialized: ForesightExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds: vec![],
                }
                .into();
                Some(cosmos_msg)
            }
            _ => None,
        };
        if cosmos_msg.is_none() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error deserializing execute_msg to ExecuteMsg",
            )]));
        }
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

    // Minter Query entry point tools
    #[tool(description = LIST_MINTER_QUERY_ENTRY_POINTS_DESCR)]
    async fn list_minter_query_entry_points(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
    ) -> Result<CallToolResult, Error> {
        let schema = match nft.to_lowercase().as_str() {
            "archies" => Some(schema_for!(ArchiesMinterQueryMsg)),
            "derpies" => Some(schema_for!(DerpiesMinterQueryMsg)),
            "ghouls" => Some(schema_for!(GhoulsMinterQueryMsg)),
            "foresight" | "the foresight ticket" => Some(schema_for!(ForesightMinterQueryMsg)),
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

    #[tool(description = BUILD_MINTER_QUERY_MSG_DESCR)]
    async fn build_minter_query_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
        #[tool(param)]
        #[schemars(description = "contract address of minter contract")]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "JSON stringified QueryMsg variant needed for building the query as a Cosmos SDK QueryRequest"
        )]
        query_msg: String,
    ) -> Result<CallToolResult, Error> {
        let query_req: Option<QueryRequest> = match nft.to_lowercase().as_str() {
            "archies" => {
                let deserialized: ArchiesMinterQueryMsg =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            "derpies" => {
                let deserialized: DerpiesMinterQueryMsg =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            "ghouls" => {
                let deserialized: GhoulsMinterQueryMsg =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            "foresight" | "the foresight ticket" => {
                let deserialized: ForesightMinterQueryMsg =
                    serde_json::from_str(query_msg.as_str()).unwrap();
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                });
                Some(query_req)
            }
            _ => None,
        };
        if query_req.is_none() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error deserializing token QueryMsg",
            )]));
        }
        let serialized_query_req = serde_json::to_string(&query_req);
        if serialized_query_req.is_err() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error wrapping token QueryMsg as QueryRequest",
            )]));
        }
        let valid_query = ValidatedQuery {
            query_msg,
            query_request: serialized_query_req.unwrap_or_default(),
        };
        let serialized: String = serde_json::to_string(&valid_query).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    // Minter Execute entry point tools
    #[tool(description = LIST_MINTER_TX_ENTRY_POINTS_DESCR)]
    async fn list_minter_tx_entry_points(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
    ) -> Result<CallToolResult, Error> {
        let schema = match nft.to_lowercase().as_str() {
            "archies" => Some(schema_for!(ArchiesMinterExecuteMsg)),
            "derpies" => Some(schema_for!(DerpiesMinterExecuteMsg)),
            "ghouls" => Some(schema_for!(GhoulsMinterExecuteMsg)),
            "foresight" | "the foresight ticket" => Some(schema_for!(ForesightMinterExecuteMsg)),
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

    #[tool(description = BUILD_MINTER_EXECUTE_MSG_DESCR)]
    async fn build_minter_execute_msg(
        &self,
        #[tool(param)]
        #[schemars(
            description = "name of the NFT collection (e.g. \"archies\", \"the foresight ticket\", \"derpies\", \"ghouls\")"
        )]
        nft: String,
        #[tool(param)]
        #[schemars(description = "contract address of minter contract")]
        contract_addr: String,
        #[tool(param)]
        #[schemars(
            description = "ExecuteMsg variant and its values needed for building the transaction as a Cosmos SDK CosmosMsg"
        )]
        execute_msg: String,
        #[tool(param)]
        #[schemars(
            description = "Optionally include native payment funds to be sent in the transaction (only required for mint)"
        )]
        payment: Option<String>,
        #[tool(param)]
        #[schemars(
            description = "Optionally include native payment denom for funds being sent in the transaction (required for any transactions that require native denom payments; e.g. not cw20 payments)"
        )]
        payment_denom: Option<String>,
    ) -> Result<CallToolResult, Error> {
        let funds: Vec<Coin> = if payment.is_some() && payment_denom.is_some() {
            let funds = Coin {
                denom: payment_denom.unwrap_or_default(),
                amount: Uint128::from_str(payment.unwrap_or_default().as_str()).unwrap_or_default(),
            };
            vec![funds]
        } else {
            vec![]
        };
        let cosmos_msg: Option<CosmosMsg> = match nft.to_lowercase().as_str() {
            "archies" => {
                let deserialized: ArchiesMinterExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds,
                }
                .into();
                Some(cosmos_msg)
            }
            "derpies" => {
                let deserialized: DerpiesMinterExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds,
                }
                .into();
                Some(cosmos_msg)
            }
            "ghouls" => {
                let deserialized: GhoulsMinterExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds,
                }
                .into();
                Some(cosmos_msg)
            }
            "foresight" | "the foresight ticket" => {
                let deserialized: ForesightMinterExecuteMsg =
                    serde_json::from_str(execute_msg.as_str()).unwrap();
                let cosmos_msg: CosmosMsg = WasmMsg::Execute {
                    contract_addr,
                    msg: to_json_binary(&deserialized).unwrap_or_default(),
                    funds,
                }
                .into();
                Some(cosmos_msg)
            }
            _ => None,
        };
        if cosmos_msg.is_none() {
            return Ok(CallToolResult::error(vec![Content::text(
                "Error deserializing execute_msg to ExecuteMsg",
            )]));
        }
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

impl Default for AmburMcp {
    fn default() -> Self {
        Self::new()
    }
}

#[tool(tool_box)]
impl ServerHandler for AmburMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(SERVER_INFO_DESCR.to_string()),
        }
    }
}
