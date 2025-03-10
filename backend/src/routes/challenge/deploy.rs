use axum::Json;

use crate::error::AppResult;

pub async fn deploy() -> AppResult<Json<()>> {
    Ok(Json(()))
}
