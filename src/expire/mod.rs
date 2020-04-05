use mio_server::db::Db;
use mio_server::command::Command;
use mio_server::Server;
use mio_server::object::Object;
use std::collections::HashMap;
use chrono::Local;
//#[cmd="get"]
//expire key time
fn expire(db: &mut Db,c: Command) -> Command {
    if c.argc < 3 {
        return Command::error("params error".to_string());
    }
    if let Ok(x) =  (c.getArgc(2)).parse::<i64>() {
        let time = Local::now().timestamp() + x;
        if let Some(_) = db.get(c.getArgc(1)) {
            db.set_expire(c.getArgc(1).to_string(), time);
            return Command::ok();
        }
        return Command::error("no key".to_string());
    }
    Command::error("params error".to_string())

}


fn ttl(db: &mut Db, c: Command) -> Command {
    let key = c.getArgc(1);
    if let Some(x) = db.get_expire(key) {
        let time = Local::now().timestamp();
        if time > *x {
            db.del_data(key);
        } else {
            return command!((format!("{}", x - time)));

        }
    }
    command!("-1".to_string())
}
pub fn route(s: &mut Server) {
    s.route("expire", &expire);
    s.route("ttl", &ttl);
}


