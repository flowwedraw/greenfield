mod config;
mod jito_client;
mod liquidity_checker;
mod strategy;
mod telegram_listener;
mod tx_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize logging
    tracing_subscriber::fmt::init();

    // load configuration
    let _config = config::Config::from_env()?;

    println!("Sniper bot skeleton running");
    Ok(())
}
