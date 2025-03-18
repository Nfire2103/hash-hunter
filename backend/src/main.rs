use anyhow::Result;
use backend::{AppConfig, AppState};
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();

    dotenvy::dotenv().ok();

    let config = AppConfig::parse();
    let address = config.address;

    let app = backend::build(AppState::try_from_conf(config).await?);
    let listener = TcpListener::bind(address).await?;

    info!("Listening on {}", address);

    // TODO make a .into_make_service?
    axum::serve(listener, app).await?;

    Ok(())
}
