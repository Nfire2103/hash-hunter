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

    Ok(())
}
