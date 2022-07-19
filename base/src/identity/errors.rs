use http::StatusCode;
use reqwest::Error as ReqwestError;

pub struct IdentityError {
    pub message: String,
    pub status_code: Option<StatusCode>,
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        match self.status_code {
            Some(code) => writeln!(f, "{}: {}", code, self.message),
            None => writeln!(f, "{}", self.message),
        }
    }
}

impl std::fmt::Debug for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl std::error::Error for IdentityError {}

impl From<ReqwestError> for IdentityError {
    fn from(re: ReqwestError) -> Self {
        Self {
            message: format!("reqwest error: {}", re),
            status_code: None,
        }
    }
}
