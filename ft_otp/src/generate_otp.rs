use aes_gcm::Aes256Gcm;
use aes_gcm::aead::Nonce;

use crate::errors::FtOtpError;
use crate::password::{self, build_aes_key, decrypt_file};
use std::fs::File;
use std::io::Read;

pub fn generate_otp(mut file: File) -> Result<String, FtOtpError> {
    let password = password::retreive_password(false)?;
    let mut to_decrypt: Vec<u8> = Vec::new();
    file.read_to_end(&mut to_decrypt)?;

    let (salt, nonce) = retreive_salt_and_nonce(&to_decrypt)?;
    let aes_key = build_aes_key(password, &salt)?;
    let decrypted_key = decrypt_file(&to_decrypt[28..], aes_key, nonce)?;

    // real decryption process

    Ok(String::new())
}

pub fn retreive_salt_and_nonce(file: &Vec<u8>) -> Result<([u8; 16], Nonce<Aes256Gcm>), FtOtpError> {
    if file.len() < 16 + 12 + 16 {
        return Err(FtOtpError::CorruptedKeyFile);
    }
    let salt: [u8; 16] = file[0..16].try_into()?;
    let nonce: Nonce<Aes256Gcm> = file[16..28].try_into()?;
    Ok((salt, nonce))
}
