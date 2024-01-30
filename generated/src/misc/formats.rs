#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NoteId(pub String);

impl std::fmt::Display for NoteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for NoteId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
