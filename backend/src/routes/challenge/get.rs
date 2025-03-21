use axum::{extract::{Extension, Path}, Json};
use uuid::Uuid;
use crate::{error::AppResult, error::AppError};
use crate::routes::challenge::Challenge;

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
}

pub async fn get_challenge(
    pool: &PgPool,
    uuid: &Uuid,
) -> AppResult<Json<Challenge>> {
    let challenge = sqlx::query_as(
        "SELECT
        (id, author_id, title, description, code, bytecode, difficulty, solved, created_at, updated_at)
        FROM challenge
        WHERE id = $1"
    )
    .bind(uuid)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(AppError::SqlxRowNotFound)?;

    Ok(Json(ChallengeBody {challenge}))
}
