use anyhow::Result;
use axum::{Extension, Json, extract::State};
use chrono::Utc;
use reqwest::Url;
use sqlx::PgPool;
use uuid::Uuid;

use super::Challenge;
use crate::{
    AppState, NodeState,
    blockchain::{BlockchainType, NodeType},
    error::{AppError, AppResult},
    routes::node::{
        create::{create_node, deploy_node, wait_pod_running},
        remove_node,
    },
};

#[derive(Clone, serde::Deserialize)]
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

pub async fn create(
    Extension(user_id): Extension<Uuid>,
    Extension(app_state): Extension<AppState>,
    State(node_state): State<NodeState>,
    Json(req): Json<CreateChallengeRequest>,
) -> AppResult<Json<Challenge>> {
    if !is_can_be_solved(
        &app_state.pool,
        &user_id,
        &node_state,
        &Challenge::from(req.clone()),
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
    .bind(&req.blockchain)
    .fetch_one(&app_state.pool)
    .await?;

    Ok(Json(challenge))
}

pub async fn is_can_be_solved(
    pool: &PgPool,
    user_id: &Uuid,
    node_state: &NodeState,
    challenge: &Challenge,
) -> AppResult<bool> {
    let node_type = NodeType::from(&challenge.blockchain);
    let node_id = create_node(pool, user_id, &challenge.id, &node_type).await?;

    let node_name = deploy_node(node_state, &node_type, &node_id).await?;
    wait_pod_running(pool, node_state, &node_id, &node_name).await?;

    let result = match deploy_and_exploit(&node_name, challenge).await {
        Ok(success) => Ok(success),
        Err(_) => Err(AppError::CannotBeDeployed),
    };

    remove_node(pool, node_state, &node_type, &node_id).await?;

    result
}

async fn deploy_and_exploit(node_name: &str, challenge: &Challenge) -> Result<bool> {
    let node_url = Url::parse(&format!("http://{node_name}"))?;
    let provider = challenge.blockchain.provider(node_url);

    let level = provider.create_level(&challenge.bytecode).await?;
    let instances = provider.create_instances(&level, &challenge.value).await?;

    let validated = provider.validate_instances(&level, &instances).await?;
    if validated {
        return Ok(false);
    }

    provider
        .exploit_instances(&instances, &challenge.exploit_bytecode, &challenge.exploit_value)
        .await?;

    let validated = provider.validate_instances(&level, &instances).await?;
    if !validated {
        return Ok(false);
    }

    Ok(true)
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
            blockchain: req.blockchain,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
