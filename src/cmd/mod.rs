mod ctx;

use clap::Parser;

pub trait CmdHandling {
    fn handle(&self) -> Result<&str, &str>;
}

#[derive(Parser)]
enum Cmd {
    Ctx(ctx::CtxCmd),
}

#[derive(Parser)]
#[clap(name = "ctxnote", version)]
pub struct NoteCmd {
    entry: Option<String>,

    #[clap(subcommand)]
    cmd: Option<Cmd>,
}

impl CmdHandling for NoteCmd {
    fn handle(&self) -> Result<&str, &str> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                Cmd::Ctx(ctx) => ctx.handle(),
            }
        } else if let Some(entry) = &self.entry {
            println!("Create entry: {}", entry);
            Ok("Ran to completion in NoteCmd")
        } else {
            todo!()
        }
    }
}
