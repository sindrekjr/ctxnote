mod ctx;

use clap::Parser;

pub trait CmdHandling {
    fn handle(&self) -> Result<String, String>;
}

#[derive(Parser)]
enum Cmd {
    Ctx(ctx::CtxCmd),
    Init(ctx::CtxInitCmd),
}

#[derive(Parser)]
#[clap(name = "ctxnote", version)]
pub struct NoteCmd {
    entry: Option<String>,

    #[clap(subcommand)]
    cmd: Option<Cmd>,
}

impl CmdHandling for NoteCmd {
    fn handle(&self) -> Result<String, String> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                Cmd::Ctx(ctx) => ctx.handle(),
                Cmd::Init(init) => init.handle(),
            }
        } else if let Some(entry) = &self.entry {
            Ok(format!("Note ran to completion with entry: {}", entry))
        } else {
            Err("Note ran to completion without entry".to_owned())
        }
    }
}
