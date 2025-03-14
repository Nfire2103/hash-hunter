use axum::extract::{Extension, Path};
use crate::error::AppResult;
use uuid::Uuid;

use super::ApiContext;

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

    Ok(())
}
