use crate::cmd::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxMvCmd {
    name: String,
    newname: String,
}

impl CmdHandling for CtxMvCmd {
    fn handle(&self) -> Result<&str, &str> {
        todo!()
    }
}
