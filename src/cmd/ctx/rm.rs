use crate::cmd::CmdHandling;
use crate::io::conf::Config;
use clap::Parser;

#[derive(Parser)]
pub struct CtxRmCmd {
    name: String,
}

impl CmdHandling for CtxRmCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        Ok(format!("CtxRm ran to completion with name: {}", self.name))
    }
}
