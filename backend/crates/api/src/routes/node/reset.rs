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
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use super::{
    NodeState,
    create::{NodeCreateResponse, deploy_instances, wait_pod_running},
    get::get_node,
};
use crate::{AppState, error::AppResult, routes::challenge::get_challenge};

#[utoipa::path(
    post,
    path = "/node/{uuid}/reset",
    responses(
        (status = 200, description = "Node reset successfully", body = NodeCreateResponse),
        (status = 404, description = "Node not found"),
        (status = 500, description = "Internal server error or Kubernetes deployment failure")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Nodes"
)]
pub async fn reset(
    Extension(app_state): Extension<AppState>,
    State(state): State<NodeState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<NodeCreateResponse>> {
    let node = get_node(&app_state.pool, uuid).await?;
    let challenge = get_challenge(&app_state.pool, node.challenge_id).await?;

    let node_name = format!("{}-{uuid}", node.r#type);
    let deployments: Api<Deployment> = Api::default_namespaced(state.kube_client.clone());

    deployments.restart(&node_name).await?;
    wait_pod_deleted(&state, &node.pod_name, &node.pod_uid).await?;
    wait_pod_running(&app_state.pool, &state, &node_name, uuid).await?;

    let ((pubkey, privatekey), instances) =
        deploy_instances(&app_state.pool, &challenge, &node_name, uuid).await?;

    Ok(Json(NodeCreateResponse {
        node_id: uuid,
        url_suffix: format!("/rpc/{uuid}"),
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
