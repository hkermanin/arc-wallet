use base64::{Engine as _, engine::general_purpose};
use rsa::{Oaep, RsaPublicKey, pkcs8::DecodePublicKey};
use sha2::Sha256;

pub fn ciphertext(public_key_pem: &str, entity_secret: &str) -> anyhow::Result<String> {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)?;

    let mut rng = rand::thread_rng();

    let secret_bytes = hex::decode(entity_secret)?;

    let encrypted = public_key.encrypt(&mut rng, Oaep::new::<Sha256>(), &secret_bytes)?;

    Ok(general_purpose::STANDARD.encode(encrypted))
}
