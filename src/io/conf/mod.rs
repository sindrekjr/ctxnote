mod data;
mod usr;

pub use data::Storage;

use crate::io::ctx::Context;
use crate::util::is_default;
use data::DataConfig;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use usr::UserConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default, skip_serializing_if = "is_default")]
    pub user: UserConfig,
    #[serde(default, skip_serializing_if = "is_default")]
    pub context: Context,
    #[serde(default, skip_serializing_if = "is_default")]
    pub data: DataConfig,
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
            },
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

    pub fn write(&self) -> Result<(), String> {
        let conf_str = toml::to_string(&self).unwrap();

        let path = Self::path();
        match OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
        {
            Err(why_not_open) => Err(why_not_open.to_string()),
            Ok(mut file) => match file.write_all(conf_str.as_bytes()) {
                Err(why_not_write) => Err(why_not_write.to_string()),
                Ok(_) => Ok(()),
            },
        }
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
