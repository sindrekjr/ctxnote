use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxMvCmd {
    name: String,
    newname: String,
}

impl CmdHandling for CtxMvCmd {
    fn handle(&self) -> Result<String, String> {
        Ok(format!(
            "CtxMv ran to completion with: {} -> {}",
            self.name, self.newname
        ))
    }
}
