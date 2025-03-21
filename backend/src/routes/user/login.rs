use anyhow::anyhow;
use argon2::{Argon2, PasswordHash, password_hash::Error};
use axum::{Extension, Json};
use sqlx::PgPool;

use super::{User, UserWithToken, token::create_token};
use crate::{
    AppState,
    error::{AppError, AppResult},
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(sqlx::FromRow)]
struct UserWithPassword {
    #[sqlx(flatten)]
    inner: User,
    password: String,
}

async fn get_user_by_email(pool: &PgPool, email: &str) -> AppResult<UserWithPassword> {
    let user = sqlx::query_as::<_, UserWithPassword>(
        r#"SELECT id, email, username, password FROM "user" WHERE email = $1"#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    Ok(user)
}

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<UserWithToken>> {
    let user = get_user_by_email(&state.pool, &req.email).await?;

    verify_password(req.password, user.password.clone()).await?;

    Ok(Json(UserWithToken {
        token: create_token(user.inner.id, &state.jwt_secret)?,
        user: user.inner,
    }))
}

async fn verify_password(password: String, password_hash: String) -> AppResult<()> {
    let hash = move || -> AppResult<()> {
        let hash = PasswordHash::new(&password_hash).map_err(|err| anyhow!(err))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|err| match err {
                Error::Password => AppError::Unauthorized,
                _ => anyhow!(err).into(),
            })
    };

    tokio::task::spawn_blocking(hash)
        .await
        .map_err(|err| anyhow!(err))?
}
