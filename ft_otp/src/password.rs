use aes_gcm::{
    Aes256Gcm, Key, KeyInit,
    aead::{Aead, Generate, Nonce},
};
use argon2::Argon2;
use inquire::Password;
use rand::RngExt;

use crate::errors::FtOtpError;

// If I had time this is where I can ask for a more robust password
pub fn retreive_password() -> Result<String, FtOtpError> {
    let name = Password::new("Password for encrypted key")
        .with_display_toggle_enabled()
        .prompt()?;
    Ok(name)
}

pub fn passe_moi_le_sel() -> [u8; 16] {
    let mut rng = rand::rng();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt);
    salt
}

pub fn built_aes_key(password: String, salt: &[u8; 16]) -> Result<[u8; 32], FtOtpError> {
    let mut output = [0u8; 32];
    let argon = Argon2::default();

    argon.hash_password_into(password.as_bytes(), salt, &mut output)?;
    Ok(output)
}

pub fn generate_nonce() -> Nonce<Aes256Gcm> {
    Nonce::<Aes256Gcm>::generate()
}

pub fn encrypt_file(
    to_encrypt: Vec<u8>,
    aes_key: [u8; 32],
    nonce: Nonce<Aes256Gcm>,
) -> Result<Vec<u8>, FtOtpError> {
    let key = Key::<Aes256Gcm>::from(aes_key);
    let cipher = Aes256Gcm::new(&key);

    Ok(cipher.encrypt(&nonce, to_encrypt.as_slice())?)
}
