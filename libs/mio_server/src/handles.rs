use std::collections::HashMap;
use crate::command::Command;
use std::error::Error;

pub type Handle = Fn(Command) -> Command;
pub struct Handles<'a> {
    handles: HashMap<String, &'a Handle>
}
impl<'a> Handles<'a> {
    pub fn new() -> Handles<'a> {
        Handles {
            handles: HashMap::new()
        }
    }

    pub fn exec(&self, command: Command) -> Option<Command> {
        let handle = self.handles.get(command.getArgc(0))?;
        Some(handle(command))
    }
    pub fn insert(&mut self, key: String, handle: &'a Handle) {
        self.handles.insert(key, handle);
    }
}