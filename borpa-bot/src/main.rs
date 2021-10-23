mod commands;

use borpa_commands::handler::Handler;

#[tokio::main]
async fn main() {

    let commands = commands::get_commands();

    let handler = Handler::new(commands);

    // let s = handler.find_command("pong ping".to_string());

    // if let Some(c) = s {
    //     println!("Found {} command with description {}", c.name, c.description);
    //     let h = &c.handler;
    //     h();
    // } else {
    //     println!("Command not found");
    // }

    println!("Hello, world!");
}