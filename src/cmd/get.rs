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

        let entries: Vec<Note> = content.lines().map(Note::from_str).collect();
        let mut output = String::new();

        let mut count = 0;
        for entry in entries {
            output.push_str("Author: ");
            output.push_str(&entry.author);
            output.push_str("\n");

            output.push_str("Date: ");
            output.push_str(&entry.time.to_rfc2822());
            output.push_str("\n\n");

            output.push_str(" - ");
            output.push_str(&entry.text);
            output.push_str("\n\n");

            count += 1;
        }

        output.push_str(&format!("found {} entries", count));

        Ok(output)
    }
}
