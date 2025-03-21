use axum::{Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppResult;
use crate::blockchain::BlockchainType;

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

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<NewChallenge>
) -> AppResult<Json<NewChallenge>> {

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
    .fetch_one(&pool)
    .await?;

    Ok(Json(NewChallenge {
        id,
        author_id: req.author_id,
        title: req.title,
        description: req.description,
        code: req.code,
        bytecode: req.bytecode,
        value: req.value,
        difficulty: req.difficulty,
        blockchain: req.blockchain,
    }))
}