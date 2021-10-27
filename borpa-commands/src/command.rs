use std::{error::Error, future::Future, pin::Pin, sync::Arc};

use twilight_model::application::command::{CommandOption, CommandType as TwilightCommandType};

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
    pub options: Vec<CommandOption>,
    /// The type of the command
    pub r#type: CommandType,
    /// The kind of command, describes the nesting of the command ie command, subcommand or subcommand group
    pub kind: CommandKind,
    /// The function used to handle the command
    pub handler: CommandHandler,
}

/// An enum of valid command types
pub enum CommandKind {
    /// Root command type
    ///
    /// The top level command
    Command,
    /// Subcommand type
    ///
    /// This should be used when nested under a subcommand group or is right after root command
    Subcommand(Arc<Command>),
    /// SubcommandGroup type
    ///
    /// This should be used when the command will hold a subcommand
    SubcommandGroup(Arc<Command>),
}

#[derive(Clone, Copy)]
/// Command type
pub enum CommandType {
    /// When the command is a slash command
    ChatInput,
    /// A command that shows when you right click a user
    User,
    /// A command that shows when you right click a message
    Message,
}

impl From<CommandType> for TwilightCommandType {
    fn from(val: CommandType) -> Self {
        match val {
            CommandType::ChatInput => TwilightCommandType::ChatInput,
            CommandType::User => TwilightCommandType::User,
            CommandType::Message => TwilightCommandType::Message,
        }
    }
}

impl Command {
    /// Returns a string
    ///
    /// Gets the fully qualified name for the command aka the command with all subcommands and the
    /// root
    pub fn get_qualified_name(&self) -> String {
        let name = match &self.kind {
            CommandKind::Command => self.name.clone(),
            CommandKind::Subcommand(c) => {
                if let CommandKind::SubcommandGroup(c_) = &c.kind {
                    format!("{} {} {}", c_.name, c.name, self.name)
                } else {
                    format!("{} {}", c.name, self.name)
                }
            }
            CommandKind::SubcommandGroup(c) => format!("{} {}", c.name, self.name),
        };

        name
    }
}
