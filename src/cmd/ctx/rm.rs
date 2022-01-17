use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxRmCmd {
    name: String,
}

impl CmdHandling for CtxRmCmd {
    fn handle(&self) -> Result<String, String> {
        Ok(format!("CtxRm ran to completion with name: {}", self.name))
    }
}
