use std::fmt::Display;
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
enum GenerateKeyError {
    MustHave64b,
}

impl Display for GenerateKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MustHave64b => write!(f, "Arg given must have at least 64 bytes"),
        }
    }
}

pub fn generate_key(file: File) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Generate a key:
    // File is open, it exists, it was checks for error
    // 1. Verify it contains at least 64 bytes
    // 2. Encrypt it
    // 3. Save it and return path to where it is stored
    check_possible_key(&file)?;
    Ok(PathBuf::new())
}

fn check_possible_key(file: &File) -> Result<(), Box<dyn std::error::Error>> {
    if file.metadata()?.len() < 64 {
        Err(Box::new(GenerateKeyError::MustHave64b))
    } else {
        Ok(())
    }
}
