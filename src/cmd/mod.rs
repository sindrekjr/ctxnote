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
    #[clap(global = true, short, long)]
    context: Option<String>,

    #[clap(subcommand)]
    cmd: Cmd,
}

impl CmdHandling for NoteCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        if let Some(_) = &self.context {
            todo!()
        }

        match &self.cmd {
            Cmd::Add(add) => add.handle(config),
            Cmd::Get(get) => get.handle(config),
            Cmd::Conf(conf) => conf.handle(config),
            Cmd::Ctx(ctx) => ctx.handle(config),
            Cmd::Init(init) => init.handle(config),
        }
    }
}
