use std::collections::HashMap;
use crate::object::list::List;

pub mod list;
pub enum Object {
    String(String),
    Hash(HashMap<String,String>),
    List(List<String>),
}