use axum::{Extension, Json};
use sqlx::PgPool;
use anyhow::anyhow;

use super::NewChallenge;
use crate::error::AppResult;

pub async fn create(Extension(pool): Extension<PgPool>, Json(req): Json<NewChallenge>) -> AppResult<()> {
    create_challenge(&pool, req).await?;
    Ok(())
}

pub async fn create_challenge(pool: &PgPool, req: NewChallenge) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO challenge (author_id, title, description, code, bytecode, value, difficulty, blockchain)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(&req.author_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.difficulty)
    .bind(&req.blockchain)
    .execute(pool)
    .await
    .map_err(|err| anyhow!(err))?;

    Ok(())
}