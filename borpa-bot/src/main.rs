mod commands;

use borpa_commands::handler::Handler;

#[tokio::main]
async fn main() {
    let commands = commands::get_commands();

    let handler = Handler::new(commands);

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
