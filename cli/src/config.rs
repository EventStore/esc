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
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub default_profile: Option<String>,

    #[serde(rename = "profile")]
    pub profiles: Vec<Profile>,
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

    pub fn get_current_profile(&self) -> Option<&Profile> {
        let default_profile_name = self.default_profile.as_ref()?.as_str();

        self.get_profile(default_profile_name)
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == name)
    }

    pub fn get_profile_mut(&mut self, name: &str) -> &mut Profile {
        let mut idx = 0;
        let mut found = false;

        for profile in self.profiles.iter() {
            if profile.name == name {
                found = true;
                break;
            }

            idx += 1;
        }

        if found {
            return self
                .profiles
                .get_mut(idx)
                .expect("Impossible situation: we know idx is valid!");
        }

        self.profiles.push(Profile {
            name: name.to_string(),
            ..Default::default()
        });

        self.profiles
            .last_mut()
            .expect("Impossible situation: we just added a new profile!")
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub name: String,

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
