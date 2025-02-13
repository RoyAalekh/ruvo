use clap::Parser;
mod cli;
mod core;
mod handlers;
mod utils;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = cli::Cli::parse();
    cli.execute().await?;
    Ok(())
}
