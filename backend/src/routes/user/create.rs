use axum::{Extension, Json};
use sqlx::PgPool;

use super::{NewUser, User, hash_password};
use crate::error::AppResult;

pub async fn create(Extension(pool): Extension<PgPool>, Json(req): Json<NewUser>) -> AppResult<Json<User>> {
    let user = create_user(&pool, req).await?;
    Ok(user)
}

pub async fn create_user(pool: &PgPool, req: NewUser) -> AppResult<Json<User>> {
    let password_hash = hash_password(req.password).await?;

    let user_id = sqlx::query_scalar(
        r#"INSERT INTO "user"
        (email, username, password)
        RETURNING id"#,
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(pool)
    .await?;

    Ok(Json(User {
        id: user_id,
        email: req.email,
        username: req.username,
    }))
}
