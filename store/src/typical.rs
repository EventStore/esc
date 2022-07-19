use crate::config::Settings;
use crate::errors::{Result, StoreError};
use crate::store::TokenStore;
use crate::store::TokenValidator;
use esc_client_base::identity::TokenConfig;
use std::path::PathBuf;

fn get_esc_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        StoreError::from_message("unsupported platform - no home directory".to_string())
    })?;
    Ok(home_dir.join(".esc"))
}

pub async fn load_settings() -> Result<Settings> {
    let esc_dir = get_esc_dir()?;
    let settings_file = esc_dir.join("settings.toml");
    if !settings_file.exists() {
        info!("Creating initial ESC settings file...");
        let settings = Settings::default();
        settings.persist(&settings_file).await?;
        Ok(settings)
    } else {
        Settings::load_settings(settings_file).await
    }
}

pub async fn token_store(token_config: TokenConfig) -> Result<TokenStore> {
    let esc_dir = get_esc_dir()?;
    let token_dir = esc_dir.join("tokens");
    let validator = TokenValidator::new_from_rsa_pem(&token_config.public_key)?;
    let ts = TokenStore::new(&token_dir, token_config, validator).map_err(|err| {
        StoreError::new("error creating default token store").source(Box::new(err))
    })?;
    Ok(ts)
}
