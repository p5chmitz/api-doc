mod create_user;
mod hello;
mod migrate;
mod serve;
//mod check;

use crate::settings::Settings;
use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(create_user::configure())
        //.subcommand(check::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;
    create_user::handle(matches, settings)?;
    //check::handle(matches, settings)?;

    Ok(())
}
