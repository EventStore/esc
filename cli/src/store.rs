use esc_api::{command::tokens::Tokens, ClientId, Email, StandardClaims, Token};
use hyper::Uri;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::error::Error;
#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct Auth {
    pub id: ClientId,
    pub audience: Uri,
}

pub struct TokenStore<'a> {
    auth: &'a Auth,
    tokens: Tokens<'a>,
    validation: jsonwebtoken::Validation,
    key: DecodingKey<'a>,
    path: PathBuf,
}

impl<'a> TokenStore<'a> {
    pub fn new(auth: &'a Auth, tokens: Tokens<'a>) -> Self {
        let path = TokenStore::token_dirs();
        let key = DecodingKey::from_rsa_pem(JWT_PUBLIC_KEY)
            .expect("Impossible, it's a valid RSA PEM key");
        let validation = Validation {
            algorithms: vec![Algorithm::RS256],
            ..Validation::default()
        };

        TokenStore {
            tokens,
            auth,
            path,
            key,
            validation,
        }
    }

    pub fn token_dirs() -> PathBuf {
        Path::new(crate::config::ESC_DIR.as_path()).join("tokens")
    }

    pub async fn access(&mut self) -> Result<Token, Box<dyn Error>> {
        let token_path = self.path.as_path().join(
            self.auth
                .audience
                .host()
                .expect("We have a host in this URI"),
        );

        let mut token = None;

        if fs::metadata(&token_path).await.is_ok() {
            let token_bytes = fs::read(&token_path).await?;
            let previous_token: Token = serde_json::from_slice(&token_bytes)?;

            match self.parse_token_claims(&previous_token) {
                Ok(claims) => {
                    if validate_claims(&claims) {
                        return Ok(previous_token);
                    }

                    token = Some(previous_token);
                }

                Err(e) => match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        error!("Invalid token: {}", e);
                        info!("Refreshing token...");

                        token = Some(previous_token);
                    }

                    _ => return Err(e.into()),
                },
            }
        }

        if token.is_none() {
            let audience = TokenStore::build_audience_str(&self.auth.audience);

            println!(
                "You don't appear to have a token for accessing {}, let's create a new one:",
                &self.auth.audience
            );
            let email = read_email_from_user()?;
            let password = rpassword::read_password_from_tty(Some("Password: "))?;
            let new_token = self
                .tokens
                .create(
                    &self.auth.id,
                    email.as_str(),
                    password.as_str(),
                    audience.as_str(),
                )
                .await?;

            let token_bytes = serde_json::to_vec(&new_token)?;

            // We persist that token to disk considering it contains the immutable
            // refresh_token.
            fs::write(&token_path, &token_bytes).await?;
            info!("Created initial token");

            token = Some(new_token);
        }

        let token = token.expect("Impossible refresh_token is undefined at this point");
        let refresh = self
            .tokens
            .refresh(&self.auth.id, token.refresh_token().unwrap().as_str())
            .await?;

        let token = token.update_access_token(refresh.access_token());
        let new_token_bytes = serde_json::to_vec(&token)?;

        fs::write(&token_path, &new_token_bytes).await?;

        Ok(token)
    }

    pub async fn configure(&self) -> std::io::Result<()> {
        fs::create_dir_all(self.path.as_path()).await?;

        #[cfg(not(target_os = "windows"))]
        {
            let mut tokens_file_permissions = tokio::fs::metadata(self.path.as_path())
                .await?
                .permissions();

            tokens_file_permissions.set_mode(0o750);
        }

        Ok(())
    }

    fn parse_token_claims(&self, token: &Token) -> jsonwebtoken::errors::Result<StandardClaims> {
        let token = jsonwebtoken::decode::<StandardClaims>(
            token.access_token(),
            &self.key,
            &self.validation,
        )?;
        Ok(token.claims)
    }

    pub fn build_audience_str(uri: &Uri) -> String {
        format!("{}://{}", uri.scheme_str().unwrap(), uri.host().unwrap())
    }
}

fn validate_claims(claims: &StandardClaims) -> bool {
    let exp = claims.expires_at.unwrap_or(0);
    let now = chrono::Utc::now().timestamp();
    let result = exp > now;

    if !result {
        warn!("Token has expired");
    }

    result
}

fn read_email_from_user() -> Result<Email, Box<dyn std::error::Error>> {
    let mut editor = rustyline::Editor::<()>::new();
    let line = editor.readline("Email: ")?;

    if let Some(email) = esc_api::Email::parse(line.as_str()) {
        return Ok(email);
    }

    Err(crate::StringError("Invalid email".to_string()).into())
}

static JWT_PUBLIC_KEY: &[u8] = include_bytes!("../key.pem");
