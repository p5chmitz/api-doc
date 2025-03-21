use api_doc::{commands, settings}; // Its the project lib, Charlie Brown!
use clap::{Arg, Command};
use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();

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

    command = commands::configure(command);
    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "DOC")?;

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    commands::handle(&matches, &settings)?;

    println!(
        "db url: {}",
        settings
            .database
            .url
            .unwrap_or("missing database url".to_string())
    );

    println!(
        "log level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );

    Ok(())
}
