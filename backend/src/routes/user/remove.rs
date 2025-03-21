use axum::extract::{Extension, Path};
use uuid::Uuid;
use sqlx::PgPool;

use crate::error::AppResult;


pub async fn remove(Extension(pool): Extension<PgPool>, Path(uuid): Path<Uuid>) -> AppResult<()> {
    remove_user(&pool, &uuid).await?;
    Ok(())
}

pub async fn remove_user(
    pool: &PgPool,
    uuid: &Uuid,
) -> AppResult<()> {
    sqlx::query(
        r#"DELETE FROM "user"
        WHERE id = $1"#
    )
    .bind(&uuid)
    .execute(pool)
    .await?;

    Ok(())
}
