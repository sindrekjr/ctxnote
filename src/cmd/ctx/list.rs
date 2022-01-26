use crate::ctx::ContextRegistry;
use crate::CmdHandling;
use crate::Config;
use clap::Parser;

#[derive(Parser)]
pub struct CtxListCmd {
    pattern: Option<String>,
}

impl CmdHandling for CtxListCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        let reg = match ContextRegistry::get() {
            Ok(reg) => reg,
            Err(_) => ContextRegistry::default(),
        };

        Ok(reg.list(&self.pattern))
    }
}
