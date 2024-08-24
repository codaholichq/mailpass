use mailpass::run_server;

use anyhow::{Context, Result};
use configured::Configured;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {:?}", e);
        std::process::exit(1);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    run_server: mailpass::Config,
}

async fn run() -> Result<()> {
    let config = Config::load().context("load configuration")?;

    run_server(config.run_server)
        .await
        .context("Failed to run server")
}
