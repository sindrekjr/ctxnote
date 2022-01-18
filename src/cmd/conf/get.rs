use crate::conf::Config;
use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct ConfGetCmd {
    key: Option<String>,
}

impl CmdHandling for ConfGetCmd {
    fn handle(&self) -> Result<String, String> {
        let config = Config::get();

        if let Some(key) = &self.key {
            Ok(format!("ConfGet ran to completion with key {}", key))
        } else {
            Ok(format!("ConfGet fetched config: {:?}", config))
        }
    }
}