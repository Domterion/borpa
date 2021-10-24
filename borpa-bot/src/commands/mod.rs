mod misc;

use std::{collections::HashMap, sync::Arc};

use borpa_commands::{command::Command, handler::Handler};

pub fn get_commands() -> HashMap<String, Arc<Command>> {
    let mut commands_: Vec<Arc<Command>> = vec![];

    let pond = Arc::new(Command {
        name: "pond".to_string(),
        description: "Get bots pond".to_string(),
        options: vec![],
        subcommands: vec![],
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    let pong = Arc::new(Command {
        name: "pong".to_string(),
        description: "Get bots pong".to_string(),
        options: vec![],
        subcommands: vec![pond],
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });

    // TODO: Clean up defining commands by making a macro to help
    let ping = Arc::new(Command {
        name: "ping".to_string(),
        description: "Get bots ping".to_string(),
        options: vec![],
        subcommands: vec![pong],
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });
    commands_.push(ping);

    let mut commands: HashMap<String, Arc<Command>> = HashMap::new();

    for command in commands_ {
        let name = Handler::recurse_commands(command.subcommands.clone(), command.name.clone());

        // Inserts to the highest level, aka 3rd
        commands.insert(name, command.clone());

        for command_ in command.subcommands.clone() {
            // Inserts to the second highest level
            commands.insert(
                format!("{} {}", command.name, command_.name),
                command_.clone(),
            );
        }

        // Inserts to the root level, aka 1st
        commands.insert(command.name.clone(), command.clone());
    }

    println!("Commands: {:?}", commands.keys());

    commands
}
