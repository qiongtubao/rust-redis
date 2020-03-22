use mio_server::db::Db;
use mio_server::command::Command;
use mio_server::Server;
use mio_server::object::Object;
use mio_server::object::list::List;

//#[cmd="get"]
fn lindex(db: &mut Db,c: Command) -> Command {
    if let Some(data) = db.get_list(c.getArgc(1)) {
        match data.get(c.getInt(2)) {
            Some(x) => {
                return command!(x.clone());
            }
            None => {}
        }

    }
    Command::nil()
}
//#[cmd="set"]
fn rpush(db: &mut Db, c: Command) -> Command {
    if db.get_mut(c.getArgc(1)).is_none() {
        db.set(c.getArgc(1).to_string(), Object::List(List::new()));
    }
    if let Some(data) = db.get_mut(c.getArgc(1)) {
        if let Object::List(list) =  data {
            let mut i = 2;
            loop {
                if(i >= c.argc) {
                    break;
                }
                list.push(c.getArgc(i).to_string());
                i += 1;
            }
            return command!(format!("{}", 1));
        } else {
            return Command::error("key is exist".to_string());
        }
    }
    Command::error("why".to_string())
}

fn rpop(db: &mut Db, c: Command) -> Command {
    command!(format!("{}", db.del(c.getArgc(1))))
}


pub fn route(s: &mut Server) {
    s.route("lindex", &lindex);
    s.route("rpush", &rpush);
    s.route("rpop", &rpop);
}

