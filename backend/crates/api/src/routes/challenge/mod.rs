mod create;
mod get;
mod remove;
mod solved;
mod update;

use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use chrono::NaiveDateTime;
pub use get::get_challenge;
use serde::Serialize;
use sqlx::FromRow;
#[cfg(feature = "swagger")]
use utoipa::OpenApi;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{NodeState, blockchain::BlockchainType, middlewares::challenge::check_curr_user_is_owner};

#[derive(Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub bytecode: String,
    pub value: String,
    pub exploit_bytecode: String,
    pub exploit_value: String,
    pub difficulty: i16,
    pub solved: i32,
    pub blockchain: BlockchainType,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
}

pub fn router() -> Router<NodeState> {
    Router::new()
        .route("/challenge/{uuid}", get(get::get))
        .route("/challenge/{uuid}", patch(update::update))
        .route("/challenge/{uuid}", delete(remove::remove))
        .layer(middleware::from_fn(check_curr_user_is_owner))
        .route("/challenge", post(create::create))
}

#[cfg(feature = "swagger")]
#[derive(OpenApi)]
#[openapi(
    paths(
        create::create,
        get::get,
        update::update,
        remove::remove
    ),
    components(
        schemas()
    ),
    tags(
        (name = "Challenges", description = "Challenge management")
    )
)]
pub struct ChallengeApiDoc;
