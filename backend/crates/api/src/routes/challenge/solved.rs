use anyhow::Result;
use reqwest::Url;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    NodeState,
    blockchain::NodeType,
    error::{AppError, AppResult},
    routes::{
        challenge::Challenge,
        node::{
            create::{create_node, deploy_node, wait_pod_running},
            remove_node,
        },
    },
};

pub async fn is_can_be_solved(
    pool: &PgPool,
    node_state: &NodeState,
    challenge: &Challenge,
    user_id: Uuid,
) -> AppResult<bool> {
    let node_type = NodeType::from(challenge.blockchain);
    let node_id = create_node(pool, user_id, challenge.id, node_type).await?;

    let node_name = deploy_node(node_state, node_id, node_type).await?;
    wait_pod_running(pool, node_state, &node_name, node_id).await?;

    let result = deploy_and_exploit(challenge, &node_name)
        .await
        .map_err(|_| AppError::CannotBeDeployed);

    // remove the node before returning the result
    remove_node(pool, node_state, node_id, node_type).await?;

    result
}

async fn deploy_and_exploit(challenge: &Challenge, node_name: &str) -> Result<bool> {
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
