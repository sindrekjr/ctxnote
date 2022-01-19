use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct AddCmd {
    entry: String,
}

impl CmdHandling for AddCmd {
    fn handle(&self) -> Result<String, String> {
        Ok(format!("Put ran to completion with entry: {}", self.entry))
    }
}
