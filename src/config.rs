use anyhow::Result;
use dotenvy::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

pub struct Config {
    pub api_key: String,
    pub entity_secret: String,
    pub public_key: String,
}

#[derive(Deserialize)]
struct PublicKeyResponse {
    data: PublicKeyData,
}

#[derive(Deserialize)]
struct PublicKeyData {
    #[serde(rename = "publicKey")]
    public_key: String,
}

pub async fn arc_config(client: &Client) -> Result<Config> {
    dotenv().ok();

    let api_key = env::var("CIRCLE_API_KEY")?;
    let entity_secret = env::var("ENTITY_SECRET")?;

    let response = client
        .get("https://api.circle.com/v1/w3s/config/entity/publicKey")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    let res: PublicKeyResponse = response.json().await?;

    let config = Config {
        api_key,
        entity_secret,
        public_key: res.data.public_key,
    };

    Ok(config)
}
