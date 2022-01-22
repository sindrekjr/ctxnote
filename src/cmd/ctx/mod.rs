mod get;
mod init;
mod mv;
mod rm;

pub use self::init::CtxInitCmd;

use crate::CmdHandling;
use crate::Config;
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
    cmd: Option<Cmd>,
}

impl CmdHandling for CtxCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                Cmd::Get(get) => get.handle(config),
                Cmd::Init(init) => init.handle(config),
                Cmd::Mv(mv) => mv.handle(config),
                Cmd::Rm(rm) => rm.handle(config),
            }
        } else if let Some(name) = &self.name {
            Ok(format!("Ctx ran to completion with context: {}", name))
        } else {
            todo!()
        }
    }
}
