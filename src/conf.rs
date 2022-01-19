use crate::ctx::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub user: User,
    pub context: Context,
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
            user: User::default(),
            context: Context::default(),
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
