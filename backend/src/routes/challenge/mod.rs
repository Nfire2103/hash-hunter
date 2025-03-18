use crate::config::Config;
use sqlx::{PgPool, FromRow};
use std::{sync::Arc, time::SystemTime};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

mod create;
mod get;
mod remove;
mod update;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeBody {
    pub challenge: Challenge,
}

#[derive(Serialize, Deserialize)]
pub struct NewChallenge {
    pub author_id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub bytecode: String,
    pub difficulty: i16
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Challenge {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub bytecode: String,
    pub difficulty: i16,
    pub solved: i32,
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

use crate::AppState;

pub fn router() -> Router {
    Router::new()
        .route("/challenge", post(create::create))
        .route("/challenge/{uuid}", get(get::get))
        .route("/challenge/{uuid}", put(update::update))
        .route("/challenge/{uuid}", delete(remove::remove))
}
