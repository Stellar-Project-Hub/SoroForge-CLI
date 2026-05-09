use anyhow::Result;
use clap::Args;
use soroforge_sandbox::Sandbox;

#[derive(Args)]
pub struct SandboxArgs {
    /// Port to expose the sandbox RPC on
    #[arg(long, default_value_t = 8000)]
    pub port: u16,
    /// Stop a running sandbox instead of starting one
    #[arg(long)]
    pub stop: bool,
}

pub async fn run(args: SandboxArgs) -> Result<()> {
    let sandbox = Sandbox::new(args.port);
    if args.stop {
        sandbox.stop().await
    } else {
        sandbox.start().await
    }
}
