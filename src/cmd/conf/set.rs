use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct ConfSetCmd {
    key: String,
    val: String,
}

impl CmdHandling for ConfSetCmd {
    fn handle(&self) -> Result<String, String> {
        Ok(format!(
            "ConfSet ran to completion with key {} and val {}",
            self.key, self.val
        ))
    }
}
