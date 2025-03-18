use axum::{Extension, Json,};
use crate::error::AppResult;

use super::{ApiContext, UpdateChallenge, ChallengeBody, Challenge};


#[axum::debug_handler]
pub async fn update(
    ctx: Extension<ApiContext>,
    Json(req): Json<UpdateChallenge>,
) -> AppResult<Json<ChallengeBody>> {
    let challenge = sqlx::query_as::<_, Challenge>(
        "UPDATE challenge
        SET author_id = coalesce($1, challenge.author_id),
            title = coalesce($2, challenge.title),
            description = coalesce($3, challenge.description),
            code = coalesce($4, challenge.code),
            bytecode = coalesce($5, challenge.bytecode),
            difficulty = coalesce($6, challenge.difficulty)
        WHERE id = $7
        RETURNING id, author_id, title, description, code, bytecode, difficulty, solved, created_at, updated_at"
    )
    .bind(&req.author_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.difficulty)
    .bind(&req.id)
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(ChallengeBody {challenge}))
}

