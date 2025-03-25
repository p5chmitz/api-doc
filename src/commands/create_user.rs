use crate::settings::Settings;
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use serde_json::json;

use crate::entities;

use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, Database, EntityTrait};
//use serde_json::json;

use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};

pub fn configure() -> Command {
    Command::new("createuser")
        .about("Supply optional values to create a new user; If you supply no values the service attempts to use the documented default values; The system only allows unique usernames, including the default \"admin\" username value")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .value_name("USER_NAME")
                .help("Identifier for new user")
                .default_value("admin"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Password for new user")
                .default_value("apidocpass"),
        )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("createuser") {
        let username = matches.get_one::<String>("username").unwrap();
        let password = matches.get_one::<String>("password").unwrap();

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url
                    .clone().unwrap_or("".to_string());
                let conn: sea_orm::DatabaseConnection = 
                    Database::connect(db_url)
                    .await
                    .expect("Database connection failed");

                let users: Vec<entities::user::Model> = 
                    entities::user::Entity::find()
                    .filter(entities::user::Column::Username
                    .eq(username))
                    .all(&conn)
                    .await?;
                
                // Early return if the service matches
                // on an existing username
                if !users.is_empty() {
                    println!("User already exists");
                    return Ok(());
                }

                let encrypted_password = encrypt_password(password)?;
                let admin_model = 
                    entities::user::ActiveModel::from_json(json!({
                    "username": username,
                    "password": encrypted_password,
                }))?;

                // save() creates a new table entry if supplied
                // values do not match, and updates the entry
                // if they do
                if let Ok(_admin) = admin_model.save(&conn).await {
                    println!("User created");
                } else {
                    println!("Failed to create user");
                }

                Ok::<(), anyhow::Error>(())
            })?;
    }

    Ok(())
}

fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash.to_string())
    } else {
        Err(anyhow!("Failed to hash password"))
    }
}
