use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::arc_config;
use crate::encrypt::ciphertext;

mod encrypt;
mod config;


#[derive(Serialize)]
struct CreateWalletSetRequest {
    idempotencyKey: String,
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
    walletSet: WalletSet,
}

#[derive(Deserialize, Debug)]
struct WalletSet {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    let config = arc_config().await?;

    let ciphertext: String = ciphertext(&config.public_key, &config.entity_secret)?;

    let client = Client::new();



    let body = CreateWalletSetRequest {
        idempotencyKey: Uuid::new_v4().to_string(),

        name: "main-wallet-set".to_string(),

        entity_secret_ciphertext: ciphertext,
    };

    let response = client
        .post("https://api.circle.com/v1/w3s/developer/walletSets")
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .json(&body)
        .send()
        .await?;

    let text = response.text().await?;

    println!("{}", text);

    Ok(())
}
