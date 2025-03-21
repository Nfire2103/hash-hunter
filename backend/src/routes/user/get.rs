use uuid::Uuid;
use axum::{extract::{Extension, Path}, Json};
use sqlx::PgPool;

use crate::{error::AppResult, error::AppError};

use super::User;

pub async fn get(Extension(pool): Extension<PgPool>, Path(uuid): Path<Uuid>) -> AppResult<Json<User>> {
    let user = get_user(&pool, &uuid).await?;
    Ok(user)
}

pub async fn get_user(
    pool: &PgPool,
    uuid: &Uuid,
) -> AppResult<Json<User>> {

    let user = sqlx::query_as(
        r#"SELECT
        (id, email, username, password)
        FROM "user"
        WHERE id = $1"#
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::SqlxRowNotFound)?;

    Ok(Json(user))
}
