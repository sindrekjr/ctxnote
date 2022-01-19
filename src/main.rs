mod cmd;
mod conf;
mod ctx;
mod note;

use clap::Parser;
use cmd::{CmdHandling, NoteCmd};

fn main() {
    match NoteCmd::parse().handle() {
        Ok(output) => println!("{}", output),
        Err(error) => panic!("{}", error),
    }
}
