use axum::{Extension, Json, extract::Path};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{User, register::hash_password};
use crate::{
    AppState,
    error::{AppError, AppResult, ResultExt},
};

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/user/{uuid}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 404, description = "User not found"),
        (status = 400, description = "Invalid request data or constraint conflict"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    ),
    tag = "Users"
)]
pub async fn update(
    Extension(app_state): Extension<AppState>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let password_hash = if let Some(password) = req.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query_as::<_, User>(
        r#"UPDATE "user"
        SET email = COALESCE($1, "user".email),
            username = COALESCE($2, "user".username),
            password = COALESCE($3, "user".password)
        WHERE id = $4
        RETURNING id, email, username"#,
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .bind(uuid)
    .fetch_optional(&app_state.pool)
    .await
    .on_constraint_conflict("user_username_key")
    .on_constraint_conflict("user_email_key")?
    .ok_or(AppError::NotFound)?;

    Ok(Json(user))
}
