use std::collections::HashMap;
use crate::object::Object;
use crate::object::list::List;
use chrono::Local;
pub struct Db {
    data: HashMap<String, Object>,
    expires: HashMap<String, i64>,
//    blocking_keys: Hash<String,String>
//    ready_keys: HashMap<String, String>
//    watched_keys: HashMap<String, String>
}
impl Db {
    pub fn new() -> Db {
        Db {
            data: HashMap::new(),
            expires: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: Object) -> Option<Object>{
        self.data.insert(key, value)
    }
    pub fn get(&self, key: &String)-> Option<&Object> {
        if self.expire_if_needed(key) {
            return None;
        }
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut Object> {
        if self.expire_if_needed(key) {
            self.del_data(key);
            return None;
        }
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
    pub fn expire_if_needed(&self, key: &String) -> bool{
        if let Some(x) = self.expires.get(key) {
            let now_time = Local::now().timestamp();
            if now_time > *x {
                return true;
            }
        }
        false
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


    pub fn activeExpireCycle(&mut self) {
        let now_time = Local::now().timestamp();
        let mut keys = vec!();
        for (key, value) in &self.expires {
            if now_time > *value {
                //del command
                keys.push(key.to_string());
            }
        }
        for key in &keys {
            self.del_data(key);
        }
    }
    pub fn set_expire(&mut self, key: String, expire: i64) {
        self.expires.insert(key, expire);
    }
    pub fn get_expire(&self, key: &String) -> Option<&i64>{
        self.expires.get(key)
    }
    pub fn del_data(&mut self, key: &String) {
        self.expires.remove(key);
        self.data.remove(key);
    }
}