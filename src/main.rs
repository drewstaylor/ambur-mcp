pub mod contract;
pub mod execute;
pub mod instruction;
pub mod network;
pub mod query;
pub mod server;

use anyhow::Result;
use rmcp::transport::streamable_http_server::axum::StreamableHttpServer;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

use crate::server::ambur::AmburMcp;

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ct = StreamableHttpServer::serve(BIND_ADDRESS.parse()?)
        .await?
        .with_service(AmburMcp::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();

    Ok(())
}
