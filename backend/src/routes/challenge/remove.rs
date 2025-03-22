use axum::Json;

use crate::error::AppResult;

pub async fn remove() -> AppResult<Json<()>> {
    Ok(Json(()))
}
