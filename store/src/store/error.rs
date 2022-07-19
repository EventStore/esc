use hyper::Uri;
use jsonwebtoken::errors::Error as JsonWebTokenError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenStoreError {
    #[error("Invalid audience - missing url scheme or host: {0}")]
    BadAudience(String),
    #[error("Invalid email address")]
    InvalidEmail(),
    #[error("URI {0} has no host")]
    MissingHost(Uri),
    #[error("File {0} is malformed")]
    MalformedFile(PathBuf),
    #[error("No refresh token found")]
    MissingRefreshToken,
    #[error("The RSA PEM key is invalid")]
    RsaPemError(#[from] JsonWebTokenError),
    #[error("Couldn't read file {0}")]
    ReadFileError(PathBuf),
}
