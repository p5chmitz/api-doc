use api_doc::{commands, settings}; // Its the project lib, Charlie Brown!
use clap::{Arg, Command};
use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
//use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn main() -> anyhow::Result<()> {
    // Loads the .env file
    dotenv().ok();

    // Defines a global -c/--config command
    // Global commands must come before sub-commands in the CLI
    let mut command = Command::new("API docs")
        .version("0.1.0")
        .author("Peter Schmitz <petermschmitz@gmail.com>")
        .about("Just an excuse to play with Rust-based APIs and documentation")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );

    // Custom configuration function in commands/mod.rs to parse sub-commands
    command = commands::configure(command);
    let matches = command.get_matches();

    // Parses the optional config path, already set to config.json
    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    // Creates a src/settings.Settings object to load values prefixed with DOC__
    let settings = settings::Settings::new(config_location, "DOC")?;

    // Dynamically parses the .env file for log level tracing
    // Otherwise sets the default to DEBUG
    use std::str::FromStr;
    let log_level = settings
        .logging
        .log_level
        .as_deref()
        .and_then(|lvl| LevelFilter::from_str(lvl).ok())
        .unwrap_or(LevelFilter::DEBUG);

    // Sets up the service tracing, defaulting to a relatively verbose "DEBUG" level
    let subscriber = Registry::default()
        //.with(LevelFilter::from_level(Level::DEBUG)) // Hard codes logging level
        .with(log_level)
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    commands::handle(&matches, &settings)?;

    Ok(())
}
