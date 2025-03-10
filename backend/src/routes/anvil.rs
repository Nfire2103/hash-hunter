use anyhow::anyhow;
use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    AppState,
    error::{AppError, AppResult},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/eth/{uuid}", post(call))
}

// TODO take a look at which fields can be optional
#[derive(Deserialize, Serialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Vec<Value>>,
}

// TODO take a look at wallet_getCapabilities method
// TODO take a look at wallet_sendTransaction & odyssey_sendTransaction methods
// TODO take a look at eth_sendTransaction method
async fn call(
    State(state): State<AppState>,
    Json(req): Json<RpcRequest>,
) -> AppResult<Json<Value>> {
    if req.method.starts_with("anvil_")
        || req.method.starts_with("hardhat_")
        || req.method.starts_with("evm_")
        || req.method.starts_with("ots_")
        || req.method == "eth_sendUnsignedTransaction"
    {
        return Err(AppError::RpcMethodDoesNotExist(req.into()));
    }

    let response = state
        .http_client
        .post(&state.anvil_uri)
        .json(&req)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::UnexpectedError(anyhow!(response.text().await?)));
    }

    let result = response.json().await?;

    Ok(Json(result))
}
