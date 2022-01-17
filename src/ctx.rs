use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

///
/// Context
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub id: Uuid,
    pub name: String,
    pub path: Option<PathBuf>,
}

impl Context {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            path: None,
        }
    }

    pub fn bind(&mut self, path: PathBuf) -> &mut Self {
        self.path = Some(path);
        self
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            name: "default".to_string(),
            path: None,
        }
    }
}

///
/// ContextRegistry
///
#[derive(Deserialize, Serialize)]
pub struct ContextRegistry {
    pub contexts: Vec<Context>,
}
