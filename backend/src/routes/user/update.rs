use axum::{Extension, Json,};
use crate::error::AppResult;
use crate::routes::ApiContext;

use super::{UpdateUser, UserBody, User, hash_password};


#[axum::debug_handler]
pub async fn update(
    ctx: Extension<ApiContext>,
    Json(req): Json<UpdateUser>,
) -> AppResult<Json<UserBody>> {

    let password_hash = if let Some(password) = req.password {
        Some(hash_password(password).await?)
    } else {
        None
    };


    let user = sqlx::query_as::<_, User>(
        r#"UPDATE "user"
        SET email = coalesce($1, email),
            username = coalesce($2, username),
            password = coalesce($3, password)
        WHERE id = $4
        RETURNING id, email, username"#
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(password_hash)
    .bind(&req.id)
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserBody {user}))
}

