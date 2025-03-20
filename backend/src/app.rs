use axum::Router;
use tower_http::{add_extension::AddExtensionLayer, auth::AsyncRequireAuthorizationLayer};

use crate::{
    AppState,
    middlewares::auth::TokenAuth,
    routes::{
        challenge,
        node::{self, NodeState},
        rpc, user,
    },
};

pub fn build(app_state: AppState, node_state: NodeState) -> Router {
    Router::new()
        .merge(user::router())
        .merge(challenge::router())
        .merge(node::router().with_state(node_state))
        .layer(AsyncRequireAuthorizationLayer::new(TokenAuth))
        .merge(rpc::router().with_state(reqwest::Client::new()))
        .merge(user::auth_router())
        .layer(AddExtensionLayer::new(app_state))
    // .layer(CorsLayer::permissive()) // TODO configure cors properly
}
