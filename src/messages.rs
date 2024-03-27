// Libs

use serenity::all::{CreateEmbed, CreateMessage, Mentionable};

use crate::models::player::Player;

// Types
type DateTimeUtc = chrono::DateTime<chrono::Utc>;

// Functions
///
/// A method to get the win message.
///
pub fn get_winner_message(winner: &Player, start_dt: DateTimeUtc) -> CreateMessage {
    let title = "CONGRATULATIONS ✨🎉!";
    let description = format!(
        "The player {} has won the game.\n\n",
        winner.user.id.mention()
    ) + &get_player_dt_description(winner, start_dt);
    let message = CreateEmbed::new().title(title).description(description);
    CreateMessage::new().embed(message)
}

///
/// A method to get the leave message.
/// The player can't be playing anymore.
///
pub fn get_leave_message(player: &Player, start_dt: DateTimeUtc) -> CreateMessage {
    let title = "Oh no... A player left the voice channel 🥺.";
    let description = format!("The player {} has left.\n\n", player.user.id.mention())
        + &get_player_dt_description(player, start_dt);
    let message = CreateEmbed::new().title(title).description(description);
    CreateMessage::new().embed(message)
}

///
/// A method to format the Embed description.
/// More precisely, the description of the start and end time of the game.
///
/// The player can't be playing anymore.
///
pub fn get_player_dt_description(player: &Player, start_dt: DateTimeUtc) -> String {
    let start_dt_str = start_dt.format("%Y-%m-%d %H:%M:%S").to_string();
    let end_dt = player.end_dt.unwrap();
    let end_dt_str = end_dt.format("%Y-%m-%d %H:%M:%S").to_string();
    let total_time_str = get_total_time(start_dt, end_dt);
    format!(
        "* **🗓️ Game start**: {}\n* **🗓️ End time**:      {}\n* **🕘 Total time**: {}",
        start_dt_str, end_dt_str, total_time_str
    )
}

///
/// A method to get and format the total time.
///
pub fn get_total_time(start_dt: DateTimeUtc, end_dt: DateTimeUtc) -> String {
    let total_time = (end_dt - start_dt).num_seconds();
    let hours = total_time / 3600;
    let minutes = (total_time % 3600) / 60;
    let seconds = total_time % 60;
    format!("{}H {}M {}S", hours, minutes, seconds)
}
