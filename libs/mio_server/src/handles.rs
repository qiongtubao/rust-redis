use std::collections::HashMap;
use crate::command::Command;
use std::error::Error;
use crate::db::Db;

pub type Handle = Fn(&mut Db, Command) -> Command;
pub struct Handles<'a> {
    handles: HashMap<String, &'a Handle>
}
impl<'a> Handles<'a> {
    pub fn new() -> Handles<'a> {
        Handles {
            handles: HashMap::new()
        }
    }

    pub fn exec(&self, db: &mut Db, command: Command) -> Option<Command> {
        let handle = self.handles.get(command.getArgc(0))?;
        Some(handle(db, command))
    }
    pub fn insert(&mut self, key: String, handle: &'a Handle) {
        self.handles.insert(key, handle);
    }
}