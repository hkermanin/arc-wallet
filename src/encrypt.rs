use rsa::{Oaep, RsaPublicKey, pkcs8::DecodePublicKey};
use sha2::Sha256;
use reqwest::Client;
use anyhow::Result;
use serde::{Deserialize};

#[derive(Deserialize)]
struct PublicKeyResponse {
    data: PublicKeyData,
}

#[derive(Deserialize)]
struct PublicKeyData {
    #[serde(rename = "publicKey")]
    public_key: String,
}


pub async fn ciphertext(api_key: &str, entity_secret: &str) -> Result<String>{
    let client = Client::new();
     let response = client
        .get("https://api.circle.com/v1/w3s/config/entity/publicKey")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    let res: PublicKeyResponse = response.json().await?;

    let ciphertext = encrypt_entity_secret(&res.data.public_key, entity_secret)?;



    Ok(ciphertext)

}

fn encrypt_entity_secret(public_key_pem: &str, entity_secret: &str) -> anyhow::Result<String> {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)?;

    let mut rng = rand::thread_rng();

    let secret_bytes = hex::decode(entity_secret)?;

    let encrypted =
        public_key.encrypt(&mut rng, Oaep::new::<Sha256>(), &secret_bytes)?;

    Ok(base64::encode(encrypted))
}
