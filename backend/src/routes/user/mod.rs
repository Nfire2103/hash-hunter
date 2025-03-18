use crate::config::Config;
use sqlx::{prelude::FromRow, PgPool};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use anyhow::Context;

mod create;
mod get;
mod remove;
mod update;

use axum::{
    Router,
    routing::{get, post, put, delete},
};

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBody {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    email: String,
    username: String,
    password: String
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
struct UpdateUser {
    id: Option<Uuid>,
    email: Option<String>,
    username: Option<String>,
    password: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String
}

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user", post(create::create))
        .route("/user/{uuid}", get(get::get))
        .route("/user/{uuid}", put(update::update))
        .route("/user/{uuid}", delete(remove::remove))
}

async fn hash_password(password: String) -> Result<String, anyhow::Error> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, &salt)
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}