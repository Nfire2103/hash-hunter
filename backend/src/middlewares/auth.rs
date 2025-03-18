use axum::{body::Body, http::Request, middleware::Next, response::Response};
use uuid::uuid;

use crate::error::AppResult;

pub async fn authenticate(mut req: Request<Body>, next: Next) -> AppResult<Response> {
    req.extensions_mut()
        .insert(uuid!("e9656288-02c7-11f0-8a2e-e3df77a621bb"));

    Ok(next.run(req).await)
}
