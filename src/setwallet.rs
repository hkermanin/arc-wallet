use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::encrypt::ciphertext;

#[derive(Serialize)]
struct CreateWalletSetRequest {
    #[serde(rename = "idempotencyKey")]
    idempotency_key: String,
    name: String,

    #[serde(rename = "entitySecretCiphertext")]
    entity_secret_ciphertext: String,
}

#[derive(Deserialize, Debug)]
struct WalletSetResponse {
    data: WalletSetData,
}

#[derive(Deserialize, Debug)]
struct WalletSetData {
    #[serde(rename = "walletSet")]
    wallet_set: WalletSet,
}

#[derive(Deserialize, Debug)]
pub struct WalletSet {
    pub id: String,
    pub name: String,
}

pub async fn create_set_wallet(name: &str, client: &Client, config: &Config) -> Result<WalletSet> {
    let ciphertext = ciphertext(&config.public_key, &config.entity_secret)?;

    let body = CreateWalletSetRequest {
        idempotency_key: Uuid::new_v4().to_string(),

        name: name.to_string(),

        entity_secret_ciphertext: ciphertext,
    };

    let response = client
        .post("https://api.circle.com/v1/w3s/developer/walletSets")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&body)
        .send()
        .await?;

    let response: WalletSetResponse = response.json().await?;

    Ok(response.data.wallet_set)
}
