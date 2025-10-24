use anyhow::anyhow;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::post,
};
use serde_json::Value;
#[cfg(feature = "swagger")]
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    AppState,
    blockchain::{NodeType, rpc::RpcRequest},
    error::{AppError, AppResult},
};

pub fn router() -> Router<reqwest::Client> {
    Router::new().route("/rpc/{uuid}", post(call))
}

#[utoipa::path(
    post,
    path = "/rpc/{uuid}",
    request_body = RpcRequest,
    responses(
        (status = 200, description = "RPC call successful", body = Value),
        (status = 404, description = "Node not found"),
        (status = 400, description = "Invalid RPC method or request"),
        (status = 500, description = "Internal server error or node communication failure")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "RPC"
)]
async fn call(
    Extension(app_state): Extension<AppState>,
    State(http_client): State<reqwest::Client>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<RpcRequest>,
) -> AppResult<Json<Value>> {
    let node_type = sqlx::query_scalar::<_, NodeType>(
        "UPDATE node SET last_activity = NOW() WHERE id = $1
        RETURNING type",
    )
    .bind(uuid)
    .fetch_optional(&app_state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    node_type.filter_methods(&req)?;

    let response = http_client
        .post(format!("http://{node_type}-{uuid}"))
        .json(&req)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::UnexpectedError(anyhow!(response.text().await?)));
    }

    let result = response.json().await?;

    Ok(Json(result))
}

#[cfg(feature = "swagger")]
#[derive(OpenApi)]
#[openapi(
    paths(
        call
    ),
    components(
        schemas()
    ),
    tags(
        (name = "RPC", description = "RPC calls to blockchain nodes")
    )
)]
pub struct RpcApiDoc;
