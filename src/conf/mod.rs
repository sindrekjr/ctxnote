mod data;
mod user;

use crate::ctx::Context;
use data::DataConfig;
use std::path::PathBuf;
use user::UserConfig;
use uuid::Uuid;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub email: String,
    pub context_id: Uuid,
    pub data_dir: PathBuf,
}

impl Config {
    pub fn get() -> Self {
        let defaults = Self::default();
        match UserConfig::get() {
            Ok(user_config) => defaults.merge(user_config),
            Err(_) => defaults,
        }
    }

    pub fn merge(mut self, user_config: UserConfig) -> Self {
        if let Some(user) = &user_config.user {
            if let Some(name) = &user.name {
                self.username = name.to_string();
            };
            if let Some(email) = &user.email {
                self.email = email.to_string();
            }
        };

        if let Some(ctx) = &user_config.context {
            self.context_id = ctx.id;
        }

        if let Some(data) = &user_config.data {
            self.data_dir = data.dir.to_path_buf();
        }

        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: String::from("user"),
            email: String::from(""),
            context_id: Context::default().id,
            data_dir: DataConfig::default().dir,
        }
    }
}
