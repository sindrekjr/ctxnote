use crate::cmd::CmdHandling;
use crate::io::conf::{Config, Storage};
use crate::note::Note;
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
        if !path.exists() {
            match std::fs::create_dir_all(&path) {
                Ok(_) => println!("Initialized context: {}", config.context.name),
                Err(why) => return Err(why.to_string()),
            };
        }

        let pattern = match &self.pattern {
            Some(pattern) => pattern,
            None => "",
        };

        path.push("note");
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => String::new(),
        };

        let entries: Vec<String> = content
            .lines()
            .filter_map(|s| match s.contains(pattern) {
                true => Some(Note::from_str(s).as_output()),
                false => None,
            })
            .collect();

        let mut output = entries.join("\n");
        output.push_str(&format!("\n\nFound {} entries", entries.len()));

        Ok(output)
    }
}
