use axum::{Extension, Json,};
use crate::error::{AppResult, AppError};
use crate::routes::ApiContext;

use super::Challenge;

#[axum::debug_handler]
pub async fn create(
    ctx: Extension<ApiContext>,
    Json(req): Json<Challenge>,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO challenge (author_id, title, description, code, bytecode, difficulty)
        VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&req.author_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.difficulty)
    .execute(&ctx.db)
    .await
    .map_err(AppError::SqlxError)?;

    Ok(())
}
