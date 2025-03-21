use axum::Router;
use tower_http::{add_extension::AddExtensionLayer, auth::AsyncRequireAuthorizationLayer};

use crate::{
<<<<<<< HEAD
    middlewares::auth,
    routes::{challenge, user, node, rpc},
    state::AppState,
=======
    AppState,
    middlewares::auth::TokenAuth,
    routes::{
        challenge,
        node::{self, NodeState},
        rpc, user,
    },
>>>>>>> main
};

pub fn build(app_state: AppState, node_state: NodeState) -> Router {
    Router::new()
        .merge(user::router())
        .merge(challenge::router())
<<<<<<< HEAD
        .merge(node::router().with_state(app_state.node_state))
        .merge(rpc::router().with_state(app_state.http_client.clone()))
        .merge(user::router().with_state(app_state.http_client))
        .layer(middleware::from_fn(auth::authenticate))
        .layer(AddExtensionLayer::new(app_state.pool))
        // .route("/register", post(user::register::register))
        // .route("/login", post(user::login::login))
=======
        .merge(node::router().with_state(node_state))
        .layer(AsyncRequireAuthorizationLayer::new(TokenAuth))
        .merge(rpc::router().with_state(reqwest::Client::new()))
        .merge(user::auth_router())
        .layer(AddExtensionLayer::new(app_state))
>>>>>>> main
    // .layer(CorsLayer::permissive()) // TODO configure cors properly
}
