use axum::Json;

use crate::error::AppResult;

pub async fn get() -> AppResult<Json<()>> {
    Ok(Json(()))
}
