use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use utoipa::ToSchema;

// TODO look which fields can be optional for anvil and solana
#[derive(Deserialize, Serialize, ToSchema)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Vec<Value>>,
}

#[derive(Debug, Serialize, Error)]
pub struct RpcError {
    jsonrpc: String,
    id: u64,
    error: RpcErrorDetail,
}

#[derive(Debug, Serialize)]
struct RpcErrorDetail {
    code: i32,
    message: String,
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"jsonrpc\":\"{}\",\"id\":{},\"error\":{}}}",
            self.jsonrpc, self.id, self.error
        )
    }
}

impl std::fmt::Display for RpcErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"code\":{},\"message\":\"{}\"}}", self.code, self.message)
    }
}

pub type RpcMethodDoesNotExistError = RpcError;

impl From<&RpcRequest> for RpcMethodDoesNotExistError {
    fn from(req: &RpcRequest) -> Self {
        Self {
            jsonrpc: req.jsonrpc.clone(),
            id: req.id,
            error: RpcErrorDetail {
                code: -32601,
                message: "Method not found".to_string(),
            },
        }
    }
}
