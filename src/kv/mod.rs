use mio_server::db::Db;
use mio_server::command::Command;
use mio_server::Server;
use mio_server::object::Object;

//#[cmd="get"]
fn get(db: &mut Db,c: Command) -> Command {
    if let Some(data) = db.get_string(c.getArgc(1)) {
        return command!(data.to_string());
    }
    Command::nil()
}
//#[cmd="set"]
fn set(db: &mut Db, c: Command) -> Command {
    db.set(c.getArgc(1).to_string(), Object::String(c.getArgc(2).to_string()));
    command!(format!("{}", 1))
}

fn del(db: &mut Db, c: Command) -> Command {
    command!(format!("{}", db.del(c.getArgc(1))))
}
fn command(db: &mut Db, c: Command) -> Command {
    Command::ok()
}

pub fn route(s: &mut Server) {
    s.route("get", &get);
    s.route("set", &set);
    s.route("del", &del);
    s.route("command", &command);
}


