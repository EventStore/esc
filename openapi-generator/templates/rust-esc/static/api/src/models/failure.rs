use super::problem_details::ProblemDetails;

pub trait Failure {
    fn status_code(&self) -> u16;
    fn content(&self) -> String;
    fn details(&self) -> Option<ProblemDetails>;
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct UnexpectedResponse {
    pub content: String,
    pub status_code: u16
}

impl std::fmt::Display for UnexpectedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Status {}: {}", self.status_code, &self.content)
    }
    
}

impl UnexpectedResponse {
    pub fn new(status_code: u16, content: String) -> Self {
        Self{status_code, content}
    }
}

impl Failure for UnexpectedResponse {
    fn details(&self) -> Option<ProblemDetails> {
        match serde_json::from_str(&self.content) {
            Ok(details) => Some(details),
            _ => None
        }
    }
    
    fn status_code(&self) -> u16 {
        return self.status_code.clone();
    }

    fn content(&self) -> String {
        return self.content.clone();
    }
}