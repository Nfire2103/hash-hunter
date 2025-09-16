use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

impl AppState {
    pub fn try_from_args(args: &Config) -> Result<Self> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(args.database.database_max_connections)
                .connect_lazy(&args.database.database_url)?,
            jwt_secret: args.jwt_secret.clone(),
        })
    }
}
