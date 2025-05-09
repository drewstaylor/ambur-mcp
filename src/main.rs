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
pub mod instruction;
pub mod query_response;
use crate::contract::*;
use crate::instruction::*;
use crate::query_response::AllResponse as AllQueryResponse;

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
            description = "QueryMsg variant and its values needed for building the query as a Cosmos SDK QueryRequest"
        )]
        query_msg: QueryMsg,
    ) -> Result<CallToolResult, Error> {
        let query_req: QueryRequest<QueryMsg> = QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_json_binary(&query_msg).unwrap_or_default(),
        });
        let serialized: String = serde_json::to_string(&query_req).unwrap_or("".to_string());
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
        execute_msg: ExecuteMsg,
        #[tool(param)]
        #[schemars(
            description = "Optionally include native payment funds to be sent in the transaction. Only required for 'finish' txs if payment_token is a native token."
        )]
        payment: Option<Coin>,
    ) -> Result<CallToolResult, Error> {
        let funds: Vec<Coin> = if payment.is_some() {
            vec![payment.unwrap_or_default()]
        } else {
            vec![]
        };
        let execute_msg: CosmosMsg = WasmMsg::Execute {
            contract_addr,
            msg: to_json_binary(&execute_msg).unwrap_or_default(),
            funds,
        }
        .into();
        let serialized: String = serde_json::to_string(&execute_msg).unwrap_or("".to_string());
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
