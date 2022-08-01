/// Represents a problem making a request which indicates the request itself
/// was not made to the API or could not be read correctly. This includes
/// errors from the API not in the spec or errors that can happen when
/// serializing data to send to or read from the API.
pub struct CommunicationError {
    pub debug: String,
    pub message: String,
    pub source: Box<dyn std::error::Error + Send + Sync>,
}

impl std::fmt::Display for CommunicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for CommunicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(
            f,
            "{}\nDetails: {}\nCaused by:\n\t{:?}",
            self.message, self.debug, self.source
        )
    }
}

impl std::error::Error for CommunicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}
