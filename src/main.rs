mod cmd;
mod conf;
mod ctx;
mod note;
mod util;

use clap::Parser;
use cmd::NoteCmd;

fn main() {
    match NoteCmd::parse().handle() {
        Ok(output) => println!("{}", output),
        Err(error) => panic!("{}", error),
    }
}
