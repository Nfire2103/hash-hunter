use axum::{
    Extension,
    body::Body,
    extract::{Path, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

pub async fn is_current_user(
    Extension(user_id): Extension<Uuid>,
    Path(uuid): Path<Uuid>,
    req: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    if user_id != uuid {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(req).await)
}
