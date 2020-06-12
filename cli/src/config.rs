use std::path::{Path, PathBuf};

lazy_static! {
    pub static ref ESC_DIR: PathBuf = {
        let home_dir = dirs::home_dir().expect("Not supported platform: can't find home directory");
        Path::new(&home_dir).join(".esc")
    };
}

lazy_static! {
    pub static ref SETTINGS_FILE: PathBuf = Path::new(ESC_DIR.as_path()).join("settings.toml");
}

lazy_static! {
    pub static ref SETTINGS: Settings = {
        match load_settings() {
            Ok(settings) => settings,
            _ => Default::default(),
        }
    };
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Settings {
    pub context: Option<Context>,
}

impl Settings {
    pub async fn persist(self) -> Result<(), Box<dyn std::error::Error>> {
        if tokio::fs::metadata(ESC_DIR.as_path()).await.is_err() {
            tokio::fs::create_dir_all(ESC_DIR.as_path()).await?;
        }

        let bytes = toml::to_vec(&self)?;

        tokio::fs::write(SETTINGS_FILE.as_path(), &bytes).await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Context {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<esc_api::OrgId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<esc_api::ProjectId>,
}

fn load_settings() -> Result<Settings, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(SETTINGS_FILE.as_path())?;
    let settings: Settings = toml::from_slice(&bytes)?;

    Ok(settings)
}
