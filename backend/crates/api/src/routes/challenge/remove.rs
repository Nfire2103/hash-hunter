use axum::{Extension, extract::Path};
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
};

#[utoipa::path(
    delete,
    path = "/challenge/{uuid}",
    responses(
        (status = 200, description = "Challenge deleted successfully"),
        (status = 404, description = "Challenge not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Challenges"
)]
pub async fn remove(Extension(app_state): Extension<AppState>, Path(uuid): Path<Uuid>) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM challenge WHERE id = $1")
        .bind(uuid)
        .execute(&app_state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
