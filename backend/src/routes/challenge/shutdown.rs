use axum::Json;

use crate::error::AppResult;

pub async fn shutdown() -> AppResult<Json<()>> {
    Ok(Json(()))
}
