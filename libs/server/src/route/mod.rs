use route::Route;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::{Db, Command};

//#[proc_macro_attribute]
//pub fn cmd(args: TokenStream, input: TokenStream) -> TokenStream {
//    emit!(attribute::route::route_attribute($method, args, input))
//}
type CommandFunc = Fn(Arc<RwLock<Db>>, &Command) -> Command;

#[derive(Default)]
pub struct Router {
    routes: HashMap<String, Route>,
}
impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: String, value: Box<CommandFunc>) {
        self.routes.insert(key.clone(), Route::new(value));
    }
    pub fn get(&self, key: &String) -> Option<&Router>{
        self.routes.get(key)
    }
}