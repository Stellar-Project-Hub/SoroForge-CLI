use anyhow::Result;
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct InitArgs {
    /// Project name
    pub name: String,
}

pub async fn run(args: InitArgs) -> Result<()> {
    fs::create_dir_all(format!("{}/contracts", args.name))?;
    fs::write(
        format!("{}/soroforge.toml", args.name),
        "[network]\nname = \"testnet\"\n",
    )?;
    println!("🚀 Initialized SoroForge project '{}'", args.name);
    Ok(())
}
