use serde::{Deserialize, Serialize};

/// Top-level project configuration, loaded from `soroforge.toml`.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoroForgeConfig {
    pub network: NetworkConfig,
    pub contracts: Vec<ContractConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub name: String,
    pub rpc_url: String,
    pub network_passphrase: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            name: "testnet".into(),
            rpc_url: "https://soroban-testnet.stellar.org".into(),
            network_passphrase: "Test SDF Network ; September 2015".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    pub name: String,
    pub path: String,
}
