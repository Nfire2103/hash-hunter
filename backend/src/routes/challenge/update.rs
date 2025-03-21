use axum::{Extension, Json,};
use crate::error::AppResult;
use sqlx::PgPool;

use super::Challenge;


pub async fn update(Extension(pool): Extension<PgPool>, Json(req): Json<Challenge>) -> AppResult<Json<Challenge>> {
    let challenge = update_challenge(&pool, req).await?;
    Ok(challenge)
}

pub async fn update_challenge(
    pool: &PgPool,
    req: Challenge,
) -> AppResult<Json<Challenge>> {
    let challenge = sqlx::query_as::<_, Challenge>(
        "UPDATE challenge
        SET author_id = coalesce($1, challenge.author_id),
            title = coalesce($2, challenge.title),
            description = coalesce($3, challenge.description),
            code = coalesce($4, challenge.code),
            bytecode = coalesce($5, challenge.bytecode),
            value = coalesce($6, challenge.value),
            difficulty = coalesce($7, challenge.difficulty)
            blockchain = coalesce($8, challenge.blockchain)
        WHERE id = $9
        RETURNING id, author_id, title, description, code, bytecode, difficulty, solved, created_at, updated_at"
    )
    .bind(&req.author_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.difficulty)
    .bind(&req.blockchain)
    .bind(&req.id)
    .fetch_one(pool)
    .await?;

    Ok(Json(challenge))
}
