use crate::note::Note;
use crate::CmdHandling;
use crate::Config;
use clap::Parser;

#[derive(Parser)]
pub struct AddCmd {
    entry: String,
}

impl CmdHandling for AddCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        let note = Note::new(config.user.name.to_owned(), config.user.email.to_owned(), self.entry.to_owned());
        Ok(format!("Add ran to completion with new note: {:?}", note))
    }
}
