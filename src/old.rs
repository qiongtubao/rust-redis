use server::{Server, Config, Command, Db, Value, One};
use std::sync::{RwLock, Arc};
#[macro_use]
use server::command;
#[cmd("get")]
fn get(db: Arc<RwLock<Db>>, command: &Command) -> Command {
    if command.argc < 2 {
        return Command::error("param ".to_string());
    }
    let d = db.write().expect("");
    let value = d.get(command.getArgc(1)).expect("value");
    match value {
        Value::String(x) => {
            return command!(x.clone());
        },
        _ => {
            return  Command::error("type not string".to_string());
        }
    }
}

fn set(db: Arc<RwLock<Db>>, command: &Command) -> Command {
    if command.argc < 3 {
        return Command::error("param ".to_string());
    }
    let mut d = db.write().expect("");
    d.set(command.getArgc(2), Value::String(command.getArgc(3).clone()));
    command!("Ok".to_string())
}
fn main() {
    let config: Config = Config::new("127.0.0.1".to_string(), 6379);
    let mut server = Server::new(config);
//    let v = route![get, set];
    server.route("get".to_string(), Box::new(get));
    server.route("get".to_string(), Box::new(set));
    server.start();
}
