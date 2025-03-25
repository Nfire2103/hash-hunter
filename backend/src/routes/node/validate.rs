use anyhow::anyhow;
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use reqwest::Url;
use uuid::Uuid;

use super::{NodeState, get::get_node, remove::remove_node};
use crate::{AppState, blockchain::BlockchainType, error::AppResult};

#[derive(serde::Serialize)]
pub struct NodeValidateResponse {
    validated: bool,
}

pub async fn validate(
    Extension(app_state): Extension<AppState>,
    State(state): State<NodeState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<NodeValidateResponse>> {
    let node = get_node(&app_state.pool, &uuid).await?;
    let node_url = Url::parse(&format!("http://{}-{}", node.node_type, uuid))
        .map_err(|_| anyhow!("Failed to parse url"))?;

    let blockchain = BlockchainType::from(&node.node_type);
    let provider = blockchain.provider(node_url);

    let validated = provider.validate_instances(&node.level, &node.instances).await?;

    if validated {
        remove_node(&app_state.pool, &state, &node.node_type, &uuid).await?;
    }

    Ok(Json(NodeValidateResponse { validated }))
}
