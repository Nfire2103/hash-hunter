use axum::{
    Extension,
    body::Body,
    extract::{Path, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
    routes::challenge::get_challenge,
};

pub async fn check_curr_user_is_owner(
    Extension(user_id): Extension<Uuid>,
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
    req: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    let challenge = get_challenge(&app_state.pool, &uuid).await?;
    if user_id != challenge.author_id {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(req).await)
}
