use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct WatchArgs {
    /// Directory to watch for contract source changes
    #[arg(short, long, default_value = "contracts")]
    pub dir: String,
}

pub async fn run(args: WatchArgs) -> Result<()> {
    println!("👀 Watching '{}' for changes… (hot-reload not yet implemented)", args.dir);
    // TODO: implement notify-based watcher (see issue #7)
    Ok(())
}
