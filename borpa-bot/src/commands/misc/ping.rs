use borpa_commands::{command::CommandReturn, context::Context};

pub async fn ping(_ctx: Context) -> CommandReturn {
    println!("Ping command called");

    Ok(())
}
