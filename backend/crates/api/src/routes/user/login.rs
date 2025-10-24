use anyhow::anyhow;
use argon2::{Argon2, PasswordHash, password_hash::Error};
use axum::{Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use super::{UserWithToken, token::create_token};
use crate::{
    AppState,
    error::{AppError, AppResult},
    routes::user::get::get_user_by_email,
};

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "/user/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = UserWithToken),
        (status = 401, description = "Unauthorized: Invalid credentials"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Users"
)]
pub async fn login(
    Extension(state): Extension<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<UserWithToken>> {
    let user = get_user_by_email(&state.pool, &req.email).await?;

    verify_password(req.password, user.password.clone()).await?;

    Ok(Json(UserWithToken {
        token: create_token(user.inner.id, &state.jwt_secret)?,
        inner: user.inner,
    }))
}

async fn verify_password(password: String, password_hash: String) -> AppResult<()> {
    let hash = move || {
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
