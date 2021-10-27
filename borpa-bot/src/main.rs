mod commands;
mod config;
mod constants;

use borpa_commands::handler::Handler;

#[tokio::main]
async fn main() {
    let handler = Handler::new(constants::COMMANDS.clone());

    let cmd = handler.find_command("owner".to_string());

    if let Some(c) = cmd {
        println!(
            "Found {} command with description {}",
            c.name, c.description
        );
        //let h = &c.handler;
        //h();
    } else {
        println!("Command not found");
    }
}
