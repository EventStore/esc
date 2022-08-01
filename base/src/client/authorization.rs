use crate::identity::Token;

/// Creates value used for "Authorization" header, and refreshes it if needed
pub trait Authorization {
    /// Gets the authorization header
    fn authorization_header(&self) -> String;

    /// Refreshes the authorization. If successful, this returns true.
    fn refresh(&mut self) -> bool;
}

/// This authorizes uses a static token and doesn't do anything if the auth
/// is bad.
pub struct StaticTokenAuthorizer {
    pub token: Token,
}

impl Authorization for StaticTokenAuthorizer {
    fn authorization_header(&self) -> String {
        self.token.authorization_header()
    }

    fn refresh(&mut self) -> bool {
        false
    }
}
