use axum::extract::{Extension, Path};
use crate::{error::AppResult, routes::challenge::{ApiContext, Challenge}};
use uuid::Uuid;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeBody {
    pub challenge: Challenge,
}

#[axum::debug_handler]
pub async fn remove(
    ctx: Extension<ApiContext>,
    Path(uuid): Path<Uuid>,
) -> AppResult<()> {
    sqlx::query(
        "DELETE FROM challenge
        WHERE id = $1"
    )
    .bind(&uuid)
    .execute(&ctx.db)
    .await?;

    Ok(())
}
