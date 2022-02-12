use crate::cmd::CmdHandling;
use crate::conf::Config;
use crate::ctx::ContextRegistry;
use clap::Parser;

#[derive(Parser)]
pub struct CtxSetCmd {
    #[clap(help = "Name of wanted context")]
    ctx_name: String,
}

impl CmdHandling for CtxSetCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        let ctx = match ContextRegistry::get().find(&self.ctx_name) {
            Ok(ctx) => ctx,
            Err(why) => return Err(why),
        };

        match Config::get().with_context(ctx).write() {
            Err(why) => Err(why),
            _ => Ok(format!("Switched to context '{}'", &self.ctx_name)),
        }
    }
}
