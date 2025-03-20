use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use uuid::Uuid;

use crate::middlewares::auth::AuthUserClaims;

const DEFAULT_SESSION_LENGTH: Duration = Duration::weeks(1);

pub fn create_token(user_id: Uuid, jwt_secret: &str) -> Result<String> {
    let claims = AuthUserClaims {
        user_id,
        exp: (Utc::now() + DEFAULT_SESSION_LENGTH).timestamp(),
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}
