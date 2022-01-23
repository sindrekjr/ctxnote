use crate::conf::{Config, Storage};
use crate::note::Note;
use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct GetCmd {
    pub pattern: Option<String>,
}

impl CmdHandling for GetCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        let mut path = match &config.data.storage {
            Storage::Fs { dir } => dir.to_path_buf(),
            _ => todo!(),
        };

        path.push(&config.context.id.to_string());
        path.push("note");
        if !path.exists() {
            return Err(String::from("context does not exist"));
        }

        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(why) => return Err(why.to_string()),
        };

        let entries: Vec<String> = content.lines().map(|s| Note::from_str(s).as_output()).collect();

        let mut output = entries.join("\n");
        output.push_str(&format!("\n\nfound {} entries", entries.len()));

        Ok(output)
    }
}
