use std::{error::Error, future::Future, pin::Pin, sync::Arc};

use twilight_model::application::interaction::application_command::CommandDataOption;

use crate::context::Context;

pub type CommandReturn = Result<(), Box<dyn Error + Sync + Send>>;
pub type CommandReturnOuter = Pin<Box<dyn Future<Output = CommandReturn> + Send>>;
pub type CommandHandler = Box<dyn Fn(Context) -> CommandReturnOuter + Send + Sync>;

pub struct Command {
    pub name: String,
    pub description: String,
    pub options: Vec<CommandDataOption>,
    pub subcommands: Vec<Arc<Command>>,
    pub handler: CommandHandler,
}
