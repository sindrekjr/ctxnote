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
    pub fn get() -> Self {
        match std::fs::read_to_string(Self::path()) {
            Err(_) => Self::default(),
            Ok(conf_str) => match toml::from_str(&conf_str) {
                Err(why_not_parse) => {
                    println!("Failed to parse user config: {}", why_not_parse);
                    Self::default()
                }
                Ok(conf) => conf,
            }
        }
    }

    fn path() -> PathBuf {
        let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
        path.push(".ctxnote");
        path.push("config");
        path
    }

    pub fn with_context(mut self, ctx: Context) -> Self {
        self.context = ctx;
        self
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
