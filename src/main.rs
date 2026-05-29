use anyhow::Result;
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::encrypt::ciphertext;

mod encrypt;



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
    dotenv().ok();

    let api_key = env::var("CIRCLE_API_KEY")?;
    let entity_secret = env::var("ENTITY_SECRET")?;


    let client = Client::new();

    let ciphertext = ciphertext(&api_key, &entity_secret).await?;

    println!("{:?}", ciphertext);

    let body = CreateWalletSetRequest {
        idempotencyKey: Uuid::new_v4().to_string(),

        name: "main-wallet-set".to_string(),

        entity_secret_ciphertext: ciphertext,
    };

    let response = client
        .post("https://api.circle.com/v1/w3s/developer/walletSets")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    let text = response.text().await?;

    println!("{}", text);

    Ok(())
}
