mod add;
mod conf;
mod ctx;
mod get;

use crate::ctx::ContextRegistry;
use crate::conf::Config;
use clap::Parser;

pub trait CmdHandling {
    fn handle(&self, config: &Config) -> Result<String, String>;
}

#[derive(Parser)]
enum Cmd {
    Add(add::AddCmd),
    Get(get::GetCmd),
    Conf(conf::ConfCmd),
    Ctx(ctx::CtxCmd),
    Init(ctx::CtxInitCmd),
}

#[derive(Parser)]
#[clap(name = "ctxnote", version)]
pub struct NoteCmd {
    #[clap(global = true, short, long, help = "Set inline context by name")]
    context: Option<String>,

    #[clap(subcommand)]
    cmd: Cmd,
}

impl NoteCmd {
    pub fn handle(&self) -> Result<String, String> {
        let config = match self.parse_config() {
            Ok(conf) => conf,
            Err(why) => return Err(why),
        };

        match &self.cmd {
            Cmd::Add(add) => add.handle(&config),
            Cmd::Get(get) => get.handle(&config),
            Cmd::Conf(conf) => conf.handle(&config),
            Cmd::Ctx(ctx) => ctx.handle(&config),
            Cmd::Init(init) => init.handle(&config),
        }
    }

    fn parse_config(&self) -> Result<Config, String> {
        let config = Config::get();

        if let Some(ctx_name) = &self.context {
            let ctx = match ContextRegistry::get().find(&ctx_name) {
                Ok(ctx) => ctx,
                Err(why) => return Err(why),
            };

            Ok(config.with_context(ctx))
        } else {
            Ok(config)
        }
    }
}
