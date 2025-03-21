use axum::{Router, middleware};
use tower_http::add_extension::AddExtensionLayer;

use crate::{
    middlewares::auth,
    routes::{challenge, user, node, rpc},
    state::AppState,
};

pub fn build(app_state: AppState) -> Router {
    Router::new()
        // .merge(user::router())
        .merge(challenge::router())
        .merge(node::router().with_state(app_state.node_state))
        .merge(rpc::router().with_state(app_state.http_client.clone()))
        .merge(user::router().with_state(app_state.http_client))
        .layer(middleware::from_fn(auth::authenticate))
        .layer(AddExtensionLayer::new(app_state.pool))
        // .route("/register", post(user::register::register))
        // .route("/login", post(user::login::login))
    // .layer(CorsLayer::permissive()) // TODO configure cors properly
}
