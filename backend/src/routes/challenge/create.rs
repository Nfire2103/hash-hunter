use axum::{Extension, Json};
use uuid::Uuid;
use crate::blockchain::BlockchainType;
use crate::{
    AppState,
    error::AppResult,
};

#[derive(serde::Serialize)]
pub struct NewChallenge {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub bytecode: String,
    pub value: String,
    pub difficulty: i16,
    pub blockchain: BlockchainType
}

use super::{Challenge, get::get_challenge};

pub async fn create(
    Extension(state): Extension<AppState>,
    Json(req): Json<NewChallenge>,
) -> AppResult<Json<Challenge>> {

    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO challenge (author_id, title, description, code, bytecode, value, difficulty, blockchain)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id",
    )
    .bind(&req.author_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.difficulty)
    .bind(&req.blockchain)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(get_challenge(&state.pool, &id).await?))
}