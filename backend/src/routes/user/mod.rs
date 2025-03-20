mod login;
mod register;
mod token;

use axum::{Router, routing::post};
use uuid::Uuid;

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

pub fn auth_router() -> Router {
    Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
}
