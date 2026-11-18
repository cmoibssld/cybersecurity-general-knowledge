use crate::password;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
enum GenerateKeyError {
    MustHave64b,
    MustBeHexDigit,
    CannotDeriveToAES,
}

impl Display for GenerateKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MustHave64b => write!(f, "Key must have at least 64 characters"),
            Self::MustBeHexDigit => write!(f, "Key must contain only Hexadecimal digits"),
            Self::CannotDeriveToAES => write!(f, "Cannot derive password to AES key"),
        }
    }
}

pub fn generate_key(mut key: File) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Generate a key:
    // File is open, it exists, it was checks for error
    // 1. Verify it contains at least 64 bytes
    // 2. Encrypt it
    // 3. Save it and return path to where it is stored
    check_key_size(&key)?;
    let key = check_key_chars(&mut key)?;
    // 2.a Ask for a password (how much digits?)
    // 2.b encrypt the key with the password (6 digits)
    // 2.c save the encrypted key. Password-key will be asked to generate a 6 digit OTP.
    let password = password::retreive_password();
    let salt = password::passe_moi_le_sel(); // random salt gen -> first 16 bytes of the new ft_otp.key
    let aes_key = password::built_aes_key(password, &salt)
        .ok_or(Box::new(GenerateKeyError::CannotDeriveToAES))?;
    let nonce = password::get_nonce();
    let encrypted_key = password::encrypt_file(key, aes_key, nonce)?;
    let mut output_path = PathBuf::new();
    output_path.push("ft_otp");
    output_path.set_extension("key");
    let mut encrypted_output: File = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true) // if already exist, produce Err
        .open(&output_path)?;
    fs::set_permissions(&output_path, fs::Permissions::from_mode(0o600))?;
    encrypted_output.write_all(&salt)?;
    encrypted_output.write_all(&nonce)?;
    encrypted_output.write_all(&encrypted_key)?;
    // println!("salt: {:?}, nonce: {:?}", salt, nonce);
    Ok(output_path)
}

fn check_key_size(file: &File) -> Result<(), Box<dyn std::error::Error>> {
    if file.metadata()?.len() < 64 {
        Err(Box::new(GenerateKeyError::MustHave64b))
    } else {
        Ok(())
    }
}

fn check_key_chars(file: &mut File) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    if buf.iter().find(|x| !x.is_ascii_hexdigit()).is_some() {
        Err(Box::new(GenerateKeyError::MustBeHexDigit))
    } else {
        Ok(buf)
    }
}
