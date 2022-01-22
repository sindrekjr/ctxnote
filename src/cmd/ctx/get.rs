use crate::CmdHandling;
use crate::Config;
use clap::Parser;

#[derive(Parser)]
pub struct CtxGetCmd {
    pattern: Option<String>,
}

impl CmdHandling for CtxGetCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        Ok("CtxGet ran to completion".to_owned())
    }
}
