<<<<<<< HEAD
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
=======
use axum::{Extension, Json, extract::Path};
use sqlx::PgPool;
use uuid::Uuid;

use super::User;
use crate::{
    AppState,
    error::{AppError, AppResult},
};

pub async fn get(
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = get_user_by_id(&app_state.pool, &uuid).await?;
    Ok(Json(user))
}

async fn get_user_by_id(pool: &PgPool, uuid: &Uuid) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(r#"SELECT id, email, username FROM "user" WHERE id = $1"#)
        .bind(uuid)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user)
}
>>>>>>> main
