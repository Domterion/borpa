use twilight_http::Client;
use twilight_model::application::interaction::application_command::CommandDataOption;

pub struct Context {
    pub http: Client,
    pub interaction_id: u64,
    pub interaction_token: String,
    pub command_id: u64,
    pub options: Vec<CommandDataOption>,
}
