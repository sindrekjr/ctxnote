mod data;
mod usr;

pub use data::Storage;

use crate::ctx::Context;
use data::DataConfig;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use usr::UserConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub context: Context,
    #[serde(default)]
    pub data: DataConfig,
    #[serde(default)]
    pub user: UserConfig,
}

impl Config {
    pub fn get_usr_config() -> Result<Self, String> {
        match std::fs::read_to_string(Self::path()) {
            Err(why) => Err(why.to_string()),
            Ok(config_str) => match toml::from_str(&config_str) {
                Err(why) => Err(why.to_string()),
                Ok(config) => Ok(config),
            },
        }
    }

    fn path() -> PathBuf {
        let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
        path.push(".ctxnote");
        path.push("config");
        path
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            context: Context::default(),
            data: DataConfig::default(),
            user: UserConfig::default(),
        }
    }
}
