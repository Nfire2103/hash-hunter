use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{config::AppConfig, routes::node::NodeState};

pub struct AppState {
    pub pool: PgPool,
    pub http_client: reqwest::Client,
    pub node_state: NodeState,
}

impl AppState {
    pub async fn try_from_conf(config: AppConfig) -> Result<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(config.database.database_max_connections)
                .connect_lazy(&config.database.database_url)?,
            http_client: reqwest::Client::new(),
            node_state: NodeState::try_from_args(config.node).await?,
        })
    }
}
