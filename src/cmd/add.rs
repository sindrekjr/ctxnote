use crate::cmd::CmdHandling;
use crate::conf::{Config, Storage};
use crate::note::Note;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
pub struct AddCmd {
    entry: String,
}

impl CmdHandling for AddCmd {
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
        };

        path.push("note");
        let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => file,
            Err(why) => return Err(why.to_string()),
        };

        let note = Note::new(
            config.user.name.to_owned(),
            config.user.email.to_owned(),
            self.entry.to_owned(),
        );

        match writeln!(file, "{}", note.to_str()) {
            Ok(_) => Ok(format!(
                "* [{}] {}",
                config.context.name, self.entry
            )),
            Err(why) => return Err(why.to_string()),
        }
    }
}
