use crate::ctx::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub usr: User,
    pub ctx: Context,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl Config {
    pub fn get() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            usr: User::default(),
            ctx: Context::default(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: None,
            email: None,
        }
    }
}
