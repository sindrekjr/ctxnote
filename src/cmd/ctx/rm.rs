use crate::cmd::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxRmCmd {
    name: String,
}

impl CmdHandling for CtxRmCmd {
    fn handle(&self) -> Result<&str, &str> {
        todo!()
    }
}
