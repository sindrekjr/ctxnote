use crate::cmd::CmdHandling;
use crate::conf::Config;
use clap::Parser;

#[derive(Parser)]
pub struct ConfGetCmd {
    key: Option<String>,
}

impl CmdHandling for ConfGetCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        if let Some(key) = &self.key {
            Ok(format!("ConfGet ran to completion with key {}", key))
        } else {
            Ok(format!("ConfGet fetched config: {:?}", config))
        }
    }
}
