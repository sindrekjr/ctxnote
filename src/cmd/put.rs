use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct PutCmd {
    entry: String,
}

impl CmdHandling for PutCmd {
    fn handle(&self) -> Result<String, String> {
        Ok(format!("Put ran to completion with entry: {}", self.entry))
    }
}
