use crate::conf::data::DataConfig;
use crate::ctx::Context;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub user: Option<User>,
    pub data: Option<DataConfig>,
    pub context: Option<Context>,
}

impl UserConfig {
    pub fn get() -> Result<Self, String> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        };

        let user_conf_str = match std::fs::read_to_string(path) {
            Ok(string) => string,
            Err(why) => return Err(why.to_string()),
        };

        match toml::from_str(&user_conf_str) {
            Ok(config) => config,
            Err(why) => Err(why.to_string()),
        }
    }

    pub fn write(&self) -> Result<(), Error> {
        let path = Self::path();
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        }

        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .open(Self::path())
        {
            Ok(f) => f,
            Err(why) => return Err(why),
        };
        let serialized = toml::to_string(&self).unwrap();
        file.write_all(serialized.as_bytes())
    }

    fn path() -> PathBuf {
        let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
        path.push(".ctxnote");
        path.push("config");
        path
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            user: Some(User::default()),
            data: Some(DataConfig::default()),
            context: Some(Context::default()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: Some(String::from("user")),
            email: None,
        }
    }
}
