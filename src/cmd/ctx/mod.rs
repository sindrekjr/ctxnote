mod init;
mod list;
mod mv;
mod rm;
mod set;

pub use self::init::CtxInitCmd;

use crate::cmd::CmdHandling;
use crate::io::conf::Config;
use clap::Parser;

#[derive(Parser)]
enum Cmd {
    Init(init::CtxInitCmd),
    List(list::CtxListCmd),
    Mv(mv::CtxMvCmd),
    Rm(rm::CtxRmCmd),
    Set(set::CtxSetCmd),
}

#[derive(Parser)]
pub struct CtxCmd {
    #[clap(subcommand)]
    cmd: Cmd,
}

impl CmdHandling for CtxCmd {
    fn handle(&self, config: &Config) -> Result<String, String> {
        match &self.cmd {
            Cmd::Init(init) => init.handle(config),
            Cmd::List(list) => list.handle(config),
            Cmd::Mv(mv) => mv.handle(config),
            Cmd::Rm(rm) => rm.handle(config),
            Cmd::Set(set) => set.handle(config),
        }
    }
}
