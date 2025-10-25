mod ethereum;
pub mod rpc;
mod solana;

use anyhow::Result;
use async_trait::async_trait;
use ethereum::EthereumProvider;
use reqwest::Url;
use rpc::RpcRequest;
use serde::{Deserialize, Serialize};
use solana::SolanaProvider;
use sqlx::Type;
use strum::Display;
use utoipa::ToSchema;

use crate::error::AppResult;

#[derive(Clone, Copy, Serialize, Type, Display, ToSchema)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
#[sqlx(type_name = "node_type", rename_all = "lowercase")]
pub enum NodeType {
    Anvil,
    Solana,
}

impl NodeType {
    pub fn filter_methods(&self, req: &RpcRequest) -> AppResult<()> {
        match self {
            Self::Anvil => ethereum::filter_methods(req),
            Self::Solana => unimplemented!("Solana filter methods not implemented"),
        }
    }
}

impl From<BlockchainType> for NodeType {
    fn from(blockchain: BlockchainType) -> Self {
        match blockchain {
            BlockchainType::Ethereum => Self::Anvil,
            BlockchainType::Solana => Self::Solana,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Type, ToSchema)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "blockchain_type", rename_all = "lowercase")]
pub enum BlockchainType {
    Ethereum,
    Solana,
}

impl BlockchainType {
    pub fn provider(&self, rpc_url: Url) -> Box<dyn BlockchainProvider> {
        match self {
            Self::Ethereum => Box::new(EthereumProvider::new(rpc_url)),
            Self::Solana => Box::new(SolanaProvider::new(rpc_url)),
        }
    }
}

impl From<NodeType> for BlockchainType {
    fn from(node: NodeType) -> Self {
        match node {
            NodeType::Anvil => Self::Ethereum,
            NodeType::Solana => Self::Solana,
        }
    }
}

#[async_trait]
pub trait BlockchainProvider: Send + Sync {
    fn player_wallet(&self) -> (&'static str, &'static str);
    async fn create_level(&self, bytecode: &str) -> Result<String>;
    async fn create_instances(&self, level: &str, value: &str) -> Result<Vec<String>>;
    async fn validate_instances(&self, level: &str, instances: &[String]) -> Result<bool>;
    async fn exploit_instances(&self, instances: &[String], bytecode: &str, value: &str) -> Result<()>;
}
