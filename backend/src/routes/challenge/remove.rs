use axum::extract::{Extension, Path};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;

pub async fn remove(Extension(pool): Extension<PgPool>, Path(uuid): Path<Uuid>) -> AppResult<()> {
    remove_challenge(&pool, &uuid).await?;
    Ok(())
}

pub async fn remove_challenge(pool: &PgPool, uuid: &Uuid) -> AppResult<()> {
    sqlx::query(
        "DELETE FROM challenge
        WHERE id = $1",
    )
    .bind(&uuid)
    .execute(pool)
    .await?;

    Ok(())
}