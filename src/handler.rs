// Libs
use std::sync::Arc;

use serenity::{
    all::{Context, CreateEmbed, CreateEmbedAuthor, CreateMessage, EventHandler, Http, Message},
    async_trait,
};
use tracing::{error, info, info_span, Instrument};

use crate::models::game::Game;

// Structs
pub struct Handler;

// Implementations
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: serenity::all::Ready) {
        info!("Bot is ready.");
    }

    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<serenity::all::GuildId>) {
        info!("Cache is ready.");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let ctx = Arc::new(ctx);

        // Check if the message is valid.
        if msg.author.bot || msg.is_private() || !msg.content.starts_with('!') {
            return;
        }

        let span = info_span!("EVENT", guild = %msg.guild_id.unwrap());
        let _guard = span.enter();

        match msg.content.as_str() {
            "!ping" => {
                command_ping(ctx.http.clone(), msg)
                    .instrument(span.clone())
                    .await;
            }
            "!play" => {
                command_play(ctx, msg).instrument(span.clone()).await;
            }
            _ => {
                info!("The sent command is not valid.");
                if let Err(e) = msg.reply(&ctx.http, "Command not found.").await {
                    error!("Couldn\'t send the message: 'Command not found.'. {}", e);
                }
            }
        }
    }
}

// Functions
async fn command_ping(http: Arc<Http>, msg: Message) {
    info!("Ping command detected.");
    let channel = msg.channel_id;
    let author =
        CreateEmbedAuthor::new(&msg.author.name).icon_url(msg.author.avatar_url().unwrap());

    // Create the ping message.
    let message = CreateEmbed::new()
        .author(author)
        .description("CaiuPerdeu is online.");
    let message = CreateMessage::new().embed(message);
    if let Err(why) = channel.send_message(http, message).await {
        error!("Error sending message: {:?}", why);
    }
}

async fn command_play(ctx: Arc<Context>, msg: Message) {
    info!("Play command detected.");

    // Create a new game.
    let mut game = Game::new(ctx, msg);
    match game.is_ctx_valid().await {
        Ok(true) => {
            info!("Game context is valid, starting game...");
        }
        Ok(false) => {
            info!("The user couldn\'t start the game because bad context.");
            return;
        }
        Err(e) => {
            error!("Couldn\'t check if the owner's context is valid. {}", e);
            return;
        }
    }

    match game.start().await {
        Ok(()) => info!("The game has ended."),
        Err(e) => error!("Couldn\'t complete the game. {}", e),
    }
}
