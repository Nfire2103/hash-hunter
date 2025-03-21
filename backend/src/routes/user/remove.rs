<<<<<<< HEAD
use axum::extract::{Extension, Path};
use uuid::Uuid;

use crate::error::AppResult;
use crate::routes::ApiContext;

#[axum::debug_handler]
pub async fn remove(
    ctx: Extension<ApiContext>,
    Path(uuid): Path<Uuid>,
) -> AppResult<()> {
    sqlx::query(
        r#"DELETE FROM "user"
        WHERE id = $1"#
    )
    .bind(&uuid)
    .execute(&ctx.db)
    .await?;
=======
use axum::{Extension, extract::Path};
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
};

pub async fn remove(Extension(app_state): Extension<AppState>, Path(uuid): Path<Uuid>) -> AppResult<()> {
    let result = sqlx::query(r#"DELETE FROM "user" WHERE id = $1"#)
        .bind(uuid)
        .execute(&app_state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
>>>>>>> main

    Ok(())
}
