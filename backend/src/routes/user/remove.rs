<<<<<<< HEAD
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
