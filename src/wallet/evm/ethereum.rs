use alloy::{
    hex,
    network::Ethereum,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockId, BlockNumberOrTag},
};
use crate::AppError;
use std::str::FromStr;

use crate::wallet::evm::types::BlockInfo;
use crate::wallet::evm::types::ChainInfo;
use crate::wallet::evm::{chains::Chain, types::AccountInfo};

pub struct EthereumClient {
    provider: Box<dyn Provider<Ethereum>>,
    chain_id: u64,
}

impl EthereumClient {
    pub async fn new() -> Result<Self, AppError> {
        // Get RPC URL from environment or use default
        let rpc_url = std::env::var("ETHEREUM_RPC_URL")
            .unwrap_or_else(|_| "https://ethereum-sepolia-rpc.publicnode.com".to_string());

        Self::new_with_rpc(rpc_url).await
    }

    pub async fn new_with_rpc(rpc_url: String) -> Result<Self, AppError> {
        // Create provider
        let provider = ProviderBuilder::new().connect_http(
            rpc_url
                .parse()
                .map_err(|e| AppError::Config(format!("Invalid RPC URL: {}", e)))?,
        );

        // Get chain ID
        let chain_id = provider
            .get_chain_id()
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        Ok(Self {
            provider: Box::new(provider),
            chain_id,
        })
    }

    #[cfg(feature = "ssr")]
    pub async fn get_chain_info(&self) -> Result<ChainInfo, AppError> {
        // Get latest block number

        let block_number = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        // Get gas price
        let gas_price = self
            .provider
            .get_gas_price()
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        let chain = Chain::get_by_chain_id(self.chain_id).await?;

        Ok(ChainInfo {
            chain_id: self.chain_id,
            name: chain.name.clone(),
            info: chain,
            latest_block: block_number,
            gas_price: gas_price.to_string(),
        })
    }

    pub async fn get_block_info(&self, block_number: Option<u64>) -> Result<BlockInfo, AppError> {
        // Determine block ID
        let block_id = match block_number {
            Some(num) => BlockId::Number(BlockNumberOrTag::Number(num)),
            None => BlockId::Number(BlockNumberOrTag::Latest),
        };

        // Get block
        let block = self
            .provider
            .get_block(block_id)
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?
            .ok_or_else(|| AppError::Provider("Block not found".to_string()))?;

        Ok(BlockInfo {
            number: block.header.number,
            hash: format!("0x{:x}", block.header.hash),
            timestamp: block.header.timestamp,
            gas_used: block.header.gas_used.to_string(),
            gas_limit: block.header.gas_limit.to_string(),
        })
    }

    pub async fn get_account_info(&self, address: &str) -> Result<AccountInfo, AppError> {
        // Parse address
        let addr =
            Address::from_str(address).map_err(|e| AppError::InvalidAddress(e.to_string()))?;

        // Get balance
        let balance = self
            .provider
            .get_balance(addr)
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        // Get nonce
        let nonce = self
            .provider
            .get_transaction_count(addr)
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        Ok(AccountInfo {
            address: address.to_string(),
            balance: balance.to_string(),
            nonce,
        })
    }

    // Additional utility methods
    pub async fn estimate_gas(
        &self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<u128, AppError> {
        let request = alloy::rpc::types::TransactionRequest {
            from: Some(from),
            to: Some(to.into()),
            value: Some(value),
            ..Default::default()
        };

        let gas = self
            .provider
            .estimate_gas(request)
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        Ok(gas as u128)
    }

    pub async fn get_code(&self, address: &str) -> Result<String, AppError> {
        let addr =
            Address::from_str(address).map_err(|e| AppError::InvalidAddress(e.to_string()))?;

        let code = self
            .provider
            .get_code_at(addr)
            .await
            .map_err(|e| AppError::Provider(e.to_string()))?;

        Ok(format!("0x{}", hex::encode(code)))
    }
}
