use axum::{Extension, Json,};
use crate::error::AppResult;
use sqlx::PgPool;

use super::{UpdateUser, User, hash_password};


pub async fn update(Extension(pool): Extension<PgPool>, Json(req): Json<UpdateUser>) -> AppResult<Json<User>> {
    let challenge = update_user(&pool, req).await?;
    Ok(challenge)
}

pub async fn update_user(
    pool: &PgPool,
    req: UpdateUser,
) -> AppResult<Json<User>> {

    let password_hash = if let Some(password) = req.password {
        Some(hash_password(password).await?)
    } else {
        None
    };


    let user = sqlx::query_as::<_, User>(
        r#"UPDATE "user"
        SET email = coalesce($1, email),
            username = coalesce($2, username),
            password = coalesce($3, password)
        WHERE id = $4
        RETURNING id, email, username"#
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(password_hash)
    .bind(&req.id)
    .fetch_one(pool)
    .await?;

    Ok(Json(user))
}

