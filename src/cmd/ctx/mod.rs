mod init;
mod list;
mod mv;
mod rm;

pub use self::init::CtxInitCmd;

use crate::cmd::CmdHandling;
use crate::conf::Config;
use crate::ctx::ContextRegistry;
use clap::Parser;

#[derive(Parser)]
enum Cmd {
    Init(init::CtxInitCmd),
    List(list::CtxListCmd),
    Mv(mv::CtxMvCmd),
    Rm(rm::CtxRmCmd),
}

#[derive(Parser)]
pub struct CtxCmd {
    #[clap(help = "Name of context to switch to")]
    ctx_name: String,

    #[clap(subcommand)]
    cmd: Option<Cmd>,
}

impl CmdHandling for CtxCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                Cmd::Init(init) => init.handle(config),
                Cmd::List(list) => list.handle(config),
                Cmd::Mv(mv) => mv.handle(config),
                Cmd::Rm(rm) => rm.handle(config),
            }
        } else {
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
}
