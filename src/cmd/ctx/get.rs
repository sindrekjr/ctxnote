use crate::cmd::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxGetCmd {
    pattern: Option<String>,
}

impl CmdHandling for CtxGetCmd {
    fn handle(&self) -> Result<&str, &str> {
        todo!()
    }
}
