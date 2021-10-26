mod misc;

use std::{collections::HashMap, sync::Arc};

use borpa_commands::command::{Command, CommandKind, CommandType};
use borpa_utils::command;

pub fn get_commands() -> HashMap<String, Arc<Command>> {
    let ping = command!(
        "ping",
        "Get bots ping",
        vec![],
        CommandType::ChatInput,
        CommandKind::Command,
        misc::ping::ping
    );

    let pong = command!(
        "pong",
        "Get bots pong",
        vec![],
        CommandType::ChatInput,
        CommandKind::SubcommandGroup(ping.clone()),
        misc::ping::ping
    );

    let pond = command!(
        "pond",
        "Get bots pond",
        vec![],
        CommandType::ChatInput,
        CommandKind::Subcommand(pong.clone()),
        misc::ping::ping
    );

    let owner = command!(
        "owner",
        "Owner command to be used with a subcommand",
        vec![],
        CommandType::ChatInput,
        CommandKind::Command,
        misc::ping::ping
    );

    let eval = command!(
        "eval",
        "Execute a code snippet",
        vec![],
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
}
