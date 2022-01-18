mod conf;
mod ctx;
mod put;

use clap::Parser;

pub trait CmdHandling {
    fn handle(&self) -> Result<String, String>;
}

#[derive(Parser)]
enum Cmd {
    Put(put::PutCmd),
    Conf(conf::ConfCmd),
    Ctx(ctx::CtxCmd),
    Init(ctx::CtxInitCmd),
}

#[derive(Parser)]
#[clap(name = "ctxnote", version)]
pub struct NoteCmd {
    #[clap(subcommand)]
    cmd: Cmd,
}

impl CmdHandling for NoteCmd {
    fn handle(&self) -> Result<String, String> {
        match &self.cmd {
            Cmd::Put(put) => put.handle(),
            Cmd::Conf(conf) => conf.handle(),
            Cmd::Ctx(ctx) => ctx.handle(),
            Cmd::Init(init) => init.handle(),
        }
    }
}
