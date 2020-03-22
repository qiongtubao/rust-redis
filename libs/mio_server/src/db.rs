use std::collections::HashMap;
use crate::object::Object;
use crate::object::list::List;

pub struct Db {
    data: HashMap<String, Object>
}
impl Db {
    pub fn new() -> Db {
        Db {
            data: HashMap::new()
        }
    }
    pub fn set(&mut self, key: String, value: Object) -> Option<Object>{
        self.data.insert(key, value)
    }
    pub fn get(&self, key: &String)-> Option<&Object> {
        self.data.get(key)
    }
    pub fn get_mut(&mut self, key: &String) -> Option<&mut Object> {
        self.data.get_mut(key)
    }
    pub fn get_string(&self, key: &String) -> Option<&String> {
        if let Some(x) =  self.data.get(key) {
            match  x  {
                Object::String(d) => {
                    return Some(d)
                },
                _ =>{}
            }
        }
        None
    }
    pub fn get_hash(&self, key: &String) -> Option<&HashMap<String, String>> {
        if let Some(x) =  self.data.get(key) {
            match  x  {
                Object::Hash(d) => {
                    return Some(d)
                },
                _ =>{}
            }
        }
        None
    }
    pub fn get_list(&self, key: &String) -> Option<&List<String>> {
        if let Some(x) = self.data.get(key) {
            match x {
                Object::List(d) => {
                    return Some(d)
                },
                _ => {}
            }
        }
        None
    }
    pub fn get_mut_hash(&mut self, key: &String) -> Option<&mut HashMap<String, String>> {
        if let Some(x) = self.data.get_mut(key) {
            match x {
                Object::Hash(d) => {
                    return Some(d)
                },
                _ => {}
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