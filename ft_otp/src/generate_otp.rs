use aes_gcm::Aes256Gcm;
use aes_gcm::aead::Nonce;
use sha1::digest::typenum::Xor;
use sha1::{Digest, Sha1};

use crate::errors::FtOtpError;
use crate::password::{self, build_aes_key, decrypt_file};
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

pub fn generate_otp(mut file: File) -> Result<(), FtOtpError> {
    let password = password::retreive_password(false)?;
    let mut to_decrypt: Vec<u8> = Vec::new();
    file.read_to_end(&mut to_decrypt)?;

    let (salt, nonce) = retreive_salt_and_nonce(&to_decrypt)?;
    let aes_key = build_aes_key(password, &salt)?;
    let decrypted_key = decrypt_file(&to_decrypt[28..], aes_key, nonce)?;

    // real HOTP process
    let instant = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let counter = instant / 30;

    htop(decrypted_key, counter.as_secs())?;
    Ok(())
}

fn retreive_salt_and_nonce(file: &Vec<u8>) -> Result<([u8; 16], Nonce<Aes256Gcm>), FtOtpError> {
    if file.len() < 16 + 12 + 16 {
        return Err(FtOtpError::CorruptedKeyFile);
    }
    let salt: [u8; 16] = file[0..16].try_into()?;
    let nonce: Nonce<Aes256Gcm> = file[16..28].try_into()?;
    Ok((salt, nonce))
}

fn hmac_sha1(mut key: Vec<u8>, counter: u64) -> Vec<u8> {
    let result = Sha1::digest(&key);
    let o_key_pad = result.iter().map(|x| *x ^ 0x5c).collect();
    let i_key_pad = result.iter().map(|x| *x ^ 0x36).collect().

    return 
}

fn htop(key: Vec<u8>, counter: u64) -> Result<(), FtOtpError> {
    const LENGHT = 6;
    let mac = hmac_sha1(key, counter);
    Ok(())
}
