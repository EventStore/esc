// use serde::export::Formatter;
use super::profile::Profile;
use crate::errors::{Result, StoreError};
#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub default_profile: Option<String>,

    #[serde(rename = "profile")]
    pub profiles: Vec<Profile>,
}

fn main_settings_file(settings_dir: impl AsRef<Path>) -> PathBuf {
    settings_dir.as_ref().join("settings.toml")
}

impl Settings {
    pub async fn load_settings(settings_file: impl AsRef<Path>) -> Result<Settings> {
        let bytes = tokio::fs::read(&settings_file).await.map_err(|err| {
            StoreError::new("Could not read settings file")
                .details(format!("settings file = {:?}", settings_file.as_ref()))
                .source(Box::new(err))
        })?;
        let settings: Settings = toml::from_slice(&bytes).map_err(|err| {
            StoreError::new("The settings file is incorectly formatted and cannot be read")
                .details(format!("settings file = {:?}", settings_file.as_ref()))
                .source(Box::new(err))
        })?;

        Ok(settings)
    }

    pub async fn persist(&self, settings_file: impl AsRef<Path>) -> Result<()> {
        let settings_dir = match settings_file.as_ref().parent() {
            Some(dir) => dir,
            None => {
                return Err(StoreError::new(
                    "can't save settings file: the given path is not located in a directory",
                )
                .details(format!("{:?}", settings_file.as_ref())))
            }
        };

        if tokio::fs::metadata(&settings_dir).await.is_err() {
            tokio::fs::create_dir_all(&settings_dir)
                .await
                .map_err(|err| {
                    StoreError::new_detailed(
                        "could not save settings: error creating directory",
                        format!("path={:?}", settings_dir),
                        Box::new(err),
                    )
                })?
        }

        let bytes = toml::to_vec(&self).map_err(|err| {
            StoreError::new("could not save settings: error in serialization to TOML")
                .source(Box::new(err))
        })?;

        let settings_file = main_settings_file(settings_dir);
        tokio::fs::write(settings_file.as_path(), &bytes)
            .await
            .map_err(|err| {
                StoreError::new("could not save settings")
                    .details(format!("file = {:?}", settings_file.as_path()))
                    .source(Box::new(err))
            })?;

        // Lock down file
        #[cfg(not(target_os = "windows"))]
        {
            let mut settings_file_permissions = tokio::fs::metadata(&settings_file)
                .await
                .map_err(|err| {
                    StoreError::new("error saving settings file: unable to set permissions")
                        .details(format!("file = {:?}", settings_file))
                        .source(Box::new(err))
                })?
                .permissions();

            settings_file_permissions.set_mode(0o640);
        }

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
