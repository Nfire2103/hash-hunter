use axum::{Extension, Json, extract::Path};
use uuid::Uuid;

use super::Challenge;
use crate::{
    AppState,
    error::{AppError, AppResult},
};

#[derive(serde::Deserialize)]
pub struct UpdateChallenge {
    title: Option<String>,
    description: Option<String>,
    code: Option<String>,
    bytecode: Option<String>,
    value: Option<String>,
    difficulty: Option<i16>,
}

// TODO if bytecode is updated, check that the code is valid
pub async fn update(
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateChallenge>,
) -> AppResult<Json<Challenge>> {
    let challenge = sqlx::query_as::<_, Challenge>(
        "UPDATE challenge
        SET title = COALESCE($1, challenge.title),
            description = COALESCE($2, challenge.description),
            code = COALESCE($3, challenge.code),
            bytecode = COALESCE($4, challenge.bytecode),
            value = COALESCE($5, challenge.value),
            difficulty = COALESCE($6, challenge.difficulty)
        WHERE id = $7
        RETURNING id, author_id, title, description, code, bytecode, value, difficulty, solved,
            blockchain, created_at, updated_at",
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.difficulty)
    .bind(&uuid)
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(challenge))
}
