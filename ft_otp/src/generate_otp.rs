use aes_gcm::Aes256Gcm;
use aes_gcm::aead::Nonce;
use hex::decode;
use sha1::{Digest, Sha1};

use crate::errors::FtOtpError;
use crate::password::{self, build_aes_key, decrypt_file};
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

pub fn generate_otp(mut file: File) -> Result<String, FtOtpError> {
    let password = password::retreive_password(false)?;
    let mut to_decrypt: Vec<u8> = Vec::new();
    file.read_to_end(&mut to_decrypt)?;

    let (salt, nonce) = retreive_salt_and_nonce(&to_decrypt)?;
    let aes_key = build_aes_key(password, &salt)?;
    let decrypted_key = decrypt_file(&to_decrypt[28..], aes_key, nonce)?;
    let decrypted_key = decode(decrypted_key)?;

    // real HOTP process
    let instant = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let counter = instant / 30;

    htop(decrypted_key, counter.as_secs())
}

fn retreive_salt_and_nonce(file: &Vec<u8>) -> Result<([u8; 16], Nonce<Aes256Gcm>), FtOtpError> {
    if file.len() < 16 + 12 + 16 {
        return Err(FtOtpError::CorruptedKeyFile);
    }
    let salt: [u8; 16] = file[0..16].try_into()?;
    let nonce: Nonce<Aes256Gcm> = file[16..28].try_into()?;
    Ok((salt, nonce))
}

fn hmac_sha1(key: Vec<u8>, counter: u64) -> Vec<u8> {
    let mut result = key;
    if result.len() > 64 {
        result = Sha1::digest(result).to_vec(); // if the key is >= 64 (which is always the case for use) -> bring it back to 20 bytes pads with 0 for reaching 64 and now it fits in one block. Returns the hash aka 64 bytes
    }
    result.resize(64, 0);
    let o_key_pad = result.iter().map(|x| *x ^ 0x5c).collect::<Vec<u8>>();
    let i_key_pad: Vec<u8> = result.iter().map(|x| *x ^ 0x36).collect::<Vec<u8>>();

    // I must do tho hash to avoid possible attack and lenght-extension attack especially
    // inner hash = SHA1(i_key_pad || counter)
    // MAC = SHA1(o_key_pad || innner_hash)

    let mut inner_hash = Sha1::new();
    inner_hash.update(&i_key_pad);
    inner_hash.update(counter.to_be_bytes());
    let inner = inner_hash.finalize();

    let mut outer_hash = Sha1::new();
    outer_hash.update(o_key_pad);
    outer_hash.update(inner);
    outer_hash.finalize().to_vec()
}

fn htop(key: Vec<u8>, counter: u64) -> Result<String, FtOtpError> {
    let mac = hmac_sha1(key, counter);
    Ok(truncate(&mac))
}

fn truncate(mac: &[u8]) -> String {
    let offset = (mac[19] & 0x0f) as usize;
    // u19 being the last bytes of the mac
    let value = u32::from_be_bytes([
        mac[offset] & 0x7f,
        mac[offset + 1],
        mac[offset + 2],
        mac[offset + 3],
    ]);
    format!("{:06}", value % 1000000)
}
