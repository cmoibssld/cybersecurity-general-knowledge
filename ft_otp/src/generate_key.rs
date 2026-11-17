use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
enum GenerateKeyError {
    MustHave64b,
    MustBeHexDigit,
}

mod password;

impl Display for GenerateKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MustHave64b => write!(f, "Key must have at least 64 bytes"),
            Self::MustBeHexDigit => write!(f, "Key must contain only Hexadecimal digits"),
        }
    }
}

pub fn generate_key(mut file: File) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Generate a key:
    // File is open, it exists, it was checks for error
    // 1. Verify it contains at least 64 bytes
    // 2. Encrypt it
    // 3. Save it and return path to where it is stored
    check_key_size(&file)?;
    check_key_chars(&mut file)?;
    // 2.a Ask for a password (how much digits?)
    // 2.b encrypt the key with the password (6 digits)
    // 2.c save the encrypted key. Password-key will be asked to generate a 6 digit OTP.
    let password = password::retreive_password();
    println!("This is the password (never do that lol): {password}");
    Ok(PathBuf::new())
}

fn check_key_size(file: &File) -> Result<(), Box<dyn std::error::Error>> {
    if file.metadata()?.len() < 64 {
        Err(Box::new(GenerateKeyError::MustHave64b))
    } else {
        Ok(())
    }
}

fn check_key_chars(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    if buf.iter().find(|x| !x.is_ascii_hexdigit()).is_some() {
        Err(Box::new(GenerateKeyError::MustBeHexDigit))
    } else {
        Ok(())
    }
}
