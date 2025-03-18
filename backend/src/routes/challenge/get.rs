use axum::{extract::{Extension, Path}, Json};
use uuid::Uuid;
use crate::{error::AppResult, error::AppError, routes::challenge::{ApiContext, ChallengeBody}};


#[axum::debug_handler]
pub async fn get(
    ctx: Extension<ApiContext>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<ChallengeBody>> {
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
