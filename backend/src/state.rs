use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub http_client: reqwest::Client,
    pub anvil_uri: String,
}

impl TryFrom<Config> for AppState {
    type Error = anyhow::Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(config.db_max_connections)
                .connect_lazy(&config.db_uri)?,
            http_client: reqwest::Client::new(),
            anvil_uri: config.anvil_uri,
        })
    }
}
