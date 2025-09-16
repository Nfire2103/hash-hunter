mod args;
pub mod create;
mod get;
mod remove;
mod reset;
mod state;
mod validate;

pub use args::NodeArgs;
use axum::{
    Router,
    routing::{delete, get, post},
};
use chrono::NaiveDateTime;
pub use remove::remove_node;
pub use state::NodeState;
use uuid::Uuid;

use crate::blockchain::NodeType;

#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: Uuid,
    pub user_id: Uuid,
    pub challenge_id: Uuid,
    pub level: String,
    pub instances: Vec<String>,
    pub pod_name: String,
    pub pod_uid: String,
    pub r#type: NodeType,
    pub last_activity: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn router() -> Router<NodeState> {
    Router::new()
        .route("/node", post(create::create))
        .route("/node/{uuid}/reset", post(reset::reset))
        .route("/node/{uuid}/validate", post(validate::validate))
        .route("/node/{uuid}", get(get::get))
        .route("/node/{uuid}", delete(remove::remove))
}
