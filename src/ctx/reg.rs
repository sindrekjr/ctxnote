use super::Context;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextRegistry {
    contexts: Vec<Context>,
}

impl ContextRegistry {
    pub fn get() -> Result<Self, String> {
        match std::fs::read_to_string(Self::path()) {
            Err(why_not_read) => Err(why_not_read.to_string()),
            Ok(string) => match toml::from_str(&string) {
                Err(why_not_parse) => Err(why_not_parse.to_string()),
                Ok(contexts) => Ok(contexts),
            },
        }
    }

    fn path() -> PathBuf {
        let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
        path.push(".ctxnote");
        path.push("contexts");
        path
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
        let mut dup_name = false;
        for ctx in self.contexts.iter() {
            if ctx == context {
                return Err(String::from("given context already exists"));
            }

            if ctx.name == context.name {
                dup_name = true;
            }
        }

        self.contexts.push(context.clone());
        let reg_str = toml::to_string(&self).unwrap();

        let path = Self::path();
        match OpenOptions::new().create(true).write(true).open(path) {
            Err(why_not_open) => Err(why_not_open.to_string()),
            Ok(mut file) => match file.write_all(reg_str.as_bytes()) {
                Err(why_not_write) => Err(why_not_write.to_string()),
                Ok(_) => {
                    if dup_name {
                        Ok(Some(String::from("NOTE! Contexts with same name already exist. This is inadvisable as it is likely to lead to confusion.")))
                    } else {
                        Ok(None)
                    }
                }
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
