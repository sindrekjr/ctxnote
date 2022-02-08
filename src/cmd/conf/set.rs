use crate::cmd::CmdHandling;
use crate::conf::Config;
use clap::Parser;

#[derive(Parser)]
pub struct ConfSetCmd {
    key: String,
    val: String,
}

impl CmdHandling for ConfSetCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        Ok(format!(
            "ConfSet ran to completion with key {} and val {}",
            self.key, self.val
        ))
    }
}
