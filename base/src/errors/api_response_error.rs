use super::problem_details::ProblemDetails;
use reqwest::StatusCode;

/// Represents an error reported by an API operation
#[derive(Clone, PartialEq)]
pub struct ApiResponseError {
    pub status_code: StatusCode,
    pub problem_details: ProblemDetails,
}

impl std::fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}: {}", self.status_code, self.problem_details)
    }
}

impl std::fmt::Debug for ApiResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl std::error::Error for ApiResponseError {}
