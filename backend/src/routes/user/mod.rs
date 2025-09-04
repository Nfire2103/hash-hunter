mod get;
mod login;
mod register;
mod remove;
mod token;
mod update;

use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::middlewares::user::check_is_curr_user;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct UserWithToken {
    #[serde(flatten)]
    inner: User,
    token: String,
}

#[derive(FromRow)]
pub struct UserWithPassword {
    #[sqlx(flatten)]
    inner: User,
    password: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/user/{uuid}", get(get::get))
        .route("/user/{uuid}", patch(update::update))
        .route("/user/{uuid}", delete(remove::remove))
        .layer(middleware::from_fn(check_is_curr_user))
}

pub fn auth_router() -> Router {
    Router::new()
        .route("/user/register", post(register::register))
        .route("/user/login", post(login::login))
}
