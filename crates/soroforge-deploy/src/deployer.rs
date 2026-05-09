use anyhow::Result;
use soroforge_core::config::NetworkConfig;
use tracing::info;

pub struct Deployer {
    pub network: NetworkConfig,
}

impl Deployer {
    pub fn new(network: NetworkConfig) -> Self {
        Self { network }
    }

    /// Upload and instantiate a compiled WASM contract.
    pub async fn deploy(&self, wasm_path: &str, contract_name: &str) -> Result<String> {
        info!(
            "Deploying '{}' to {} ({})",
            contract_name, self.network.name, self.network.rpc_url
        );
        // TODO: invoke stellar-sdk upload + instantiate
        let _ = wasm_path;
        let contract_id = "PLACEHOLDER_CONTRACT_ID".to_string();
        Ok(contract_id)
    }
}
