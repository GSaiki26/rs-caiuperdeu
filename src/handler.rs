// Libs
use std::sync::Arc;

use serenity::{
    all::{
        ChannelId, Command, CommandInteraction, Context, CreateCommand, CreateEmbed,
        CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateMessage, EventHandler, GuildId, Http, Interaction, User,
    },
    async_trait,
};
use tracing::{error, info, info_span, Instrument};

use crate::models::game::Game;

// Structs
pub struct Handler;

// Implementations
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: serenity::all::Ready) {
        info!("Bot is ready.");

        // Define the slash commands.
        let ping_command =
            CreateCommand::new("ping").description("A command to check if the bot is online.");
        if let Err(e) = Command::create_global_command(&ctx.http, ping_command).await {
            error!("Couldn\'t create the slash command: 'ping'. {}", e);
        }
        let play_command = CreateCommand::new("play").description("A command to start a new game.");
        if let Err(e) = Command::create_global_command(&ctx.http, play_command).await {
            error!("Couldn\'t create the slash command: 'play'. {}", e);
        }
    }

    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<serenity::all::GuildId>) {
        info!("Cache is ready.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let ctx = Arc::new(ctx);

        // Get the command interaction.
        let mut command: Option<CommandInteraction> = None;
        if let Interaction::Command(received_command) = interaction {
            command = Some(received_command);
        }
        if command.is_none() {
            return;
        }

        // Check if the interaction is valid.
        let command = command.unwrap();
        if command.user.bot | command.guild_id.is_none() {
            return;
        }

        // End the interaction, so the user knows that the bot is processing the command.
        let message = CreateInteractionResponseMessage::new().content("Processing...");
        let message = CreateInteractionResponse::Message(message);
        if let Err(e) = command.create_response(&ctx.http, message).await {
            error!("Couldn\'t create the interaction response. {}", e);
            return;
        }

        // Match the command.
        let span = info_span!("EVENT", guild = %command.guild_id.unwrap());
        let _guard = span.enter();
        match command.data.name.as_str() {
            "play" => {
                command_play(
                    ctx,
                    command.guild_id.unwrap(),
                    command.user,
                    command.channel_id,
                )
                .instrument(span.clone())
                .await;
            }
            "ping" => {
                command_ping(ctx.http.clone(), command.user, command.channel_id)
                    .instrument(span.clone())
                    .await;
            }
            _ => {
                info!("The sent command is not valid.");
                if let Err(e) = command
                    .channel_id
                    .say(&ctx.http, "Command not found.")
                    .await
                {
                    error!("Couldn\'t send the message: 'Command not found.'. {}", e);
                }
            }
        }
    }
}

// Functions
async fn command_ping(http: Arc<Http>, user: User, channel_id: ChannelId) {
    info!("Ping command detected.");
    let author = CreateEmbedAuthor::new(&user.name).icon_url(user.avatar_url().unwrap());

    // Create the ping message.
    let message = CreateEmbed::new()
        .author(author)
        .description("CaiuPerdeu is online.");
    let message = CreateMessage::new().embed(message);
    if let Err(why) = channel_id.send_message(http, message).await {
        error!("Error sending message: {:?}", why);
    }
}

async fn command_play(ctx: Arc<Context>, guild_id: GuildId, user: User, channel_id: ChannelId) {
    info!("Play command detected.");

    // Create a new game.
    let mut game = Game::new(ctx, guild_id, user.id, channel_id);
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
