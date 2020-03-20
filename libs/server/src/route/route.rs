
pub struct Route {
    pub handler: Box<CommandFunc>,
}
impl Route {
    pub fn new(handler: Box<CommandFunc>) -> Route {
        Router {
            handler
        }
    }
    pub fn run(&self, db: Arc<RwLock<Db>>, cmd: &Command) -> Command {
        self.handler(db, cmd)
    }
}