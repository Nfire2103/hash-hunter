mod create;
mod deploy;
mod get;
mod remove;
mod shutdown;
mod update;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/challenge", post(create::create))
        .route("/challenge/{uuid}", get(get::get))
        .route("/challenge/{uuid}", put(update::update))
        .route("/challenge/{uuid}", delete(remove::remove))
        .route("/challenge/{uuid}/deploy", post(deploy::deploy))
        .route("/challenge/{uuid}/shutdown/{rpc}", post(shutdown::shutdown))
}
