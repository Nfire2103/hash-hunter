use anyhow::Result;
use api::{AppState, Config, NodeState};
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();

    dotenvy::dotenv().ok();

    let args = Config::parse();
    let address = args.address;

    let app_state = AppState::try_from_args(&args)?;
    let node_state = NodeState::try_from_args(args.node).await?;

    let app = api::build(app_state, node_state);
    let listener = TcpListener::bind(address).await?;

    info!("Listening on {}", address);

    // TODO make a .into_make_service?
    axum::serve(listener, app).await?;

    Ok(())
}
