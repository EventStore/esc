use super::standard_claims::StandardClaims;
use crate::errors::{Result, StoreError};
use esc_client_base::Token;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

pub struct TokenValidator {
    public_key: DecodingKey,
    validation: Validation,
}

impl TokenValidator {
    pub fn new(public_key: DecodingKey) -> Self {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.required_spec_claims.clear();
        Self {
            public_key,
            validation,
        }
    }

    pub fn new_from_rsa_pem(rsa_pem: &String) -> Result<Self> {
        let data = rsa_pem.as_bytes();
        let public_key = DecodingKey::from_rsa_pem(data).map_err(|err| {
            StoreError::new("error creating token validator: could not decode given rsa_pem")
                .source(Box::new(err))
        })?;
        Ok(Self::new(public_key))
    }

    pub fn parse_token_claims(
        &self,
        token: &Token,
    ) -> jsonwebtoken::errors::Result<StandardClaims> {
        let token = jsonwebtoken::decode::<StandardClaims>(
            token.access_token(),
            &self.public_key,
            &self.validation,
        )?;
        Ok(token.claims)
    }
}
