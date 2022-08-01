#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JobId(pub String);

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for JobId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
