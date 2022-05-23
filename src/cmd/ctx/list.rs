use crate::cmd::CmdHandling;
use crate::io::conf::Config;
use crate::io::ctx::ContextRegistry;
use clap::Parser;

#[derive(Parser)]
pub struct CtxListCmd {
    pattern: Option<String>,
}

impl CmdHandling for CtxListCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        let reg = ContextRegistry::get();
        Ok(reg.list(&self.pattern))
    }
}
