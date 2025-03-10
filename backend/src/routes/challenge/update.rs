use axum::Json;

use crate::error::AppResult;

pub async fn update() -> AppResult<Json<()>> {
    Ok(Json(()))
}
