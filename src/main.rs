// Libs
use std::process::exit;

use serenity::all::GatewayIntents;
use tracing::{error, info};

use handler::Handler;

mod handler;
mod messages;
mod models;

// Functions
fn is_env_valid() -> bool {
    let variables = vec!["COOLDOWN_TIME_MS", "DISCORD_TOKEN"];
    for variable in variables {
        let var = std::env::var(variable);
        if var.is_err() {
            error!("{} not set.", variable);
            return false;
        }
    }

    true
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if !is_env_valid() {
        exit(1);
    }

    // Define the bot's intents and get it's token in environment.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES;

    let token = std::env::var("DISCORD_TOKEN").unwrap();

    // Create and start the client.
    let mut client = serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .status(serenity::all::OnlineStatus::DoNotDisturb)
        .await
        .expect("Error creating bot");

    info!("Starting bot...");
    client.start().await.expect("Couldn\'t start bot");
}
