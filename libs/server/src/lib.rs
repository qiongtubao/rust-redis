use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc::Sender;
use std::io;
use std::collections::{HashMap, BTreeMap};
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::iter::Map;
use proc_macro::TokenStream;
use crate::route::{Router, Handler};

mod route;
//#[macro_use] extern crate quote;
extern crate proc_macro;
pub struct Config {
    port: u64,
    host: String,
}
impl Config {
    pub fn new(host: String, port: u64) -> Config {
        Config {
            host,
            port,
        }
    }
}

#[derive(Clone)]
pub struct Command {
    argv: Vec<String>,
    pub argc: usize,
}
#[macro_export]
macro_rules! command {
    // $x 是变量
    // :expr 是关键字语法, 表示表达式
    // * 表示零次或多次表达式匹配
    ($($x:expr), *) => {
        {
            let mut command = Command::new();
            $(
                command.arg($x);
            )*                          // 多次匹配会多次运行这个代码块.
            command
        }
    }
}
impl Command {
    pub fn new() -> Command {
        Command {
            argv: Vec::new(),
            argc: 0
        }
    }
    pub fn error(err: String) -> Command {
       command!(err)
    }
    pub fn ok() -> Command {
        command!("Ok".to_string())
    }
    pub fn arg(&mut self, v: String) {
        self.argv.push(v);
    }
    pub fn setArgc(&mut self, argc:usize) {
        self.argc = argc;
    }
    pub fn isOk(&self) -> bool {
        self.argc == self.argv.len()
    }
    pub fn getArgc(&self, index: usize) -> &String {
        self.argv.get(index).expect("get cmd")
    }
    pub fn write(&self, write: &mut BufWriter<&TcpStream>) -> std::io::Result<()>{
        write.write_all(b"*")?;
        itoa::write(&mut *write, self.argv.len())?;
        write.write_all(b"\r\n")?;
        for item in &self.argv {
            let bytes = item.as_bytes();
            write.write_all(b"$")?;
            itoa::write(&mut *write, bytes.len())?;
            write.write_all(b"\r\n")?;
            write.write_all(bytes)?;
            write.write_all(b"\r\n")?;
        }
        write.flush()?;
        Ok(())
    }
}

pub enum Value {
    String(String)
}
pub struct Db {
    dict: BTreeMap<String, Value>,
}
impl  Db {
    fn new() -> Db {
        Db {
            dict: BTreeMap::new()
        }
    }
    pub fn set(&mut self, key: &String, value: Value) {
        self.dict.insert(key.clone(), value);
    }
    pub fn get(&self, key: &String) -> Option<&Value> {
        self.dict.get(key)
    }
}




pub struct Server  {
    port: u64,
    host: String,
    channels: Option<Sender<u8>>,
    thread: Vec<thread::JoinHandle<()>>,
    hands: Arc<RwLock<Router>>,
    db: Arc<RwLock<Db>>,
}

impl Server  {
    pub fn new(config: Config) -> Server {
        Server {
            port: config.port,
            host: config.host,
            channels: None,
            thread: Vec::new(),
            hands: Arc::new(RwLock::new(Router::new())),
            db: Arc::new(RwLock::new(Db::new()))
        }
    }
    pub fn route(&mut self, cmd: String, handle: Box<Handler>) {
        let mut r = self.hands.write().expect("route");
        r.add(cmd, handle);
    }
    pub fn start(&mut self)  {
        println!("{}:{:?}", self.host, self.port);
        let l = TcpListener::bind(format!("{}:{:?}", self.host, self.port)).expect("abc");
        for stream in l.incoming() {
            let hands = self.hands.clone();
            let db = self.db.clone();
            self.thread.push(thread::spawn(move || {
                let stream = stream.unwrap();
                let reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                let mut command: Command = Command::new();
                let mut next = 0;
                for line in reader.lines() {
                    let line = line.unwrap();
                    match &line[0..1] {
                        "*" => {
                            command.setArgc(line[1..].parse::<usize>().expect("parse * int"));
                        },
                        "$" => {
                            next = line[1..].parse::<usize>().expect("parse $ int");
                        },
                        _ => {
                            if line.len() == next {
                                command.arg(line);
                            }
                        }
                    }
                    if command.isOk() {
                        let mut result = hands.read().expect("exec").get(command.getArgc(0)).expect("abc").run(db.clone(), &command);
                        result.write((&mut writer));
                    }

                }
            }));
        }

    }
}