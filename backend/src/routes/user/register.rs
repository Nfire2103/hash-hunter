use anyhow::{Result, anyhow};
use argon2::{
    Argon2, PasswordHash,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Extension, Json};
use serde::Deserialize;

use super::{User, UserWithToken, token::create_token};
use crate::{
    AppState,
    error::{AppResult, ResultExt},
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<UserWithToken>> {
    let password_hash = hash_password(req.password).await?;

    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO "user" (email, username, password)
        VALUES ($1, $2, $3) RETURNING id, email, username"#,
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await
    .on_constraint_conflict("user_username_key")
    .on_constraint_conflict("user_email_key")?;

    Ok(Json(UserWithToken {
        token: create_token(user.id, &state.jwt_secret)?,
        inner: user,
    }))
}

pub async fn hash_password(password: String) -> Result<String> {
    let hash = move || {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|err| anyhow!(err))?
            .to_string();

        Ok(password_hash)
    };

    tokio::task::spawn_blocking(hash).await?
}
