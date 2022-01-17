mod get;
mod init;
mod mv;
mod rm;

pub use self::init::CtxInitCmd;

use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
enum Cmd {
    Get(get::CtxGetCmd),
    Init(init::CtxInitCmd),
    Mv(mv::CtxMvCmd),
    Rm(rm::CtxRmCmd),
}

#[derive(Parser)]
pub struct CtxCmd {
    name: Option<String>,

    #[clap(subcommand)]
    subcmd: Option<Cmd>,
}

impl CmdHandling for CtxCmd {
    fn handle(&self) -> Result<String, String> {
        if let Some(cmd) = &self.subcmd {
            match cmd {
                Cmd::Get(get) => get.handle(),
                Cmd::Init(init) => init.handle(),
                Cmd::Mv(mv) => mv.handle(),
                Cmd::Rm(rm) => rm.handle(),
            }
        } else if let Some(name) = &self.name {
            Ok(format!("Ctx ran to completion with context: {}", name))
        } else {
            todo!()
        }
    }
}
