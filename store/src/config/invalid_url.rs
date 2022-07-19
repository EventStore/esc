pub struct InvalidUrl {}

impl std::fmt::Display for InvalidUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Malformed URL. Expecting HTTP/HTTPS scheme, a valid host or IP"
        )
    }
}

impl std::fmt::Debug for InvalidUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Malformed URL. Expecting HTTP/HTTPS scheme, a valid host or IP"
        )
    }
}

impl std::error::Error for InvalidUrl {}
