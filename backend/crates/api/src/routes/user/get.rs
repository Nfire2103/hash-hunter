use axum::{Extension, Json, extract::Path};
use sqlx::PgPool;
use uuid::Uuid;

use super::User;
use crate::{
    AppState,
    error::{AppError, AppResult},
    routes::user::UserWithPassword,
};

pub async fn get(
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = get_user_by_id(&app_state.pool, uuid).await?;
    Ok(Json(user))
}

async fn get_user_by_id(pool: &PgPool, uuid: Uuid) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(r#"SELECT id, email, username FROM "user" WHERE id = $1"#)
        .bind(uuid)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> AppResult<UserWithPassword> {
    let user = sqlx::query_as::<_, UserWithPassword>(
        r#"SELECT id, email, username, password FROM "user" WHERE email = $1"#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    Ok(user)
}
