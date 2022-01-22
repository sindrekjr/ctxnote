mod get;
mod set;

use crate::CmdHandling;
use crate::Config;
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
    fn handle(&self, config: &Config) -> Result<String, String> {
        match &self.cmd {
            Cmd::Get(get) => get.handle(config),
            Cmd::Set(set) => set.handle(config),
        }
    }
}
