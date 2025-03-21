use axum::{Extension, Json};
use uuid::Uuid;

use super::Challenge;
use crate::{AppState, blockchain::BlockchainType, error::AppResult};

#[derive(serde::Deserialize)]
pub struct CreateChallengeRequest {
    title: String,
    description: String,
    code: String,
    bytecode: String,
    value: String,
    difficulty: i32,
    blockchain: BlockchainType,
}

// TODO check that the challenge is valid
pub async fn create(
    Extension(user_id): Extension<Uuid>,
    Extension(app_state): Extension<AppState>,
    Json(req): Json<CreateChallengeRequest>,
) -> AppResult<Json<Challenge>> {
    let challenge = sqlx::query_as::<_, Challenge>(
        "INSERT INTO challenge (author_id, title, description, code, bytecode, value, difficulty,
            blockchain) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, author_id, title, description, code, bytecode, value, difficulty, solved,
            blockchain, created_at, updated_at",
    )
    .bind(&user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.difficulty)
    .bind(&req.blockchain)
    .fetch_one(&app_state.pool)
    .await?;

    Ok(Json(challenge))
}
