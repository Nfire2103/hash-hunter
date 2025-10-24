use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{Challenge, get_challenge};
use crate::{
    AppState, NodeState,
    error::{AppError, AppResult},
    routes::challenge::solved::is_can_be_solved,
};

#[derive(Clone, Deserialize, ToSchema)]
pub struct UpdateChallengeRequest {
    title: Option<String>,
    description: Option<String>,
    code: Option<String>,
    bytecode: Option<String>,
    value: Option<String>,
    exploit_bytecode: Option<String>,
    exploit_value: Option<String>,
    difficulty: Option<i16>,
}

impl Challenge {
    fn with_updated_fields(self, req: UpdateChallengeRequest) -> Self {
        Self {
            id: self.id,
            author_id: self.author_id,
            title: req.title.unwrap_or(self.title),
            description: req.description.unwrap_or(self.description),
            code: req.code.unwrap_or(self.code),
            bytecode: req.bytecode.unwrap_or(self.bytecode),
            value: req.value.unwrap_or(self.value),
            exploit_bytecode: req.exploit_bytecode.unwrap_or(self.exploit_bytecode),
            exploit_value: req.exploit_value.unwrap_or(self.exploit_value),
            difficulty: req.difficulty.unwrap_or(self.difficulty),
            solved: self.solved,
            blockchain: self.blockchain,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[utoipa::path(
    patch,
    path = "/challenge/{uuid}",
    request_body = UpdateChallengeRequest,
    responses(
        (status = 200, description = "Challenge updated successfully", body = Challenge),
        (status = 404, description = "Challenge not found"),
        (status = 400, description = "Challenge cannot be solved or invalid data"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Challenges"
)]
pub async fn update(
    Extension(user_id): Extension<Uuid>,
    Extension(app_state): Extension<AppState>,
    State(node_state): State<NodeState>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateChallengeRequest>,
) -> AppResult<Json<Challenge>> {
    if req.bytecode.is_some()
        || req.value.is_some()
        || req.exploit_bytecode.is_some()
        || req.exploit_value.is_some()
    {
        let challenge = get_challenge(&app_state.pool, uuid).await?;

        if !is_can_be_solved(
            &app_state.pool,
            &node_state,
            &challenge.with_updated_fields(req.clone()),
            user_id,
        )
        .await?
        {
            return Err(AppError::CannotBeSolved);
        }
    }

    let challenge = sqlx::query_as::<_, Challenge>(
        "UPDATE challenge
        SET title = COALESCE($1, challenge.title),
            description = COALESCE($2, challenge.description),
            code = COALESCE($3, challenge.code),
            bytecode = COALESCE($4, challenge.bytecode),
            value = COALESCE($5, challenge.value),
            exploit_bytecode = COALESCE($6, challenge.exploit_bytecode),
            exploit_value = COALESCE($7, challenge.exploit_value),
            difficulty = COALESCE($8, challenge.difficulty)
        WHERE id = $9
        RETURNING id, author_id, title, description, code, bytecode, value, exploit_bytecode,
            exploit_value, difficulty, solved, blockchain, created_at, updated_at",
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.exploit_bytecode)
    .bind(&req.exploit_value)
    .bind(req.difficulty)
    .bind(uuid)
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(challenge))
}
