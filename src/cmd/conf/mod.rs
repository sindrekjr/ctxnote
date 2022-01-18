mod get;
mod set;

use crate::CmdHandling;
use clap::Parser;

#[derive(Parser)]
enum Cmd {
    Get(get::ConfGetCmd),
    Set(set::ConfSetCmd),
}

#[derive(Parser)]
pub struct ConfCmd {
    #[clap(subcommand)]
    cmd: Cmd,
}

impl CmdHandling for ConfCmd {
    fn handle(&self) -> Result<String, String> {
        match &self.cmd {
            Cmd::Get(get) => get.handle(),
            Cmd::Set(set) => set.handle(),
        }
    }
}
