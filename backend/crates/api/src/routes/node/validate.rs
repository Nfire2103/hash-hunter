use anyhow::anyhow;
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use reqwest::Url;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{NodeState, get::get_node, remove::remove_node};
use crate::{AppState, blockchain::BlockchainType, error::AppResult};

#[derive(Serialize, ToSchema)]
pub struct NodeValidateResponse {
    validated: bool,
}

#[utoipa::path(
    post,
    path = "/node/{uuid}/validate",
    responses(
        (status = 200, description = "Node validation completed", body = NodeValidateResponse),
        (status = 404, description = "Node not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Nodes"
)]
pub async fn validate(
    Extension(app_state): Extension<AppState>,
    State(state): State<NodeState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<NodeValidateResponse>> {
    let node = get_node(&app_state.pool, uuid).await?;
    let node_url = Url::parse(&format!("http://{}-{uuid}", node.r#type))
        .map_err(|_| anyhow!("Failed to parse url"))?;

    let blockchain = BlockchainType::from(node.r#type);
    let provider = blockchain.provider(node_url);

    let validated = provider.validate_instances(&node.level, &node.instances).await?;
    if validated {
        remove_node(&app_state.pool, &state, uuid, node.r#type).await?;
    }

    Ok(Json(NodeValidateResponse { validated }))
}
