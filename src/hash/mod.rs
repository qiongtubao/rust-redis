use mio_server::db::Db;
use mio_server::command::Command;
use mio_server::Server;
use mio_server::object::Object;
use std::collections::HashMap;

//#[cmd="get"]
fn hget(db: &mut Db,c: Command) -> Command {
    if let Some(data) = db.get_hash(c.getArgc(1)) {
        if let Some(x) = data.get(c.getArgc(2)) {
            return command!(x.to_string());
        }
    }
    Command::nil()
}
//#[cmd="set"]
fn hset(db: &mut Db, c: Command) -> Command {
    if db.get_mut(c.getArgc(1)).is_none() {
        let mut h: HashMap<String, String> = HashMap::new();
        db.set(c.getArgc(1).to_string(), Object::Hash(h));
    }
    if let Some(data) = db.get_mut(c.getArgc(1)) {
        if let Object::Hash(hash) =  data {
            let mut i = 2;
            loop {
                if(i >= c.argc) {
                    break;
                }
                hash.insert(c.getArgc(i).to_string(), c.getArgc(i + 1).to_string());
                i += 2;
            }
            return command!(format!("{}", 1));
        } else {
            return Command::error("key is exist".to_string());
        }
    }
    Command::error("why".to_string())

}

fn hdel(db: &mut Db, c: Command) -> Command {
    if let Some(data) = db.get_mut_hash(c.getArgc(1)) {
            let result = if data.remove(c.getArgc(2)).is_none() { 0 }else { 1 };
            return command!(format!("{}", result));
    }
    command!(format!("{}", 0))
}



pub fn route(s: &mut Server) {
    s.route("hget", &hget);
    s.route("hset", &hset);
    s.route("hdel", &hdel);
}


