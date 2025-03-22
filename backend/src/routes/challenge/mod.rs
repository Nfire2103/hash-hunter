mod create;
mod get;
mod remove;
mod update;

use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use chrono::NaiveDateTime;
pub use get::get_challenge;
use uuid::Uuid;

use crate::{blockchain::BlockchainType, middlewares::challenge::check_curr_user_is_owner};

#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn router() -> Router {
    Router::new()
        .route("/challenge/{uuid}", get(get::get))
        .route("/challenge/{uuid}", patch(update::update))
        .route("/challenge/{uuid}", delete(remove::remove))
        .layer(middleware::from_fn(check_curr_user_is_owner))
        .route("/challenge", post(create::create))
}
