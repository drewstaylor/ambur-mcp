use cosmwasm_std::{Coin, CosmosMsg, QueryRequest, WasmMsg, WasmQuery, to_json_binary};
use philabs_cw721_marketplace::msg::{ExecuteMsg, QueryMsg};
use rmcp::{
    Error, ServerHandler, ServiceExt, model::CallToolResult, model::Content, model::Implementation,
    model::ProtocolVersion, model::ServerCapabilities, model::ServerInfo, tool, transport::stdio,
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

pub mod contract;
pub mod execute;
pub mod instruction;
pub mod query;
use crate::contract::*;
use crate::execute::*;
use crate::instruction::*;
use crate::query::{AllResponse as AllQueryResponse, ValidatedQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mcp_server = AmburMcp::new().serve(stdio()).await.inspect_err(|e| {
        println!("{e}");
    })?;

    mcp_server.waiting().await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ArchwayNetwork {
    Mainnet,
    Constantine,
    Titus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburContract {
    network: ArchwayNetwork,
    contract_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmburMcp {
    contracts: [AmburContract; 2],
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
        }
    }

    /// Network tools
    #[tool(description = LIST_CONTRACTS_DESCR)]
    async fn list_contract_deployments(&self) -> Result<CallToolResult, Error> {
        let serialized: String = serde_json::to_string(&self.contracts).unwrap_or("".to_string());
        Ok(CallToolResult::success(vec![Content::text(serialized)]))
    }

    /// Query entry point tools
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

    /// Execute entry point tools
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
    ) -> Result<CallToolResult, Error> {
        let funds: Vec<Coin> = if payment.is_some() {
            let deserialized: Coin =
                serde_json::from_str(payment.unwrap().as_str()).unwrap_or_default();
            vec![deserialized]
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
