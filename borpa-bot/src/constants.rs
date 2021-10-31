use std::{collections::HashMap, path::PathBuf, sync::Arc};

use borpa_commands::command::{Command, CommandKind, CommandType};
use borpa_utils::command;
use lazy_static::lazy_static;
use twilight_model::application::command::{
    BaseCommandOptionData, ChoiceCommandOptionData, CommandOption,
};

use crate::{commands::misc, config::Config};

lazy_static! {
    pub static ref CONFIG: Config = Config::new(PathBuf::from("config.toml"));
    pub static ref COMMANDS: HashMap<String, Arc<Command>> = {
        let ping = command!(
            "ping",
            "Get bots ping",
            vec![
                CommandOption::Boolean(BaseCommandOptionData {
                    name: "food?".to_string(),
                    description: "No".to_string(),
                    required: false
                }),
                CommandOption::String(ChoiceCommandOptionData {
                    choices: vec![],
                    name: "gibe".to_string(),
                    description: "Yes".to_string(),
                    required: false
                })
            ],
            CommandType::ChatInput,
            CommandKind::Command,
            misc::ping::ping
        );

        let pong = command!(
            "pong",
            "Get bots pong",
            vec![CommandOption::Boolean(BaseCommandOptionData {
                name: "pong?".to_string(),
                description: "Yes".to_string(),
                required: false
            })],
            CommandType::ChatInput,
            CommandKind::SubcommandGroup(ping.clone()),
            misc::ping::ping
        );

        let pond = command!(
            "pond",
            "Get bots pond",
            vec![CommandOption::Boolean(BaseCommandOptionData {
                name: "pond?".to_string(),
                description: "Yes".to_string(),
                required: false
            })],
            CommandType::ChatInput,
            CommandKind::Subcommand(pong.clone()),
            misc::ping::ping
        );

        let owner = command!(
            "owner",
            "Owner command to be used with a subcommand",
            vec![CommandOption::Boolean(BaseCommandOptionData {
                name: "no_owner?".to_string(),
                description: "Yes".to_string(),
                required: false
            })],
            CommandType::ChatInput,
            CommandKind::Command,
            misc::ping::ping
        );

        let eval = command!(
            "eval",
            "Execute a code snippet",
            vec![CommandOption::Boolean(BaseCommandOptionData {
                name: "maybe?".to_string(),
                description: "..".to_string(),
                required: false
            })],
            CommandType::ChatInput,
            CommandKind::Subcommand(owner.clone()),
            misc::ping::ping
        );

        let commands_ = vec![ping, pong, pond, owner, eval];

        let mut commands: HashMap<String, Arc<Command>> = HashMap::new();

        for command in commands_ {
            let name = command.get_qualified_name();
            commands.insert(name, command);
        }

        commands
    };
}
