use std::sync::Arc;

use twilight_model::application::interaction::{
    application_command::CommandDataOption,
};

use crate::command::Command;

pub struct Handler {
    pub commands: Vec<Arc<Command>>,
}

impl Handler {
    pub fn new(commands: Vec<Arc<Command>>) -> Handler {
        Handler { commands }
    }

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
                None => break
            }
        }

        found
    }

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
