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
    #[error("You are not authenticated")]
    Unauthorized,

    #[error("You do not have permission")]
    Forbidden,

    #[error("The entity does not exist")]
    NotFound,

    #[error("This {0} is already taken")]
    Conflict(String),

    #[error("This challenge can not be deployed")]
    CannotBeDeployed,

    #[error("This challenge can not be solved")]
    CannotBeSolved,

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
            Self::CannotBeDeployed | Self::CannotBeSolved => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("Bad request", self.to_string())),
            )
                .into_response(),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("Unauthorized", self.to_string())),
            )
                .into_response(),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::new("Forbidden", self.to_string())),
            )
                .into_response(),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("Not found", self.to_string())),
            )
                .into_response(),
            Self::Conflict(_) => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new("Conflict", self.to_string())),
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

pub trait ResultExt<T> {
    fn on_constraint_conflict(self, constraint: &str) -> Result<T, AppError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<AppError>,
{
    fn on_constraint_conflict(self, constraint: &str) -> Result<T, AppError> {
        self.map_err(|err| match err.into() {
            AppError::DatabaseError(sqlx::Error::Database(dbe))
                if dbe.constraint() == Some(constraint) =>
            {
                let attribute = constraint.split('_').nth(1).unwrap_or_default();
                AppError::Conflict(attribute.to_string())
            },
            err => err,
        })
    }
}
