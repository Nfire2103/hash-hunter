use sqlx::prelude::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use anyhow::Context;
use crate::routes::auth;
use reqwest::Client;

mod create;
mod get;
mod remove;
mod update;

use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware
};

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

pub fn router() -> Router<Client> {
    Router::new()
        .route("/user", post(create::create))
        .route("/user/signin", post(auth::sign_in))
        .route("/user/{uuid}", get(get::get)).layer(middleware::from_fn(auth::authorize))
        .route("/user/{uuid}", put(update::update).layer(middleware::from_fn(auth::authorize)))
        .route("/user/{uuid}", delete(remove::remove).layer(middleware::from_fn(auth::authorize)))
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