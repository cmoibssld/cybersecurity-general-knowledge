use thiserror::Error;

#[derive(Debug, Error)]
pub enum FtOtpError {
    #[error("Key must have at least 64 characters")]
    MustHave64b,
    #[error("Key must contain only Hexadecimal digits")]
    MustBeHexDigit,
    #[error(transparent)] // transparent pass the underlying error message
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    Argon2Error(#[from] argon2::Error),
    #[error(transparent)]
    AESError(#[from] aes_gcm::Error),
    #[error(transparent)]
    InquireError(#[from] inquire::InquireError),
    #[error("Key file corrupted")]
    CorruptedKeyFile,
    #[error(transparent)]
    TryFailed(#[from] std::array::TryFromSliceError),
    #[error(transparent)]
    TimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
}
