use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserConfig {
    #[serde(default = "default_user")]
    pub name: String,
    #[serde(default)]
    pub email: String,
}

fn default_user() -> String {
    String::from("user")
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            name: default_user(),
            email: String::default(),
        }
    }
}
