use axum::{Extension, extract::Path};
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
};

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
