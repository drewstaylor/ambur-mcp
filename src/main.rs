pub mod contract;
pub mod execute;
pub mod instruction;
pub mod network;
pub mod query;
pub mod server;

use rmcp::{ServiceExt, transport::stdio};
use std::error::Error as StdError;

use crate::server::ambur::AmburMcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mcp_server = AmburMcp::new().serve(stdio()).await.inspect_err(|e| {
        println!("{e}");
    })?;

    mcp_server.waiting().await?;

    Ok(())
}