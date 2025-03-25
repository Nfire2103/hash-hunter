use axum::{Extension, Json, extract::Path};
use sqlx::PgPool;
use uuid::Uuid;

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

pub async fn get_challenge(pool: &PgPool, uuid: &Uuid) -> AppResult<Challenge> {
    let challenge = sqlx::query_as::<_, Challenge>(
        "SELECT id, author_id, title, description, code, bytecode, value, exploit_bytecode,
            exploit_value, difficulty, solved, blockchain, created_at, updated_at
        FROM challenge WHERE id = $1",
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(challenge)
}
