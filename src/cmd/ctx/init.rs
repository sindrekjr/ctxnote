use crate::cmd::CmdHandling;
use clap::Parser;

#[derive(Parser)]
pub struct CtxInitCmd {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(short, long)]
    bind: Option<bool>,
}

impl CmdHandling for CtxInitCmd {
    fn handle(&self) -> Result<&str, &str> {
        todo!()
    }
}
