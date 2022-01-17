use crate::cmd::CmdHandling;
use crate::ctx::Context;
use clap::Parser;

#[derive(Parser)]
pub struct CtxInitCmd {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(short, long)]
    bind: bool,
}

impl CmdHandling for CtxInitCmd {
    fn handle(&self) -> Result<String, String> {
        let mut ctx = match &self.name {
            Some(name) => Context::new(name.to_string()),
            None => {
                let dir = std::env::current_dir().unwrap();
                Context::new(dir.file_name().unwrap().to_str().unwrap().to_owned())
            }
        };

        if self.bind {
            ctx.bind(std::env::current_dir().unwrap());
        }

        Ok(format!(
            "CtxInit ran to completion with new context: {:?}",
            ctx
        ))
    }
}
