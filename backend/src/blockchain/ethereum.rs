use std::str::FromStr;

use alloy::{
    hex,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, TxHash, U256},
    providers::{
        DynProvider, Provider, ProviderBuilder,
        ext::{AnvilApi, DebugApi},
    },
    rpc::types::{TransactionReceipt, TransactionRequest, trace::geth::GethDebugTracingOptions},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
};
use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use reqwest::Url;
use tokio::time::{Duration, sleep};

use super::{BlockchainProvider, RpcRequest};
use crate::error::{AppError, AppResult};

sol! {
    #[sol(rpc)]
    interface Level {
        function createInstances(
            address _player
        ) external payable returns (address[] memory);

        function validateInstances(
            address _player,
            address[] calldata _instances
        ) external returns (bool);
    }

    #[sol(rpc)]
    interface Exploit {
        function exploitInstances(address[] calldata _instances) external payable;
    }
}

pub struct EthereumProvider {
    provider: DynProvider,
}

impl EthereumProvider {
    pub fn new(rpc_url: Url) -> Self {
        Self {
            provider: ProviderBuilder::new().on_http(rpc_url).erased(),
        }
    }
}

#[async_trait]
impl BlockchainProvider for EthereumProvider {
    fn player_wallet(&self) -> (&'static str, &'static str) {
        const PUBLIC_KEY: &str = "0xAEBbAfC34E4B4Eb412bb30936CCF3B46b38fD3f6";
        const PRIVATE_KEY: &str = "0xa7acaaadefc63d39ec8abf16e10d476792f60d00c7fa7350382d1bcbc21010e1";

        (PUBLIC_KEY, PRIVATE_KEY)
    }

    async fn create_level(&self, bytecode: &str) -> Result<String> {
        let tx = TransactionRequest::default()
            .with_from(Address::default())
            .with_deploy_code(hex::decode(bytecode)?)
            .with_max_fee_per_gas(0)
            .with_max_priority_fee_per_gas(0);

        let tx_hash = self.provider.eth_send_unsigned_transaction(tx).await?;
        let tx_receipt = get_transaction_receipt_with_retry(&self.provider, tx_hash, 50).await?;

        let level_addr = tx_receipt
            .contract_address
            .ok_or_else(|| anyhow!("Failed to get level address"))?;

        Ok(level_addr.to_string())
    }

    async fn create_instances(&self, level: &str, value: &str) -> Result<Vec<String>> {
        let (player, _) = self.player_wallet();

        let player_addr = Address::from_str(player)?;
        let level_addr = Address::from_str(level)?;
        let value_u256 = U256::from_str(value)?;

        let level = Level::new(level_addr, &self.provider);

        self.provider
            .anvil_set_balance(Address::default(), value_u256)
            .await?;

        let tx = level
            .createInstances(player_addr)
            .into_transaction_request()
            .with_from(Address::default())
            .with_value(value_u256)
            .with_max_fee_per_gas(0)
            .with_max_priority_fee_per_gas(0);

        let tx_hash = self.provider.eth_send_unsigned_transaction(tx).await?;
        let _ = get_transaction_receipt_with_retry(&self.provider, tx_hash, 50).await?;

        let debug_trace = self
            .provider
            .debug_trace_transaction(tx_hash, GethDebugTracingOptions::default())
            .await?;

        let encoded = debug_trace.try_into_default_frame()?.return_value;
        let decoded = Vec::<Address>::abi_decode(&encoded, true)?;

        let instances = decoded.iter().map(ToString::to_string).collect();

        Ok(instances)
    }

    async fn validate_instances(&self, level: &str, instances: &[String]) -> Result<bool> {
        let (player, _) = self.player_wallet();

        let player_addr = Address::from_str(player)?;
        let level_addr = Address::from_str(level)?;
        let instances_addr = instances
            .iter()
            .map(|instance| Address::from_str(instance))
            .collect::<Result<Vec<_>, _>>()?;

        let level = Level::new(level_addr, &self.provider);

        let validate = level
            .validateInstances(player_addr, instances_addr)
            .call()
            .await?
            ._0;

        Ok(validate)
    }

    async fn exploit_instances(&self, instances: &[String], bytecode: &str, value: &str) -> Result<()> {
        let (player, privatekey) = self.player_wallet();

        let player_addr = Address::from_str(player)?;
        let value_u256 = U256::from_str(value)?;
        let instances_addr = instances
            .iter()
            .map(|instance| Address::from_str(instance))
            .collect::<Result<Vec<_>, _>>()?;

        let private_signer = PrivateKeySigner::from_str(privatekey)?;
        let wallet = EthereumWallet::from(private_signer);

        let nonce = self.provider.get_transaction_count(player_addr).await?;
        let chain_id = self.provider.get_chain_id().await?;

        let tx = TransactionRequest::default()
            .with_from(player_addr)
            .with_deploy_code(hex::decode(bytecode)?)
            .with_nonce(nonce)
            .with_gas_limit(21_000_000)
            .with_chain_id(chain_id)
            .with_max_fee_per_gas(0)
            .with_max_priority_fee_per_gas(0);

        let tx_envelope = tx.build(&wallet).await?;

        let tx_receipt = self
            .provider
            .send_tx_envelope(tx_envelope)
            .await?
            .get_receipt()
            .await?;

        let exploit_addr = tx_receipt
            .contract_address
            .ok_or_else(|| anyhow!("Failed to get exploit address"))?;

        let exploit = Exploit::new(exploit_addr, &self.provider);

        let tx = exploit
            .exploitInstances(instances_addr)
            .into_transaction_request()
            .with_from(player_addr)
            .with_value(value_u256)
            .with_nonce(nonce + 1)
            .with_gas_limit(21_000_000)
            .with_chain_id(chain_id)
            .with_max_fee_per_gas(0)
            .with_max_priority_fee_per_gas(0);

        let tx_envelope = tx.build(&wallet).await?;

        self.provider.send_tx_envelope(tx_envelope).await?.watch().await?;

        Ok(())
    }
}

async fn get_transaction_receipt_with_retry(
    provider: &DynProvider,
    tx_hash: TxHash,
    max_retry: u32,
) -> Result<TransactionReceipt> {
    for _ in 0..max_retry {
        if let Some(tx_receipt) = provider.get_transaction_receipt(tx_hash).await? {
            return Ok(tx_receipt);
        }

        sleep(Duration::from_millis(100)).await;
    }

    bail!("Failed to get transaction receipt");
}

// TODO take a look at wallet_getCapabilities method
// TODO take a look at wallet_sendTransaction & odyssey_sendTransaction methods
// TODO take a look at eth_sendTransaction method
// TODO check that we filter all broken methods
pub fn filter_methods(req: &RpcRequest) -> AppResult<()> {
    if req.method.starts_with("anvil_")
        || req.method.starts_with("hardhat_")
        || req.method.starts_with("evm_")
        || req.method.starts_with("ots_")
        || req.method == "eth_sendUnsignedTransaction"
    {
        return Err(AppError::RpcMethodDoesNotExist(req.into()));
    }

    Ok(())
}
