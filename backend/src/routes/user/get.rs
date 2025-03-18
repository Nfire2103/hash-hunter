use uuid::Uuid;
use axum::{extract::{Extension, Path}, Json};

use crate::{error::AppResult, error::AppError};
use crate::routes::ApiContext;

use super::UserBody;

#[axum::debug_handler]
pub async fn get(
    ctx: Extension<ApiContext>,
    Path(uuid): Path<Uuid>,
) -> AppResult<Json<UserBody>> {


    let user = sqlx::query_as(
        r#"SELECT
        (id, email, username, password)
        FROM "user"
        WHERE id = $1"#
    )
    .bind(uuid)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(AppError::SqlxRowNotFound)?;

    Ok(Json(UserBody {user}))
}
