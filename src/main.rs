mod cmd;
mod conf;
mod ctx;
mod note;

use clap::Parser;
use cmd::{CmdHandling, NoteCmd};
use conf::Config;

fn main() {
    let config = match Config::get_user_config() {
        Ok(config) => config,
        Err(why) => {
            println!("error when reading user config: {}", why);
            println!("using default config");
            Config::default()
        },
    };

    match NoteCmd::parse().handle(&config) {
        Ok(output) => println!("{}", output),
        Err(error) => panic!("{}", error),
    }
}
