#[macro_use]
extern crate mio_server;
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Poll, Token, Interest, Waker};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;
use std::error::Error;
use mio_server::handles::Handle;
use mio_server::command::Command;
use mio_server::db::Db;
use mio_server::object::Object;
use std::{thread, net};
use std::time::Duration;
use std::net::SocketAddr;
use std::sync::Arc;

mod kv;
mod hash;
mod list;
mod expire;

fn main() {
    let mut s =  mio_server::Server::new("127.0.0.1", 6379);
    s.init();
    kv::route(&mut s);
    hash::route(&mut s);
    list::route(&mut s);
    expire::route(&mut s);
    s.run();
}