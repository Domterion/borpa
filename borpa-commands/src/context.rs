use std::sync::Arc;

use twilight_http::Client;
use twilight_model::application::interaction::application_command::CommandDataOption;

/// A struct with common data used in commands
pub struct Context {
    /// The http client of the bot
    pub http: Arc<Client>,
    /// The id of the interaction from Discord
    pub interaction_id: u64,
    /// The token of the interaction from Discord
    pub interaction_token: String,
    /// The commands unique identifier from Discord
    pub command_id: u64,
    /// The options for the command that the user provided
    pub options: Vec<CommandDataOption>,
}
