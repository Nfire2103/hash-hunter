use anyhow::{Context, Result, anyhow, bail};
use axum::{Extension, Json, extract::State};
use k8s_openapi::api::{
    apps::v1::Deployment,
    core::v1::{Pod, Service},
};
use kube::{
    Api,
    api::ListParams,
    runtime::{conditions::is_pod_running, reflector::Lookup, wait::await_condition},
};
use reqwest::Url;
use sqlx::PgPool;
use tokio::time::{Duration, sleep, timeout};
use uuid::Uuid;

use super::state::NodeState;
use crate::{
    AppState,
    blockchain::NodeType,
    error::AppResult,
    routes::challenge::{Challenge, get_challenge},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCreateRequest {
    challenge_id: Uuid,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCreateResponse {
    pub node_id: Uuid,
    pub url_suffix: String,
    pub instances: Vec<String>,
    pub player_pubkey: String,
    pub player_privatekey: String,
}

pub async fn create(
    Extension(user_id): Extension<Uuid>,
    Extension(app_state): Extension<AppState>,
    State(state): State<NodeState>,
    Json(req): Json<NodeCreateRequest>,
) -> AppResult<Json<NodeCreateResponse>> {
    let challenge = get_challenge(&app_state.pool, &req.challenge_id).await?;

    let node_type = NodeType::from(&challenge.blockchain);
    let node_id = create_node(&app_state.pool, &user_id, &challenge.id, &node_type).await?;

    let node_name = deploy_node(&state, &node_type, &node_id).await?;
    wait_pod_running(&app_state.pool, &state, &node_id, &node_name).await?;

    let ((pubkey, privatekey), instances) =
        deploy_instances(&app_state.pool, &node_id, &node_name, &challenge).await?;

    Ok(Json(NodeCreateResponse {
        node_id,
        url_suffix: format!("/rpc/{}", node_id),
        instances,
        player_pubkey: pubkey.to_string(),
        player_privatekey: privatekey.to_string(),
    }))
}

pub async fn create_node(
    pool: &PgPool,
    user_id: &Uuid,
    challenge_id: &Uuid,
    node_type: &NodeType,
) -> AppResult<Uuid> {
    let node_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO node (user_id, challenge_id, type)
        VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(user_id)
    .bind(challenge_id)
    .bind(node_type)
    .fetch_one(pool)
    .await?;

    Ok(node_id)
}

fn fill_manifests(
    state: &NodeState,
    node_type: &NodeType,
    node_id: &Uuid,
) -> Result<(Deployment, Service)> {
    let mut context = tera::Context::new();
    context.insert("node", node_type);
    context.insert("uuid", node_id);

    let deployment = state.tera.render(&state.deployment_file, &context)?;
    let service = state.tera.render(&state.service_file, &context)?;

    let deployment: Deployment = serde_yaml::from_str(&deployment)?;
    let service: Service = serde_yaml::from_str(&service)?;

    Ok((deployment, service))
}

pub async fn deploy_node(state: &NodeState, node_type: &NodeType, node_id: &Uuid) -> Result<String> {
    let (deployment, service) = fill_manifests(state, node_type, node_id)?;

    let deployments: Api<Deployment> = Api::default_namespaced(state.kube_client.clone());
    deployments.create(&Default::default(), &deployment).await?;

    let services: Api<Service> = Api::default_namespaced(state.kube_client.clone());
    services.create(&Default::default(), &service).await?;

    Ok(format!("{}-{}", node_type, node_id))
}

pub async fn wait_pod_running(
    pool: &PgPool,
    state: &NodeState,
    node_id: &Uuid,
    node_name: &str,
) -> Result<()> {
    let pods: Api<Pod> = Api::default_namespaced(state.kube_client.clone());
    let pod = get_pod_with_retry(&pods, node_name, 50).await?;

    let pod_name = pod.name().context("Failed to deploy instance")?;
    let pod_uip = pod.uid().context("Failed to deploy instance")?;

    let running = await_condition(pods, &pod_name, is_pod_running());
    timeout(Duration::from_secs(30), running).await??;

    await_service_up(&state.http_client, node_name, 50).await?;

    sqlx::query("UPDATE node SET pod_name = $1, pod_uid = $2 WHERE id = $3")
        .bind(pod_name)
        .bind(pod_uip)
        .bind(node_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_pod_with_retry(pods: &Api<Pod>, node_name: &str, max_retry: u32) -> Result<Pod> {
    let label = format!("app.kubernetes.io/name={node_name}");

    for _ in 0..max_retry {
        if let Some(pod) = pods
            .list(&ListParams::default().labels(&label))
            .await?
            .items
            .first()
        {
            return Ok(pod.clone());
        }

        sleep(Duration::from_millis(100)).await;
    }

    bail!("Failed to get pod");
}

async fn await_service_up(client: &reqwest::Client, node_name: &str, max_retry: u32) -> Result<()> {
    for _ in 0..max_retry {
        if let Ok(_) = client.get(format!("http://{node_name}")).send().await {
            return Ok(());
        }

        sleep(Duration::from_millis(100)).await;
    }

    bail!("Failed to await service up");
}

pub async fn deploy_instances(
    pool: &PgPool,
    node_id: &Uuid,
    node_name: &str,
    challenge: &Challenge,
) -> Result<((&'static str, &'static str), Vec<String>)> {
    let node_url =
        Url::parse(&format!("http://{node_name}")).map_err(|_| anyhow!("Failed to parse url"))?;
    let provider = challenge.blockchain.provider(node_url);

    let (pubkey, privatekey) = provider.player_wallet();
    let level = provider.create_level(&challenge.bytecode).await?;
    let instances = provider.create_instances(&level, &challenge.value).await?;

    sqlx::query("UPDATE node SET level = $1, instances = $2 WHERE id = $3")
        .bind(&level)
        .bind(&instances)
        .bind(node_id)
        .execute(pool)
        .await?;

    Ok(((pubkey, privatekey), instances))
}
