use axum::Router;
use tower_http::{add_extension::AddExtensionLayer, auth::AsyncRequireAuthorizationLayer};

#[cfg(feature = "swagger")]
use crate::docs::get_doc_router;
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
    let router = Router::new()
        .merge(user::router())
        .merge(challenge::router().with_state(node_state.clone()))
        .merge(node::router().with_state(node_state.clone()))
        .merge(rpc::router().with_state(node_state.http_client));

    #[cfg(not(feature = "dev"))]
    let router = router.layer(AsyncRequireAuthorizationLayer::new(TokenAuth));

    #[cfg(feature = "dev")]
    let router = router.layer(axum::Extension::<uuid::Uuid>(uuid::Uuid::nil()));

    #[cfg(feature = "swagger")]
    let router = router.merge(get_doc_router());

    router
        .merge(user::auth_router())
        .layer(AddExtensionLayer::new(app_state))
}
