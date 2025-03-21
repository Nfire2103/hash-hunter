use anyhow::Result;
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use k8s_openapi::api::{apps::v1::Deployment, core::v1::Pod};
use kube::{
    Api,
    runtime::{conditions::is_deleted, wait::await_condition},
};
use sqlx::PgPool;
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use super::{
    NodeState,
    create::{NodeCreateResponse, deploy_instances, wait_pod_running},
    get::get_node,
};
use crate::{error::AppResult, routes::challenge::get::get_challenge};

pub async fn reset(
    Extension(pool): Extension<PgPool>,
    State(state): State<NodeState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<NodeCreateResponse>> {
    let node = get_node(&pool, &uuid).await?;
    let challenge = get_challenge(&pool, &node.challenge_id).await?;

    let node_name = format!("{}-{}", node.node_type, uuid);
    let deployments: Api<Deployment> = Api::default_namespaced(state.kube_client.clone());

    deployments.restart(&node_name).await?;
    wait_pod_deleted(&state, &node.pod_name, &node.pod_uid).await?;
    wait_pod_running(&pool, &state, &uuid, &node_name).await?;

    let ((pubkey, privatekey), instances) =
        deploy_instances(&pool, &uuid, &node_name, &challenge).await?;

    Ok(Json(NodeCreateResponse {
        node_id: uuid,
        url_suffix: format!("/rpc/{}", uuid),
        instances,
        player_pubkey: pubkey.to_string(),
        player_privatekey: privatekey.to_string(),
    }))
}

async fn wait_pod_deleted(state: &NodeState, pod_name: &str, pod_uid: &str) -> Result<()> {
    let pods: Api<Pod> = Api::default_namespaced(state.kube_client.clone());

    let running = await_condition(pods, pod_name, is_deleted(pod_uid));
    timeout(Duration::from_secs(30), running).await??;

    Ok(())
}
