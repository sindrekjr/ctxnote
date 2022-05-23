use super::Context;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextRegistry {
    contexts: Vec<Context>,
}

impl ContextRegistry {
    pub fn get() -> Self {
        match std::fs::read_to_string(Self::path()) {
            Err(_) => Self::default(),
            Ok(reg_str) => match toml::from_str(&reg_str) {
                Err(why_not_parse) => {
                    println!("Failed to parse context registry: {}", why_not_parse);
                    Self::default()
                }
                Ok(reg) => reg,
            },
        }
    }

    fn path() -> PathBuf {
        let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
        path.push(".ctxnote");
        path.push("contexts");
        path
    }

    pub fn find(&self, ctx_name: &str) -> Result<Context, String> {
        if let Ok(id) = Uuid::parse_str(ctx_name) {
            return self.find_by_id(id);
        }

        let found: Vec<&Context> = self
            .contexts
            .iter()
            .filter(|ctx| ctx.name == ctx_name)
            .collect();

        match found.len() {
            1 => Ok(found[0].clone()),
            0 => Err(format!("Found no context with name {}", ctx_name)),
            _ => {
                let mut output = format!(
                    "Found {} contexts with name \"{}\". Specify context by id instead.",
                    found.len(),
                    ctx_name
                );

                output.push('\n');
                output.push_str(
                    &found
                        .iter()
                        .map(|ctx| ctx.as_output())
                        .collect::<Vec<String>>()
                        .join("\n"),
                );

                Err(output)
            }
        }
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Context, String> {
        match self.contexts.iter().find(|ctx| ctx.id == id) {
            Some(ctx) => Ok(ctx.clone()),
            None => Err(format!("Found no context with id {}", id)),
        }
    }

    pub fn list(&self, pattern: &Option<String>) -> String {
        let pattern = match pattern {
            Some(pattern) => pattern,
            None => "",
        };

        let contexts: Vec<String> = self
            .contexts
            .iter()
            .filter_map(|ctx| match ctx.name.contains(pattern) {
                true => Some(ctx.as_output()),
                false => None,
            })
            .collect();

        contexts.join("\n")
    }

    pub fn push(&mut self, context: &Context) -> Result<Option<String>, String> {
        for ctx in self.contexts.iter() {
            if ctx == context {
                return Err(String::from("Context already exists"));
            }

            if ctx.name == context.name {
                return Err(format!("Context named '{}' already exists.", ctx.name));
            }
        }

        self.contexts.push(context.clone());
        let reg_str = toml::to_string(&self).unwrap();

        let path = Self::path();
        match OpenOptions::new().create(true).write(true).open(path) {
            Err(why_not_open) => Err(why_not_open.to_string()),
            Ok(mut file) => match file.write_all(reg_str.as_bytes()) {
                Err(why_not_write) => Err(why_not_write.to_string()),
                Ok(_) => Ok(None),
            },
        }
    }
}

impl Default for ContextRegistry {
    fn default() -> Self {
        Self {
            contexts: vec![Context::default()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_contains_only_default_context() {
        let reg = ContextRegistry::default();
        assert_eq!(reg.contexts.len(), 1);
        assert_eq!(reg.contexts[0], Context::default());
    }
}
