mod ethereum;
pub mod rpc;
mod solana;

use anyhow::Result;
use async_trait::async_trait;
use ethereum::EthereumProvider;
use reqwest::Url;
use rpc::RpcRequest;
use solana::SolanaProvider;

use crate::error::AppResult;

#[derive(Clone, serde::Serialize, sqlx::Type, strum::Display)]
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
            NodeType::Anvil => ethereum::filter_methods(req),
            NodeType::Solana => unimplemented!("Solana filter methods not implemented"),
        }
    }
}

impl From<&NodeType> for BlockchainType {
    fn from(node: &NodeType) -> Self {
        match node {
            NodeType::Anvil => BlockchainType::Ethereum,
            NodeType::Solana => BlockchainType::Solana,
        }
    }
}

impl From<&BlockchainType> for NodeType {
    fn from(blockchain: &BlockchainType) -> Self {
        match blockchain {
            BlockchainType::Ethereum => NodeType::Anvil,
            BlockchainType::Solana => NodeType::Solana,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "blockchain_type", rename_all = "lowercase")]
pub enum BlockchainType {
    Ethereum,
    Solana,
}

impl BlockchainType {
    pub fn provider(&self, rpc_url: Url) -> Box<dyn BlockchainProvider> {
        match self {
            BlockchainType::Ethereum => Box::new(EthereumProvider::new(rpc_url)),
            BlockchainType::Solana => Box::new(SolanaProvider::new(rpc_url)),
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
