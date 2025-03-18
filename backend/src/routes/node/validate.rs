use anyhow::anyhow;
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use reqwest::Url;
use sqlx::PgPool;
use uuid::Uuid;

use super::{NodeState, get::get_node, remove::remove_node};
use crate::{blockchain::BlockchainType, error::AppResult};

#[derive(serde::Serialize)]
pub struct NodeValidateResponse {
    validated: bool,
}

pub async fn validate(
    Extension(pool): Extension<PgPool>,
    State(state): State<NodeState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<NodeValidateResponse>> {
    let node = get_node(&pool, &uuid).await?;
    let node_url = Url::parse(&format!("http://{}-{}", node.node_type, uuid))
        .map_err(|_| anyhow!("Failed to parse url"))?;

    let blockchain = BlockchainType::from(&node.node_type);
    let provider = blockchain.provider(node_url);

    let (pubkey, _) = provider.player_wallet();
    let validated = provider
        .validate_instances(&node.level, pubkey, &node.instances)
        .await?;

    if validated {
        remove_node(&pool, &state, &node.node_type, &uuid).await?;
    }

    Ok(Json(NodeValidateResponse { validated }))
}
