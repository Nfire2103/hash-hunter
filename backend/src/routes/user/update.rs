<<<<<<< HEAD
use axum::{Extension, Json,};
use crate::error::AppResult;
use crate::routes::ApiContext;

use super::{UpdateUser, UserBody, User, hash_password};


#[axum::debug_handler]
pub async fn update(
    ctx: Extension<ApiContext>,
    Json(req): Json<UpdateUser>,
) -> AppResult<Json<UserBody>> {

=======
use axum::{Extension, Json, extract::Path};
use uuid::Uuid;

use super::{User, register::hash_password};
use crate::{
    AppState,
    error::{AppError, AppResult, ResultExt},
};

#[derive(serde::Deserialize)]
pub struct UpdateUserRequest {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

pub async fn update(
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
>>>>>>> main
    let password_hash = if let Some(password) = req.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

<<<<<<< HEAD

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

=======
    let user = sqlx::query_as::<_, User>(
        r#"
            UPDATE "user"
            SET email = COALESCE($1, "user".email),
                username = COALESCE($2, "user".username),
                password = COALESCE($3, "user".password)
            WHERE id = $4
            RETURNING id, email, username
        "#,
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .bind(&uuid)
    .fetch_optional(&app_state.pool)
    .await
    .on_constraint_conflict("user_username_key")
    .on_constraint_conflict("user_email_key")?
    .ok_or(AppError::NotFound)?;

    Ok(Json(user))
}
>>>>>>> main
