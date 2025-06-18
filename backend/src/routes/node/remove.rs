use axum::{
    Extension,
    extract::{Path, State},
};
use k8s_openapi::api::{apps::v1::Deployment, core::v1::Service};
use kube::{Api, api::DeleteParams};
use sqlx::PgPool;
use uuid::Uuid;

use super::NodeState;
use crate::{
    AppState,
    blockchain::NodeType,
    error::{AppError, AppResult},
};

pub async fn remove(
    Extension(app_state): Extension<AppState>,
    State(state): State<NodeState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<()> {
    let node_type = sqlx::query_scalar::<_, NodeType>("SELECT type FROM node WHERE id = $1")
        .bind(uuid)
        .fetch_optional(&app_state.pool)
        .await?
        .ok_or(AppError::NotFound)?;

    remove_node(&app_state.pool, &state, &node_type, &uuid).await?;

    Ok(())
}

pub async fn remove_node(
    pool: &PgPool,
    state: &NodeState,
    node_type: &NodeType,
    uuid: &Uuid,
) -> AppResult<()> {
    let node_name = format!("{}-{}", node_type, uuid);

    let deployments: Api<Deployment> = Api::default_namespaced(state.kube_client.clone());
    deployments.delete(&node_name, &DeleteParams::default()).await?;

    let services: Api<Service> = Api::default_namespaced(state.kube_client.clone());
    services.delete(&node_name, &DeleteParams::default()).await?;

    sqlx::query("DELETE FROM node WHERE id = $1")
        .bind(uuid)
        .execute(pool)
        .await?;

    Ok(())
}
