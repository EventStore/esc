#[derive(Clone)]
pub struct TokenConfig {
    // Identifies the recipients the JWT is intended for.
    // See https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.3
    pub audience: String,
    // Identifies the client, but not the user
    pub client_id: String,
    // Base URL of identity API
    pub identity_url: String,
    // public key the token should be signed with
    pub public_key: String,
}

// The public key of the signing certificate. Find it with:
//  openssl x509 -noout -pubkey -in signing.crt -out key.pem
static JWT_PUBLIC_KEY: &str = include_str!("key.pem");

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            audience: "https://api.eventstore.cloud".to_owned(),
            client_id: "OraYp3cFES9O8aWuQtnqi1A7m534iTwt".to_owned(),
            identity_url: "https://identity.eventstore.com".to_owned(),
            public_key: JWT_PUBLIC_KEY.to_owned(),
        }
    }
}
