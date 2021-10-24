use std::{collections::HashMap, sync::Arc};

use twilight_model::application::interaction::application_command::CommandDataOption;

use crate::command::Command;

/// The struct to handle parsing commands
pub struct Handler {
    /// A hashamp of string and arc wrapped command that the handler should know of
    pub commands: HashMap<String, Arc<Command>>,
}

impl Handler {
    /// Returns the handler
    ///
    /// Constructs a new handler
    ///
    /// # Arguments
    ///
    /// * `commands` - A hashmap of string and arc wrapped command
    pub fn new(commands: HashMap<String, Arc<Command>>) -> Handler {
        Handler { commands }
    }

    /// Returns an optional arc wrapped command
    ///
    /// Looks through all handler aware commands for a command matching the name.
    ///
    /// # Arguments
    ///
    /// * `to_find` - The name of the command to find
    pub fn find_command(&self, to_find: String) -> Option<Arc<Command>> {
        println!("{:?} | {:?}", self.commands.keys(), to_find);
        self.commands.get(&to_find).cloned()
    }

    /// Returns a string
    ///
    /// Recurses through a vector of commands to construct a full string with subcommand names
    ///
    /// # Arguments
    ///
    /// * `commands` - A vector of arc wrapped command
    /// * `name` - The root command name
    pub fn recurse_commands(commands: Vec<Arc<Command>>, name: String) -> String {
        for command in commands {
            //if let CommandDataOption::SubCommand { name, options } = option {
            //    return Handler::recurse_command_options(options, format!("{} {}", command, name));
            //}

            return Handler::recurse_commands(
                command.subcommands.clone(),
                format!("{} {}", name, command.name),
            );
        }

        name
    }
}
