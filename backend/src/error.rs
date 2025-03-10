use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::warn;

use crate::routes::anvil::RpcRequest;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    RpcMethodDoesNotExist(#[from] RpcMethodDoesNotExistError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub type RpcMethodDoesNotExistError = RpcError;

impl From<RpcRequest> for RpcMethodDoesNotExistError {
    fn from(req: RpcRequest) -> Self {
        Self {
            jsonrpc: req.jsonrpc,
            id: req.id,
            error: RpcErrorDetail {
                code: -32601,
                message: "Method not found".to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize, Error)]
pub struct RpcError {
    jsonrpc: String,
    id: u64,
    error: RpcErrorDetail,
}

#[derive(Debug, Serialize)]
struct RpcErrorDetail {
    code: i32,
    message: String,
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"jsonrpc\":\"{}\",\"id\":{},\"error\":{}}}",
            self.jsonrpc, self.id, self.error
        )
    }
}

impl std::fmt::Display for RpcErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"code\":{},\"message\":\"{}\"}}",
            self.code, self.message
        )
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    description: String,
}

impl ErrorResponse {
    fn new(error: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            description: description.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::RpcMethodDoesNotExist(err) => (StatusCode::OK, err.to_string()).into_response(),
            _ => {
                warn!("An error occurred: {}", self);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new(
                        "Internal server error",
                        self.to_string(),
                    )),
                )
                    .into_response()
            },
        }
    }
}
