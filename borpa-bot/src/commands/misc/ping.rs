use borpa_commands::{command::CommandReturn, context::Context};

pub async fn ping(ctx: Context) -> CommandReturn {
    println!("Ping command called");

    Ok(())
}
