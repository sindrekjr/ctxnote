use crate::CmdHandling;
use crate::note::Note;
use clap::Parser;

#[derive(Parser)]
pub struct AddCmd {
    entry: String,
}

impl CmdHandling for AddCmd {
    fn handle(&self) -> Result<String, String> {
        let note = Note::new(self.entry.to_owned());
        Ok(format!("Add ran to completion with new note: {:?}", note))
    }
}
