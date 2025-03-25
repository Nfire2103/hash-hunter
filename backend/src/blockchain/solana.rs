use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;

use super::BlockchainProvider;

pub struct SolanaProvider {
    _rpc_url: Url,
}

impl SolanaProvider {
    pub fn new(rpc_url: Url) -> Self {
        Self { _rpc_url: rpc_url }
    }
}

#[async_trait]
impl BlockchainProvider for SolanaProvider {
    fn player_wallet(&self) -> (&'static str, &'static str) {
        unimplemented!("Solana player wallet not implemented");
    }

    async fn create_level(&self, _bytecode: &str) -> Result<String> {
        unimplemented!("Solana create level not implemented");
    }

    async fn create_instances(&self, _level: &str, _value: &str) -> Result<Vec<String>> {
        unimplemented!("Solana create instances not implemented");
    }

    async fn validate_instances(&self, _level: &str, _instances: &[String]) -> Result<bool> {
        unimplemented!("Solana validate instances not implemented");
    }

    async fn exploit_instances(
        &self,
        _instances: &[String],
        _bytecode: &str,
        _value: &str,
    ) -> Result<()> {
        unimplemented!("Solana exploit instances not implemented");
    }
}
