use axum::Json;

use crate::error::AppResult;

pub async fn create() -> AppResult<Json<()>> {
    Ok(Json(()))
}
