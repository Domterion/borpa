/*

For now, we will use async function references to call commands
;;

like this for the async function ref stuff

pub type CommandResultOuter = Pin<Box<dyn Future<Output = CommandResult> + Send>>;
pub type CommandHandler = Box<dyn Fn(CommandContext) -> CommandResultOuter + Send + Sync>;

;;

*/

pub mod command;
pub mod context;
pub mod handler;
