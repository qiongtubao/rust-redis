use std::collections::HashMap;
use crate::object::Object;

pub struct Db {
    data: HashMap<String, Object>
}
impl Db {
    pub fn new() -> Db {
        Db {
            data: HashMap::new()
        }
    }
    pub fn set(&mut self, key: String, value: Object) {
        self.data.insert(key, value);
    }
    pub fn get(&self, key: &String)-> Option<&Object> {
        self.data.get(key)
    }
    pub fn get_string(&self, key: &String) -> Option<&String> {
        if let Some(x) =  self.data.get(key) {
            match  x  {
                Object::String(d) => {
                    return Some(d)
                }
            }
        }
        None
    }
    pub fn del(&mut self, key: &String) -> u64{
        if let Some(x) =  self.data.remove(key) {
            return 1;
        }
        0
    }
}