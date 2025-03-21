use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::blockchain::BlockchainType;

mod create;
pub mod get;
mod remove;
mod update;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
// enleve mes types et faire comme node/create et returne challenge

#[derive(Serialize, Deserialize, FromRow)]
pub struct Challenge {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub bytecode: String,
    pub value: String,
    pub difficulty: i16,
    pub solved: i32,
    pub blockchain: BlockchainType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateChallenge {
    pub id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub code: Option<String>,
    pub bytecode: Option<String>,
    pub difficulty: Option<i16>,
}

pub fn router() -> Router {
    Router::new()
        .route("/challenge", post(create::create))
        .route("/challenge/{uuid}", get(get::get))
        .route("/challenge/{uuid}", put(update::update))
        .route("/challenge/{uuid}", delete(remove::remove))
}
