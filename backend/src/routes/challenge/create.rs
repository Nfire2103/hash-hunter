use axum::{Extension, Json, extract::State};

use crate::{AppState, error::AppResult};

// TODO replace () by the right type
pub async fn create(
    Extension(user_id): Extension<i32>,
    State(state): State<AppState>,
    Json(req): Json<()>,
) -> AppResult<Json<()>> {
    Ok(Json(()))
}
