use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

mod commands;

#[derive(Parser)]
#[command(
    name = "soroforge",
    about = "Hardhat-like ergonomics for Soroban smart contract developers",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a local Soroban sandbox
    Sandbox(commands::sandbox::SandboxArgs),
    /// Deploy a contract to testnet or mainnet
    Deploy(commands::deploy::DeployArgs),
    /// Watch contracts and hot-reload on file changes
    Watch(commands::watch::WatchArgs),
    /// Initialize a new SoroForge project
    Init(commands::init::InitArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Sandbox(args) => commands::sandbox::run(args).await,
        Commands::Deploy(args) => commands::deploy::run(args).await,
        Commands::Watch(args) => commands::watch::run(args).await,
        Commands::Init(args) => commands::init::run(args).await,
    }
}
