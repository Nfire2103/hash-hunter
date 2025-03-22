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
use uuid::Uuid;

use crate::middlewares::user::check_is_curr_user;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(serde::Serialize)]
pub struct UserWithToken {
    #[serde(flatten)]
    user: User,
    token: String,
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
        .route("/register", post(register::register))
        .route("/login", post(login::login))
}
