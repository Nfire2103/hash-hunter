use axum::{Extension, Json,};
use crate::error::AppResult;

use super::{ApiContext, NewUser, User, UserBody, hash_password};

#[axum::debug_handler]
pub async fn create(
    ctx: Extension<ApiContext>,
    Json(req): Json<NewUser>,
) -> AppResult<Json<UserBody>> {

    let password_hash = hash_password(req.password).await?;

    let user_id = sqlx::query_scalar(
        r#"INSERT INTO "user"
        (email, username, password)
        RETURNING id"#
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserBody {
        user: User {
            id: user_id,
            email: req.email,
            username: req.username
    }}))
}
