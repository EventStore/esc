use super::error::TokenStoreError;
use super::standard_claims::StandardClaims;
use super::token_file::TokenFile;
use super::token_validator::TokenValidator;
use crate::errors::{Result, StoreError};
use esc_client_base::identity::operations;
use esc_client_base::identity::TokenConfig;
use esc_client_base::Token;
use std::path::Path;

pub struct TokenStore {
    token_config: TokenConfig,
    token_file: TokenFile,
    validator: TokenValidator,
}

impl TokenStore {
    pub fn new(
        directory: &Path,
        token_config: TokenConfig,
        validator: TokenValidator,
    ) -> Result<Self> {
        let host = match get_host(&token_config.audience) {
            Some(host) => host,
            None => return Err(StoreError::new("can't create token store- the given token config has an audience with no host (is the URL correct?)")),
        };

        let token_path = directory.join(host);
        let token_file = TokenFile::new(token_path);

        Ok(TokenStore {
            token_config,
            token_file,
            validator,
        })
    }

    // Grabs the active Token after refreshing it if it's expired
    pub async fn access(&mut self, client: &reqwest::Client) -> Result<Token> {
        let previous_token = self.token_file.load().await?;
        match previous_token {
            Some(previous_token) => match self.validator.parse_token_claims(&previous_token) {
                Ok(claims) => {
                    if validate_claims(&claims) {
                        Ok(previous_token)
                    } else {
                        self.refresh_active_token_provided_token(client, previous_token)
                            .await
                    }
                }
                Err(e) => match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        error!("Invalid token: {}", e);
                        info!("Refreshing token...");
                        self.refresh_active_token_provided_token(client, previous_token)
                            .await
                    }
                    _ => Err(StoreError::new(
                        "can't access token - error parsing current token's claims",
                    )
                    .source(Box::new(e))),
                },
            },
            None => self.create_token_from_prompt(client).await,
        }
    }

    pub async fn create_token_from_prompt(&mut self, client: &reqwest::Client) -> Result<Token> {
        println!(
            "You don't appear to have a token for accessing {}, let's create a new one:",
            &self.token_config.audience
        );
        let email = read_email_from_user().map_err(|_| {
            StoreError::new("can't create token - reading email from prompt failed")
        })?;
        self.create_token_from_prompt_password_only(client, email)
            .await
    }

    pub async fn create_token_from_prompt_password_only(
        &mut self,
        client: &reqwest::Client,
        email: String,
    ) -> Result<Token> {
        let password = rpassword::read_password_from_tty(Some("Password: ")).map_err(|err| {
            StoreError::new("can't create token - reading password from prompt failed")
                .source(Box::new(err))
        })?;
        self.create_token(client, email, password).await
    }

    pub async fn create_token(
        &mut self,
        client: &reqwest::Client,
        email: String,
        password: String,
    ) -> Result<Token> {
        let new_token = operations::create(client, &self.token_config, &email, &password)
            .await
            .map_err(|err| {
                StoreError::new("can't create token - the call to the identity API failed")
                    .source(Box::new(err))
            })?;

        let new_token = self.token_file.save(new_token).await.map_err(|err| {
            StoreError::new("can't create token - saving the token failed").source(Box::new(err))
        })?;

        info!("Created initial token");

        Ok(new_token)
    }

    async fn refresh_active_token_provided_token(
        &mut self,
        client: &reqwest::Client,
        token: Token,
    ) -> Result<Token> {
        let refresh_token = match token.refresh_token() {
            Some(s) => s,
            None => {
                return Err(StoreError::new(
                    "can't refresh the token: the optional refresh token field wasn't set",
                ))
            }
        };
        let refreshed_token = operations::refresh(client, &self.token_config, refresh_token)
            .await
            .map_err(|err| {
                StoreError::new("can't refresh the token: the call to the identity API failed")
                    .source(Box::new(err))
            })?;
        let token = token.update_access_token(refreshed_token.access_token());
        self.token_file.save(token).await
    }

    // loads the active token, then calls the refresh API to update it, then
    // writes it back to the file
    pub async fn refresh_active_token(&mut self, client: &reqwest::Client) -> Result<Token> {
        let previous_token = self.token_file.load().await.map_err(|err| {
            StoreError::new("can't refresh the token: the token file could not be loaded")
                .details(format!("token file = {:?}", self.token_file))
                .source(Box::new(err))
        })?;
        match previous_token {
            Some(previous_token) => {
                self.refresh_active_token_provided_token(client, previous_token)
                    .await
            }
            None => self.create_token_from_prompt(client, None).await,
        }
    }

    pub async fn show(&self) -> Result<Option<Token>> {
        self.token_file.load().await
    }
}

fn read_email_from_user() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut editor = rustyline::Editor::<()>::new();
    let line = editor.readline("Email: ")?;
    if validator::validate_email(line.as_str()) {
        Ok(line)
    } else {
        Err(Box::new(TokenStoreError::InvalidEmail()))
    }
}

fn validate_claims(claims: &StandardClaims) -> bool {
    let exp = claims.expires_at.unwrap_or(0);
    let now = chrono::Utc::now().timestamp();
    exp > now
}

fn get_host(some_url: &str) -> Option<String> {
    match url::Url::parse(some_url) {
        Ok(url) => url.host().map(|host| host.to_string()),
        Err(_) => None,
    }
}
