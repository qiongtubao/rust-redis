use std::collections::HashMap;

pub enum Object {
    String(String),
    Hash(HashMap<String,String>)
}