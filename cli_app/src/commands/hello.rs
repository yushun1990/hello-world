use clap::{ArgMatches, Command};

use crate::settings::Settings;

pub const COMMAND_NAME: &'static str = "hello";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Hello World!")
}

pub fn handle(_matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    println!("Hello World!");

    Ok(())
}
