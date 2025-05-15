pub mod contract;
pub mod execute;
pub mod instruction;
pub mod network;
pub mod query;
pub mod server;

use rmcp::{ServiceExt, transport::stdio};
use std::error::Error as StdError;

// use crate::server::{ambur::AmburMcp, token::AmburTokenMcp};
use crate::server::ambur::AmburMcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let ambur_mcp = AmburMcp::new().serve(stdio()).await.inspect_err(|e| {
        println!("{e}");
    })?;
    ambur_mcp.waiting().await?;

    // let token_mcp = AmburTokenMcp::new().serve(stdio()).await.inspect_err(|e| {
    //     println!("{e}");
    // })?;
    // token_mcp.waiting().await?;

    Ok(())
}
