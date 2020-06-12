use esc_api::{command::tokens::Tokens, ClientId, StandardClaims, Token};
use hyper::Uri;
use std::error::Error;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct Auth {
    pub id: ClientId,
    pub audience: Uri,
    pub username: String,
    pub password: String,
}

pub struct TokenStore<'a> {
    auth: Auth,
    tokens: Tokens<'a>,
    //validation: jsonwebtoken::Validation,
    //key: jsonwebtoken::DecodingKey<'a>,
    path: PathBuf,
}

impl<'a> TokenStore<'a> {
    pub fn new(auth: Auth, tokens: Tokens<'a>) -> Self {
        let path = Path::new(crate::config::ESC_DIR.as_path()).join("tokens");
        //let key = jsonwebtoken::DecodingKey::from_rsa_pem(JWT_PUBLIC_KEY).unwrap();//.expect("Impossible, it's a valid RSA PEM key");
        //let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

        TokenStore { tokens, auth, path } //, validation, key }
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
            let new_token = self
                .tokens
                .create(
                    &self.auth.id,
                    self.auth.username.as_ref(),
                    self.auth.password.as_ref(),
                    TokenStore::build_audience_str(&self.auth.audience).as_str(),
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
        fs::create_dir_all(&self.path).await
    }

    // FIXME - Use safe version where we verify the token signature.
    fn parse_token_claims(&self, token: &Token) -> jsonwebtoken::errors::Result<StandardClaims> {
        //frank_jwt::decode(token.access_token(), &String::from_utf8(JWT_PUBLIC_KEY.to_owned()).unwrap(), frank_jwt::Algorithm::RS256, &frank_jwt::ValidationOptions::new()).unwrap();
        //let header = jsonwebtoken::decode_header(token.access_token())?;
        // let key = jsonwebtoken::DecodingKey::from_rsa_components(header["n"], header["e"]);
        // let validaton = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        // let container = jsonwebtoken::decode(token.access_token(), &self.key, &self.validation)?;
        let container = jsonwebtoken::dangerous_unsafe_decode(token.access_token())?;

        Ok(container.claims)
        //panic!("Header: {:?}", header)
    }

    fn build_audience_str(uri: &Uri) -> String {
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

static _JWT_PUBLIC_KEY: &[u8] = include_bytes!("../key.pem");
static _JWT_PUBLIC_CERT: &[u8] = include_bytes!("../cert.pem");
