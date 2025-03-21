use axum::{extract::{Extension, Path}, Json};
use uuid::Uuid;
use sqlx::PgPool;
use crate::{error::AppResult, error::AppError};
use crate::routes::challenge::Challenge;

<<<<<<< HEAD

pub async fn get(Extension(pool): Extension<PgPool>, Path(uuid): Path<Uuid>) -> AppResult<Json<Challenge>> {
    let challenge = get_challenge(&pool, &uuid).await?;
    Ok(challenge)
=======
use super::Challenge;
use crate::{
    AppState,
    error::{AppError, AppResult},
};

pub async fn get(
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<Challenge>> {
    let challenge = get_challenge(&app_state.pool, &uuid).await?;
    Ok(Json(challenge))
>>>>>>> main
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
    .ok_or(AppError::SqlxRowNotFound)?;

    Ok(Json(challenge))
}
