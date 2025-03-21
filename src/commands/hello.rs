//use crate::settings::Settings;
//use clap::{ArgMatches, Command};
//
///// Vestigial configuration operation for a more rudimentary project state
//pub fn configure() -> Command {
//    Command::new("hello").about("Prints a greeting")
//}
//
///// Vestigial handler operation for a more rudimentary project state
//pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
//    if let Some(_matches) = matches.subcommand_matches("hello") {
//        println!("Hello!\nThats... all I've got");
//    }
//
//    Ok(())
//}
