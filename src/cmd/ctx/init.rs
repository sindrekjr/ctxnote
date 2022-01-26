use crate::ctx::Context;
use crate::{CmdHandling, Config};
use clap::Parser;

#[derive(Parser)]
pub struct CtxInitCmd {
    name: String,

    #[clap(short, long)]
    bind: bool,
}

impl CmdHandling for CtxInitCmd {
    fn handle(&self, _config: &Config) -> Result<String, String> {
        let mut ctx = Context::new(self.name.to_string());
        if self.bind {
            ctx.bind(&std::env::current_dir().unwrap());
        }

        match ctx.register() {
            Ok(msg) => Ok(format!(
                "CtxInit ran to completion with new context: {:?}{}",
                ctx,
                match msg {
                    Some(msg) => format!("\n{}", msg),
                    None => "".to_string(),
                }
            )),
            Err(why) => Err(why.to_string()),
        }
    }
}
