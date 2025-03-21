use axum::{extract::{Extension, Path}, Json};
use uuid::Uuid;
use sqlx::PgPool;
use crate::{error::AppResult, error::AppError};
use crate::routes::challenge::Challenge;


pub async fn get(Extension(pool): Extension<PgPool>, Path(uuid): Path<Uuid>) -> AppResult<Json<Challenge>> {
    let challenge = get_challenge(&pool, &uuid).await?;
    Ok(challenge)
}

pub async fn get_challenge(
    pool: &PgPool,
    uuid: &Uuid,
) -> AppResult<Json<Challenge>> {
    let challenge = sqlx::query_as(
        "SELECT
        (id, author_id, title, description, code, bytecode, value, difficulty, solved, blockchain, created_at, updated_at)
        FROM challenge
        WHERE id = $1"
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(challenge))
}