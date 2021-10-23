use std::sync::Arc;

use twilight_model::application::interaction::application_command::CommandDataOption;

use crate::command::Command;

/// The struct to handle parsing commands
pub struct Handler {
    /// Vector of arc wrapped commands that the handler should know of
    pub commands: Vec<Arc<Command>>,
}

impl Handler {
    /// Returns the handler
    /// 
    /// Constructs a new handler
    /// 
    /// # Arguments
    ///
    /// * `commands` - A vector of arc wrapped commands
    pub fn new(commands: Vec<Arc<Command>>) -> Handler {
        Handler { commands }
    }

    /// Returns an optional arc wrapped command
    /// 
    /// Looks through all handler aware commands for a command matching the name. 
    /// This is subcommand aware and will respect subcommands, otherwise it will find a command matching the root
    /// 
    /// # Arguments
    ///
    /// * `to_find` - The name of the command to find
    pub fn find_command(&self, to_find: String) -> Option<Arc<Command>> {
        let mut commands = &self.commands;
        let mut found: Option<Arc<Command>> = None;
        let mut index = 0;
        let split = to_find.split(' ').collect::<Vec<&str>>();

        while index < to_find.len() {
            match commands.iter().find(|c| c.name == split[index]) {
                Some(command) => {
                    index += 1;

                    found = Some(command.clone());
                    commands = &command.subcommands;
                }
                None => break,
            }
        }

        found
    }

    /// Returns a string
    /// 
    /// Recurses through a vector of command options to construct a full string with subcommand names
    /// 
    /// # Arguments
    ///
    /// * `options` - A vector of command options
    /// * `command` - The root command name
    pub fn recurse_command_options(
        &self,
        options: Vec<CommandDataOption>,
        command: String,
    ) -> String {
        for option in options {
            if let CommandDataOption::SubCommand { name, options } = option {
                return self.recurse_command_options(options, format!("{} {}", command, name));
            }
        }

        command
    }
}
