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

    pub fn bind(&mut self, path: &PathBuf) -> &mut Self {
        self.path = Some(path.to_path_buf());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_correctly_stores_name() {
        let name = String::from("Text Context");
        let ctx = Context::new(name.to_string());
        assert_eq!(ctx.name, name);
    }

    #[test]
    fn new_creates_unique_id() {
        let ctx1 = Context::new(String::from("Test Context"));
        let ctx2 = Context::new(String::from("Test Context"));
        assert_ne!(ctx1.id, ctx2.id);
    }

    #[test]
    fn bind_correctly_stores_path() {
        let path = PathBuf::default();
        let mut ctx = Context::new(String::from("Test Context"));
        ctx.bind(&path);
        assert_eq!(ctx.path.unwrap(), path);
    }
}
