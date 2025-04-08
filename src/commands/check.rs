use crate::settings::Settings;
//use crate::state::ApplicationState;
//use axum::Router;
//use clap::{value_parser, Arg, ArgMatches, Command};
use clap::{ArgMatches, Command};
//use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//use std::sync::Arc;
//use tower_http::trace::TraceLayer;
//use sea_orm::{ActiveModelTrait, Database, EntityTrait};
use sea_orm::Database;

use migration::{Migrator, MigratorTrait};

/// Convenience operation to check necessary service components
pub fn configure() -> Command {
    Command::new("check").about("Convenience operation to check necessary service components")
}

pub async fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("check") {
        let db_url = settings.database.url.clone().unwrap_or("".to_string());

        println!("\nChecking database connection...\n");

        // Attempt to establish a database connection
        match Database::connect(&db_url).await {
            Ok(db_conn) => {
                println!("✅ Database connection successful");

                // Print applied migrations
                match Migrator::get_applied_migrations(&db_conn).await {
                    Ok(applied) => {
                        println!("📦 Applied migrations: {}", applied.len());
                    }
                    Err(e) => {
                        println!("⚠️ Error retrieving applied migrations: {e}");
                    }
                }

                // Print pending migrations
                match Migrator::get_pending_migrations(&db_conn).await {
                    Ok(pending) => {
                        if pending.is_empty() {
                            println!("✅ All migrations are up to date.");
                        } else {
                            println!("⚠️ {} pending migration(s) detected:", pending.len());
                            for m in &pending {
                                println!("- {}", m.name());
                            }
                        }
                    }
                    Err(e) => {
                        println!("⚠️ Error retrieving pending migrations: {e}");
                    }
                }
            }
            Err(e) => {
                println!("⚠️ Database connection failed: {e}");
            }
        }

        // Parsing and printing DB connection information
        if let Some(connection) = parse_db_url(&db_url) {
            println!("\nDB connection details:");
            println!("   prefix: {}", connection.prefix);
            println!("   username: {}", connection.username);
            println!("   password: {}", connection.password);
            println!("   host: {}", connection.host);
            println!("   port: {}", connection.port.unwrap_or(5432));
            println!("   DB name: {}", connection.db_name);
        } else {
            println!("⚠️ Unable to parse DB URL.");
        }

        // Printing the log level
        println!(
            "Tracing log level: {}",
            settings
                .logging
                .log_level
                .clone()
                .unwrap_or_else(|| "info".to_string())
        );

        // Printing the token timeout in seconds
        println!(
            "Token timeout (seconds): {}",
            settings.token_timeout_seconds.clone()
        );

        // Printing the API documentation URLs (Swagger UI and Raw OAS)
        if let Some(connection) = parse_db_url(&db_url) {
            println!("Doc URLs (when server is running): ");
            println!("   Swagger UI: http://{}:8080{}", connection.host, crate::api::SWAGGER);
            println!("   Raw OAS: http://{}:8080{}", connection.host, crate::api::JSON);
        }
        println!();
    }

    Ok(())
}

use regex::Regex;

#[derive(Clone, Debug)]
struct DbConnection {
    prefix: String,
    username: String,
    password: String,
    host: String,
    port: Option<u16>,
    db_name: String,
}

fn parse_db_url(url: &str) -> Option<DbConnection> {
    // Regex that handles optional port
    let re = Regex::new(r"(?P<prefix>^[a-z]+://)(?P<username>[^:]+):(?P<password>[^@]+)@(?P<host>[^:/]+)(?::(?P<port>\d+))?/(?P<db_name>[^/?#]+)").unwrap();
    re.captures(url).map(|caps| {
        let port = caps
            .name("port")
            .map(|m| m.as_str().parse::<u16>().unwrap());
        DbConnection {
            prefix: caps["prefix"].to_string(),
            username: caps["username"].to_string(),
            password: caps["password"].to_string(),
            host: caps["host"].to_string(),
            port,
            db_name: caps["db_name"].to_string(),
        }
    })
}
