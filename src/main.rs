use anyhow::Result;
use reqwest::Client;

use crate::config::arc_config;
use crate::setwallet::create_set_wallet;

mod encrypt;
mod config;
mod setwallet;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let config = arc_config(&client).await?;

    let wallet_set = create_set_wallet("wallet-set", &client, &config).await?;

    println!("wallet_set_id: {}\nwallet_set_name: {}", wallet_set.id, wallet_set.name);

    Ok(())
}
