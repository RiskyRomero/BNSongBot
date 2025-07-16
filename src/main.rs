use std::{sync::Arc, time::Duration};

use tokio::sync::Mutex;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, GuildId};
use rusqlite::Connection;

mod checks;
mod commands;

struct Data {
    db: Arc<Mutex<Connection>>, // Wrap Connection in Mutex for thread safety
    mod_role_id: serenity::RoleId, // ID of the moderator role
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection = rusqlite::Connection::open("data.db").expect("Failed to open database");

    // Create the table if it doesn't exist
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS songs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                album TEXT 
            )",
            [],
        )
        .unwrap();

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS albums (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              name TEXT
            )",
            [],
        )
        .unwrap();

    // Wrap the connection in Arc<Mutex<Connection>> for thread safety
    let shared_db = Arc::new(Mutex::new(connection));

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let guild_id_num = std::env::var("GUILD_ID").expect("`GUILD_ID` env variable is not set!");
    let guild_id = guild_id_num
        .parse::<GuildId>()
        .expect("Invalid `GUILD_ID` env var");

    let mod_role_id: serenity::RoleId = std::env::var("MOD_ROLE_ID")
        .expect("Failed to get 'MOD_ROLE_ID' from .env file")
        .parse::<u64>()
        .expect("Failed to parse 'MOD_ROLE_ID' as u64")
        .into();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::help::help(),
                commands::new::new(),
                commands::list::list(),
                commands::random::random(),
                commands::delete::delete(),
                commands::album::album(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    Duration::from_secs(3600),
                ))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)), // on_error must be Send
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                Ok(Data {
                    db: shared_db.clone(),
                    mod_role_id,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
