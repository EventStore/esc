pub struct StoreError {
    pub debug: Option<String>,
    pub message: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl StoreError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            debug: None,
            source: None,
        }
    }

    pub fn debug(mut self, debug: String) -> Self {
        self.debug = Some(debug);
        self
    }

    pub fn details(mut self, details: String) -> Self {
        self.debug = Some(format!("{}\nDetails: {}", self.message, details));
        self
    }

    pub fn source(mut self, source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self.source = Some(source);
        self
    }

    pub fn from_message(message: String) -> Self {
        Self {
            message,
            debug: None,
            source: None,
        }
    }

    pub fn new_detailed(
        message: &str,
        debug: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self {
            message: message.to_string(),
            debug: Some(format!("{}\nDetails: {}", message, debug)),
            source: Some(source),
        }
    }
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        match &self.debug {
            Some(d) => write!(f, "{}", d),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            None => None,
        }
    }
}
