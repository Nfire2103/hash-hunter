use crate::{routes::anvil, state::AppState};
use axum::Router;

pub fn build(app_state: AppState) -> Router {
    Router::new()
        // .merge(user::router())
        // .merge(challenge::router())
        // .layer(middleware::from_fn(auth::authenticate))
        .merge(anvil::router())
        // .route("/register", post(user::register::register))
        // .route("/login", post(user::login::login))
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .layer(CorsLayer::permissive()) // TODO configure the cors properly
        .with_state(app_state)
}
