use anyhow::Result;
use backend::{AppConfig, AppState, NodeState};
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();

    dotenvy::dotenv().ok();

    let config = AppConfig::parse();
    let address = config.address;

    let app_state = AppState::try_from_config(&config)?;
    let node_state = NodeState::try_from_args(config.node).await?;

    let app = backend::build(app_state, node_state);
    let listener = TcpListener::bind(address).await?;

    info!("Listening on {}", address);

    // TODO make a .into_make_service?
    axum::serve(listener, app).await?;

    Ok(())
}
