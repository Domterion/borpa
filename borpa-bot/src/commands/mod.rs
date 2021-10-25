mod misc;

use std::{collections::HashMap, sync::Arc};

use borpa_commands::command::{Command, CommandKind, CommandType};

pub fn get_commands() -> HashMap<String, Arc<Command>> {
    let mut commands_: Vec<Arc<Command>> = vec![];

    let ping = Arc::new(Command {
        name: "ping".to_string(),
        description: "Get bots ping".to_string(),
        options: vec![],
        r#type: CommandType::ChatInput,
        kind: CommandKind::Command,
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    let pong = Arc::new(Command {
        name: "pong".to_string(),
        description: "Get bots pong".to_string(),
        options: vec![],
        r#type: CommandType::ChatInput,
        kind: CommandKind::SubcommandGroup(ping.clone()),
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    let pond = Arc::new(Command {
        name: "pond".to_string(),
        description: "Get bots pond".to_string(),
        options: vec![],
        r#type: CommandType::ChatInput,
        kind: CommandKind::Subcommand(pong.clone()),
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    let owner = Arc::new(Command {
        name: "owner".to_string(),
        description: "Owner command to be used with a subcommand".to_string(),
        options: vec![],
        r#type: CommandType::ChatInput,
        kind: CommandKind::Command,
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    let eval = Arc::new(Command {
        name: "eval".to_string(),
        description: "Execute a code snippet".to_string(),
        options: vec![],
        r#type: CommandType::ChatInput,
        kind: CommandKind::Subcommand(owner.clone()),
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    commands_.push(ping);
    commands_.push(pong);
    commands_.push(pond);
    commands_.push(owner);
    commands_.push(eval);

    let mut commands: HashMap<String, Arc<Command>> = HashMap::new();

    for command in commands_ {
        let name = command.get_qualified_name();
        commands.insert(name, command);
    }

    commands
}
