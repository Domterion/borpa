mod misc;

use std::sync::Arc;

use borpa_commands::command::Command;

pub fn get_commands() -> Vec<Arc<Command>> {
    let mut commands: Vec<Arc<Command>> = vec![];

    // TODO: Clean up defining commands by making a macro to help
    let ping = Arc::new(Command {
        name: "ping".to_string(),
        description: "Get bots ping".to_string(),
        options: vec![],
        subcommands: vec![],
        handler: Box::new(move |ctx| Box::pin(misc::ping::ping(ctx))),
    });
    commands.push(ping);

    commands
}