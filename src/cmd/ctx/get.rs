use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxGetCmd {
    pattern: Option<String>,
}

impl CmdHandling for CtxGetCmd {
    fn handle(&self) -> Result<String, String> {
        Ok("CtxGet ran to completion".to_owned())
    }
}
