use axum::{Extension, Json, extract::State};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use super::Challenge;
use crate::{
    AppState, NodeState,
    blockchain::BlockchainType,
    error::{AppError, AppResult},
    routes::challenge::solved::is_can_be_solved,
};

#[derive(Clone, Deserialize, utoipa::ToSchema)]
pub struct CreateChallengeRequest {
    title: String,
    description: String,
    code: String,
    bytecode: String,
    value: String,
    exploit_bytecode: String,
    exploit_value: String,
    difficulty: i16,
    blockchain: BlockchainType,
}

impl From<CreateChallengeRequest> for Challenge {
    fn from(req: CreateChallengeRequest) -> Self {
        Self {
            id: Uuid::default(),
            author_id: Uuid::default(),
            title: req.title,
            description: req.description,
            code: req.code,
            bytecode: req.bytecode,
            value: req.value,
            exploit_bytecode: req.exploit_bytecode,
            exploit_value: req.exploit_value,
            difficulty: req.difficulty,
            solved: 0,
            blockchain: BlockchainType::Ethereum,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[utoipa::path(
    post,
    path = "/challenge",
    request_body = CreateChallengeRequest,
    responses(
        (status = 200, description = "Challenge created successfully", body = Challenge),
        (status = 400, description = "Challenge cannot be solved or invalid data"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Challenges"
)]
pub async fn create(
    Extension(user_id): Extension<Uuid>,
    Extension(app_state): Extension<AppState>,
    State(node_state): State<NodeState>,
    Json(req): Json<CreateChallengeRequest>,
) -> AppResult<Json<Challenge>> {
    if !is_can_be_solved(
        &app_state.pool,
        &node_state,
        &Challenge::from(req.clone()),
        user_id,
    )
    .await?
    {
        return Err(AppError::CannotBeSolved);
    }

    let challenge = sqlx::query_as::<_, Challenge>(
        "INSERT INTO challenge (author_id, title, description, code, bytecode, value, exploit_bytecode,
            exploit_value, difficulty, blockchain) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, author_id, title, description, code, bytecode, value, exploit_bytecode,
            exploit_value, difficulty, solved, blockchain, created_at, updated_at",
    )
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.code)
    .bind(&req.bytecode)
    .bind(&req.value)
    .bind(&req.exploit_bytecode)
    .bind(&req.exploit_value)
    .bind(req.difficulty)
    .bind(req.blockchain)
    .fetch_one(&app_state.pool)
    .await?;

    Ok(Json(challenge))
}
