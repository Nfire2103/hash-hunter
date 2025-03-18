use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::warn;

use crate::blockchain::rpc::RpcMethodDoesNotExistError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The entity does not exist")]
    NotFound,

    #[error(transparent)]
    RpcMethodDoesNotExist(#[from] RpcMethodDoesNotExistError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    KubeError(#[from] kube::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(serde::Serialize)]
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
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("Not found", self.to_string())),
            )
                .into_response(),
            _ => {
                warn!("An error occurred: {}", self);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("Internal server error", self.to_string())),
                )
                    .into_response()
            },
        }
    }
}
