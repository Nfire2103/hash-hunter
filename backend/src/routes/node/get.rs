use axum::{Extension, Json, extract::Path};
use sqlx::PgPool;
use uuid::Uuid;

use super::Node;
use crate::error::{AppError, AppResult};

pub async fn get(Extension(pool): Extension<PgPool>, Path(uuid): Path<Uuid>) -> AppResult<Json<Node>> {
    let node = get_node(&pool, &uuid).await?;
    Ok(Json(node))
}

pub async fn get_node(pool: &PgPool, uuid: &Uuid) -> AppResult<Node> {
    let node = sqlx::query_as::<_, Node>(
        "SELECT id, user_id, challenge_id, level, instances, type, pod_name,
        pod_uid, last_activity, created_at, updated_at FROM node WHERE id = $1",
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(node)
}
