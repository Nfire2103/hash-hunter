use std::sync::Arc;
use sqlx::PgPool;
use crate::config::Config;

pub mod anvil;
pub mod challenge;
pub mod node;
pub mod rpc;
pub mod user;
pub mod auth;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}