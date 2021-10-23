use std::{error::Error, future::Future, pin::Pin, sync::Arc};

use twilight_model::application::interaction::application_command::CommandDataOption;

use crate::context::Context;

pub type CommandReturn = Result<(), Box<dyn Error + Sync + Send>>;
pub type CommandReturnOuter = Pin<Box<dyn Future<Output = CommandReturn> + Send>>;
pub type CommandHandler = Box<dyn Fn(Context) -> CommandReturnOuter + Send + Sync>;

/// A common command object
pub struct Command {
    /// The name of the commmand
    pub name: String,
    /// The description of the command
    pub description: String,
    /// A vector of command options
    pub options: Vec<CommandDataOption>,
    /// A vector of arc wrapped commands
    pub subcommands: Vec<Arc<Command>>,
    /// The function used to handle the command
    pub handler: CommandHandler,
}
