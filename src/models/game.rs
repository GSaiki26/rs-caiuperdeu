// Libs
use std::{sync::Arc, time::Duration};

use serenity::all::{
    Cache, ChannelId, Context, CreateEmbed, CreateMessage, EditMessage, GuildChannel, GuildId,
    Http, Mentionable, Message, UserId,
};
use tracing::info;

use super::player::Player;
use crate::messages::*;

// Types
type DateTimeUtc = chrono::DateTime<chrono::Utc>;

// Structs
pub struct Game {
    cache: Arc<Cache>,
    http: Arc<Http>,
    game_owner_id: UserId,
    channel_id: ChannelId,
    guild_id: GuildId,
    original_players: Vec<Player>,
    v_channel: Option<GuildChannel>,
    game_message: Option<Message>,
    start_dt: Option<DateTimeUtc>,
}

// Implementations
impl Game {
    /// A method to instance a new Game.
    pub fn new(ctx: Arc<Context>, original_message: Message) -> Self {
        Self {
            cache: ctx.cache.clone(),
            http: ctx.http.clone(),
            game_owner_id: original_message.author.id,
            channel_id: original_message.channel_id,
            guild_id: original_message.guild_id.unwrap(),
            v_channel: None,
            game_message: None,
            original_players: Vec::new(),
            start_dt: None,
        }
    }
    /// A method to check if the context is valid.
    /// It'll check if the user is in a voice channel and if there's more than one player in the voice channel.
    pub async fn is_ctx_valid(&mut self) -> serenity::Result<bool> {
        // Check if the user is in a voice channel.
        self.set_v_channel().await?;

        if self.v_channel.is_none() {
            let message = format!(
                "{} you're not in a voice channel.",
                self.game_owner_id.mention()
            );
            self.channel_id.say(&self.http, message).await?;
            return Ok(false);
        }

        // Check if there's more than one player in the voice channel.
        let members = self.v_channel.as_ref().unwrap().members(&self.cache)?;
        if members.len() < 2 {
            self.channel_id
                .say(
                    &self.http,
                    "The current voice channel doesn't have enough players.",
                )
                .await?;
            return Ok(false);
        }

        Ok(true)
    }

    /// A method to start the game. IT NEED TO BE CALLED AFTER THE is_ctx_valid METHOD.
    pub async fn start(&mut self) -> serenity::Result<()> {
        // Start the game.
        self.channel_id.say(&self.http, "Starting game...").await?;

        // Get and save all the original players.
        self.original_players = self.get_players_in_v_channel()?;
        self.start_dt = Some(chrono::Utc::now());

        // Create the loop to check the voice chat.
        let cooldown_time_ms: u64 = std::env::var("COOLDOWN_TIME_MS").unwrap().parse().unwrap();
        let mut alive_players: Vec<&Player>;
        loop {
            // Check the voice channel.
            info!("Checking the alive players...");
            let left_players = self.check_players().await?;

            // Send the leave message.
            info!("Sending the leave message...");
            for player in &left_players {
                let message = get_leave_message(player, self.start_dt.unwrap());
                self.channel_id.send_message(&self.http, message).await?;
            }

            info!("Editing the game message...");
            self.edit_game_message().await?;

            // Check if the game is over.
            // It makes the subtracting of the players because de discord server's delay.
            alive_players = self.get_alive_players();
            if alive_players.len() < 2 {
                break;
            }

            info!("Sleeping...");
            std::thread::sleep(Duration::from_millis(cooldown_time_ms));
        }

        // Check if nobody won.
        if alive_players.is_empty() {
            self.channel_id
                .say(&self.http, "Nobody won the game.")
                .await?;
            return Ok(());
        }

        // Send the win message.
        let mut winner = alive_players.pop().unwrap().clone();
        winner.end_dt = Some(chrono::Utc::now());
        let message = get_winner_message(&winner, self.start_dt.unwrap());
        self.channel_id.send_message(&self.http, message).await?;

        Ok(())
    }

    /// A method to check all the players in the voice chat.
    /// It's called in the game loop.
    /// Returns a Result with the players that left the voice chat.
    async fn check_players(&mut self) -> serenity::Result<Vec<Player>> {
        // Loop through all the alive players.
        info!("Checking players...");
        let current_players = self.get_players_in_v_channel()?;
        let alive_players = self.get_alive_players_mut();
        let mut left_players = Vec::new();
        for player in alive_players {
            // Check if the player is still in the voice channel.
            if current_players.contains(player) {
                continue;
            }

            // As the player left the voice channel, remove it from the alive players.
            info!("Player {} has left the voice channel.", player.user);
            player.end_dt = Some(chrono::Utc::now());
            left_players.push(player.clone());

            // Send the leave message.
        }

        Ok(left_players)
    }

    /// A method to edit the game message.
    /// The game need to be started before calling this method.
    async fn edit_game_message(&mut self) -> serenity::Result<()> {
        // Get the embed message.
        let message = self.get_embed_game_message().await?;

        // Check if the game message is not created yet.
        let game_message = self.game_message.clone();
        if game_message.is_none() {
            let message = CreateMessage::new().embed(message);
            self.game_message = Some(self.channel_id.send_message(&self.http, message).await?);
            return Ok(());
        }

        // Edit the game message.
        let message = EditMessage::new().embed(message);
        game_message.unwrap().edit(&self.http, message).await?;

        Ok(())
    }

    /// A method to create the embed game message.
    async fn get_embed_game_message(&self) -> serenity::Result<CreateEmbed> {
        // Define the message properties.
        let game_total_time = get_total_time(self.start_dt.unwrap(), chrono::Utc::now());
        let title = format!("[{}] Game Status 🎮🎯:", game_total_time);
        let description = "Current players:";
        let mut embed = CreateEmbed::new().title(title).description(description);

        // Add all the original players to the description.
        for player in &self.original_players {
            let mut player_name = String::from("❤️ ") + &player.user.global_name.clone().unwrap();
            // Check if the player is still playing.
            if player.end_dt.is_none() {
                embed = embed.field(player_name, "* Still alive.", false);
                continue;
            }

            let total_time = get_total_time(self.start_dt.unwrap(), player.end_dt.unwrap());
            player_name = player_name.replace("❤️ ", "💔 ");
            embed = embed.field(
                player_name,
                &format!("* Has lost with {}.", total_time),
                false,
            );
        }

        Ok(embed)
    }

    /// A method to get the voice channel the game's owner is.
    async fn set_v_channel(&mut self) -> serenity::Result<()> {
        // Get the owner's voice state.
        let guild = self.guild_id.to_guild_cached(&self.cache).unwrap().clone();
        let user_state = guild.voice_states.get(&self.game_owner_id);
        if user_state.is_none() {
            return Ok(());
        }

        // Get the voice channel.
        let current_owner_v_channel = user_state.unwrap().channel_id.unwrap();
        let v_channel = self.cache.channel(current_owner_v_channel).unwrap().clone();
        self.v_channel = Some(v_channel);

        Ok(())
    }

    /// A method to get all the players in a voice channel.
    /// It'll return a vector of Players. (Will convert members -> Player)
    fn get_players_in_v_channel(&self) -> serenity::Result<Vec<Player>> {
        let v_channel = self.v_channel.clone().unwrap();
        Ok(v_channel
            .members(&self.cache)?
            .iter()
            .map(|member| Player::new(member.user.clone()))
            .collect())
    }

    /// A method to get all the alive players in the game.
    /// It'll return a MUTABLE vector of Players.
    fn get_alive_players_mut(&mut self) -> Vec<&mut Player> {
        self.original_players
            .iter_mut()
            .filter(|p| p.end_dt.is_none())
            .collect()
    }

    /// A method to get all the alive players in the game.
    /// It'll return a vector of Players.
    fn get_alive_players(&self) -> Vec<&Player> {
        self.original_players
            .iter()
            .filter(|p| p.end_dt.is_none())
            .collect()
    }
}
