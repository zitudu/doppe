extern crate clap;

mod command;

use clap::App;
use command::Command;
use std::iter::Iterator;

fn main() {
    let sub_commands: Vec<Box<Command>> = vec![];
    let matches = App::new("doppe")
        .version("0.1.0")
        .author("zitudu <zitudu@protonmail.com>")
        .about("Command Line Tool for dispatch works to remote hosts")
        .subcommands(
            sub_commands
                .iter()
                .map(|ref sub_command| sub_command.sub_command()),
        )
        .get_matches();
    for sub_command in &sub_commands {
        if let Some(matches) = matches.subcommand_matches(sub_command.name()) {
            if let Err(reason) = sub_command.handle(matches) {
                eprintln!("Failed: {}", reason);
            }
            break;
        }
    }
}
