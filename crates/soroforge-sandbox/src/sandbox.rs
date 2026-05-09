use anyhow::Result;
use tracing::info;

/// Manages a local `stellar-quickstart` sandbox container.
pub struct Sandbox {
    pub port: u16,
}

impl Sandbox {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Start the sandbox (spawns docker container).
    pub async fn start(&self) -> Result<()> {
        info!("Starting local Soroban sandbox on port {}", self.port);
        // TODO: spawn stellar-quickstart docker container
        Ok(())
    }

    /// Stop and remove the sandbox container.
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping local Soroban sandbox");
        // TODO: stop docker container
        Ok(())
    }
}
