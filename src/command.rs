extern crate clap;

use clap::{App, ArgMatches};

pub type CommandResult = Result<(), String>;

pub trait Command {
    fn name(&self) -> &str;
    fn sub_command(&self) -> App;
    fn handle(&self, &ArgMatches) -> CommandResult;
}
