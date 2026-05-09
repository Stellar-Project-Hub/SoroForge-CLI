use anyhow::Result;
use clap::Args;
use soroforge_core::config::NetworkConfig;
use soroforge_deploy::Deployer;

#[derive(Args)]
pub struct DeployArgs {
    /// Path to the compiled .wasm file
    #[arg(short, long)]
    pub wasm: String,
    /// Human-readable contract name
    #[arg(short, long)]
    pub name: String,
    /// Target network: testnet | mainnet
    #[arg(long, default_value = "testnet")]
    pub network: String,
}

pub async fn run(args: DeployArgs) -> Result<()> {
    let network = NetworkConfig {
        name: args.network,
        ..Default::default()
    };
    let deployer = Deployer::new(network);
    let contract_id = deployer.deploy(&args.wasm, &args.name).await?;
    println!("✅ Deployed '{}' → Contract ID: {}", args.name, contract_id);
    Ok(())
}
