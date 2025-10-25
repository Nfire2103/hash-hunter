use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::{challenge::ChallengeApiDoc, node::NodeApiDoc, rpc::RpcApiDoc, user::UserApiDoc};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "HashHunter API",
    ),
    servers(
        (url = "http://localhost:5555", description = "Development server")
    )
)]
pub struct ApiDoc;

pub fn get_doc_router() -> SwaggerUi {
    let mut merged_doc = ApiDoc::openapi();

    merged_doc.merge(UserApiDoc::openapi());
    merged_doc.merge(NodeApiDoc::openapi());
    merged_doc.merge(ChallengeApiDoc::openapi());
    merged_doc.merge(RpcApiDoc::openapi());

    SwaggerUi::new("/docs").url("/api-docs/openapi.json", merged_doc)
}
