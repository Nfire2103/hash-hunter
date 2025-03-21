use alloy::transports::BoxFuture;
use anyhow::Context;
use axum::{
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation};
use reqwest::header::AUTHORIZATION;
use tower_http::auth::AsyncAuthorizeRequest;
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthUserClaims {
    pub user_id: Uuid,
    pub exp: i64,
}

#[derive(Clone, Copy)]
pub struct TokenAuth;

impl<B> AsyncAuthorizeRequest<B> for TokenAuth
where
    B: Send + 'static,
{
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        Box::pin(async {
            match check_auth(&request) {
                Ok(user_id) => {
                    request.extensions_mut().insert(user_id);
                    Ok(request)
                },
                Err(err) => Err(err.into_response()),
            }
        })
    }
}

fn check_auth<B>(request: &Request<B>) -> AppResult<Uuid> {
    let app_state: &AppState = request
        .extensions()
        .get()
        .context("AppState was not added as an extension")?;

    let header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?;

    let bearer = header.to_str().map_err(|_| AppError::Unauthorized)?;

    let token = bearer.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;

    let claims: AuthUserClaims = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(app_state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?
    .claims;

    Ok(claims.user_id)
}
