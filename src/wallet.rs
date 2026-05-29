use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::encrypt::ciphertext;

#[derive(Serialize)]
struct CreateWalletRequest {
    #[serde(rename = "idempotencyKey")]
    idempotency_key: String,

    #[serde(rename = "walletSetId")]
    wallet_set_id: String,

    blockchains: Vec<String>,

    count: u32,

    #[serde(rename = "accountType")]
    account_type: String,

    #[serde(rename = "entitySecretCiphertext")]
    entity_secret_ciphertext: String,
}

#[derive(Deserialize, Debug)]
struct CreateWalletResponse {
    data: WalletData,
}

#[derive(Deserialize, Debug)]
struct WalletData {
    wallets: Vec<Wallet>,
}

#[derive(Deserialize, Debug)]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub blockchain: String,
}

pub async fn create_wallet(
    wallet_set_id: &str,
    client: &Client,
    config: &Config,
) -> Result<Wallet> {
    let entity_secret_ciphertext = ciphertext(&config.public_key, &config.entity_secret)?;

    let body = CreateWalletRequest {
        idempotency_key: Uuid::new_v4().to_string(),

        wallet_set_id: wallet_set_id.to_string(),

        blockchains: vec!["ARC-TESTNET".to_string()],

        count: 1,

        account_type: "EOA".to_string(),

        entity_secret_ciphertext,
    };

    let response = client
        .post("https://api.circle.com/v1/w3s/developer/wallets")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&body)
        .send()
        .await?;

    let text = response.text().await?;

    let response: CreateWalletResponse = serde_json::from_str(&text)?;

    let wallet = response
        .data
        .wallets
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("no wallet returned"))?;

    Ok(wallet)
}
