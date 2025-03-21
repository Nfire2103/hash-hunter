use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

impl AppState {
    pub fn try_from_config(config: &AppConfig) -> Result<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(config.database.database_max_connections)
                .connect_lazy(&config.database.database_url)?,
            jwt_secret: config.jwt_secret.clone(),
        })
    }
}
