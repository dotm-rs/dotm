use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use xdg::BaseDirectoriesError;

use crate::constants::{CONFIG_FILE_NAME, XDG_PREFIX};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub profile: String,
    pub profiles: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("xdg error: {0}")]
    XdgError(#[from] BaseDirectoriesError),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("toml deserialization error: {0}")]
    TomlDeserializationError(#[from] toml::de::Error),

    #[error("toml serialization error: {0}")]
    TomlSerializationError(#[from] toml::ser::Error),
}

impl Config {
    /// Returns the default config file path (`$XDG_DATA_HOME/dotm/dotm.toml`).
    pub fn default_file_path() -> Result<PathBuf, BaseDirectoriesError> {
        let base_data_dir = xdg::BaseDirectories::with_prefix(XDG_PREFIX)?.get_data_home();
        Ok(base_data_dir.join(CONFIG_FILE_NAME))
    }

    /// Read the [`Config`] from the the default config path. See
    /// [`Config::default_file_path()`] for more information.
    pub fn read() -> Result<Self, ConfigError> {
        let config_file_path = Self::default_file_path()?;

        let content = fs::read_to_string(config_file_path)?;
        let config = toml::from_str(&content)?;

        Ok(config)
    }

    /// Writes config data to the default config file path encoded as TOML.
    pub fn write(&self) -> Result<(), ConfigError> {
        let config_file_path = Self::default_file_path()?;

        let content = toml::to_string(self)?;
        fs::write(config_file_path, content)?;

        Ok(())
    }

    /// Creates the config file if needed (not already present). It returns
    /// wether the file was newly created or not.
    pub fn create_if_needed(&self) -> Result<bool, ConfigError> {
        let config_file_path = Self::default_file_path()?;

        if config_file_path.exists() {
            return Ok(false);
        }

        self.write()?;
        Ok(true)
    }
}
