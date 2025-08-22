#![allow(dead_code)]
pub struct Position {
    pub token: String,
    pub amount: u64,
    pub entry_price: f64,
    pub take_profit: f64,
    pub stop_loss: f64,
}

pub async fn execute() -> anyhow::Result<()> {
    // TODO: implement strategy logic
    Ok(())
}
