use std::{collections::HashMap, sync::Arc};

use crate::command::Command;

/// The struct to handle parsing commands
pub struct Handler {
    /// A hashamp of string and arc wrapped command that the handler should know of
    pub commands: HashMap<String, Arc<Command>>,
}

impl Handler {
    /// Returns the handler
    ///
    /// Constructs a new handler
    ///
    /// # Arguments
    ///
    /// * `commands` - A hashmap of string and arc wrapped command
    pub fn new(commands: HashMap<String, Arc<Command>>) -> Handler {
        Handler { commands }
    }

    /// Returns an optional arc wrapped command
    ///
    /// Looks through all handler aware commands for a command matching the name.
    ///
    /// # Arguments
    ///
    /// * `to_find` - The name of the command to find
    pub fn find_command(&self, to_find: String) -> Option<Arc<Command>> {
        self.commands.get(&to_find).cloned()
    }
}
