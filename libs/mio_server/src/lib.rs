use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use mio::{Events, Interest, Poll, Registry, Token};
use mio::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::io::Read;
use mio::event::Event;
use std::io;
use crate::handles::Handle;
use std::error::Error;
use crate::command::Command;


pub mod handles;
pub mod command;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);
pub struct Server<'a> {
    port: u64,
    host: String,
    token: Token,
    connections: HashMap<Token, TcpStream>,
    handles: handles::Handles<'a>,
    server: Option<TcpListener>,
    poll: Option<Poll>,
}

impl<'a> Server<'a> {
    pub fn new(host: &str, port: u64) -> Server<'a> {
        Server {
            host: host.to_string(),
            port,
            token: Token(SERVER.0 + 1),
            connections: HashMap::new(),
            handles: handles::Handles::new(),
            server: None,
            poll: None
        }
    }
    pub fn route(&mut self, key: &str, handle: &'a Handle) {
        self.handles.insert(key.to_string(), handle)
    }

    fn readCommand(
        registry: &Registry,
        connection: &mut TcpStream,
        event: &Event,
    ) -> Option<command::Command> {
        println!("????? {} {}", event.is_writable(), event.is_readable());
        if event.is_readable() {
            println!("why can't read");
            let mut connection_closed = false;
            let mut received_data = Vec::with_capacity(4096);
            // We can (maybe) read from the connection.
            loop {
                let mut buf = [0; 256];
                match connection.read(&mut buf) {
                    Ok(0) => {
                        connection_closed = true;
                        break;
                    }
                    Ok(n) => received_data.extend_from_slice(&buf[..n]),
                    // Would block "errors" are the OS's way of saying that the
                    // connection is not actually ready to perform this I/O operation.
                    Err(ref err) if would_block(err) => break,
                    Err(ref err) if interrupted(err) => continue,
                    // Other errors we'll consider fatal.
                    Err(err) => return None,
                }
            }
            let mut command = command::Command::new();
            let mut next = 0;
            if let Ok(str_buf) = from_utf8(&received_data) {
                for line in str_buf.split_whitespace(){
                    println!("{}",line);
                    let data = line.to_string();
                    match &line[0..1] {
                        "*" => {
                            command.setArgc(data[1..].parse::<usize>().expect("parse * int"));
                        },
                        "$" => {
                            next = data[1..].parse::<usize>().expect("parse $ int");
                        },
                        _ => {
                            if line.len() == next {
                                command.arg(data);
                            }
                        }
                    }
                    if command.isOk() {
                        // let mut result = hands.read().expect("exec").get(command.getArgc(0)).expect("abc").run(db.clone(), &command);
                        return Some(command);
                    }
                }

                // println!("Received data: {}", str_buf.trim_end());
            } else {
                println!("Received (none UTF-8) data: {:?}", &received_data);
            }

            if connection_closed {
                println!("Connection closed");
                return None;
            }
        }
        None
    }
    pub fn init(&mut self) -> io::Result<()>{

        Ok(())
    }
    fn handle_connection_event(
        registry: &Registry,
        connection: &mut TcpStream,
        event: &Event,
    ) -> io::Result<bool> {
        if event.is_writable() {
            // We can (maybe) write to the connection.
            // match connection.write(DATA) {
            //     // We want to write the entire `DATA` buffer in a single go. If we
            //     // write less we'll return a short write error (same as
            //     // `io::Write::write_all` does).
            //     Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
            //     Ok(_) => {
            //         // After we've written something we'll reregister the connection
            //         // to only respond to readable events.
            //         registry.reregister(connection, event.token(), Interest::READABLE)?
            //     }
            //     // Would block "errors" are the OS's way of saying that the
            //     // connection is not actually ready to perform this I/O operation.
            //     Err(ref err) if would_block(err) => {}
            //     // Got interrupted (how rude!), we'll try again.
            //     Err(ref err) if interrupted(err) => {
            //         return handle_connection_event(registry, connection, event)
            //     }
            //     // Other errors we'll consider fatal.
            //     Err(err) => return Err(err),
            // }
        }

        if event.is_readable() {
            let mut connection_closed = false;
            let mut received_data = Vec::with_capacity(4096);
            // We can (maybe) read from the connection.
            loop {
                let mut buf = [0; 256];
                match connection.read(&mut buf) {
                    Ok(0) => {
                        // Reading 0 bytes means the other side has closed the
                        // connection or is done writing, then so are we.
                        connection_closed = true;
                        break;
                    }
                    Ok(n) => received_data.extend_from_slice(&buf[..n]),
                    // Would block "errors" are the OS's way of saying that the
                    // connection is not actually ready to perform this I/O operation.
                    Err(ref err) if would_block(err) => break,
                    Err(ref err) if interrupted(err) => continue,
                    // Other errors we'll consider fatal.
                    Err(err) => return Err(err),
                }
            }
            let mut command = Command::new();
            let mut next = 0;
            if let Ok(str_buf) = from_utf8(&received_data) {
                for line in str_buf.split_whitespace(){
                    println!("line : {}", line);
                    let data = line.to_string();
                    match &line[0..1] {
                        "*" => {
                            command.setArgc(data[1..].parse::<usize>().expect("parse * int"));
                        },
                        "$" => {
                            next = data[1..].parse::<usize>().expect("parse $ int");
                        },
                        _ => {
                            if line.len() == next {
                                command.arg(data);
                            }
                        }
                    }
                    if command.isOk() {
                        // let mut result = hands.read().expect("exec").get(command.getArgc(0)).expect("abc").run(db.clone(), &command);
                        let mut result = Command::ok();
                        result.write((connection));
                    }
                }

                // println!("Received data: {}", str_buf.trim_end());
            } else {
                println!("Received (none UTF-8) data: {:?}", &received_data);
            }

            if connection_closed {
                println!("Connection closed");
                return Ok(true);
            }
        }

        Ok(false)
    }
    pub fn run(&mut self) -> Result<(), Box<dyn Error>>  {
        // Create a poll instance.
        let mut poll = Poll::new()?;
        // Create storage for events.
        let mut events = Events::with_capacity(128);

        // Setup the server socket.
        let addr = "127.0.0.1:6379".parse()?;
        let mut server = TcpListener::bind(addr)?;
        // Start listening for incoming connections.
        poll.registry()
            .register(&mut server, SERVER, Interest::READABLE)?;

        // Setup the client socket.
        let mut client = TcpStream::connect(addr)?;
        // Register the socket.
        poll.registry()
            .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;
        let mut connections = HashMap::new();
        // Unique token for each incoming connection.
        let mut unique_token = Token(SERVER.0 + 1);
        // Start an event loop.
        loop {
            // Poll Mio for events, blocking until we get an event.
            poll.poll(&mut events, None)?;
            // Process each event.
            for event in events.iter() {
                // We can use the token we previously provided to `register` to
                // determine for which socket the event is.
                match event.token() {
                    SERVER => {
                        // If this is an event for the server, it means a connection
                        // is ready to be accepted.
                        //
                        // Accept the connection and drop it immediately. This will
                        // close the socket and notify the client of the EOF.
                        let (mut connection, address) = server.accept()?;
                        println!("Accepted connection from: {}", address);
                        let token = next(&mut unique_token);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;

                        connections.insert(token, connection);
                    }
                    token => {
                        let done = if let Some(connection) = connections.get_mut(&token) {
                            Server::handle_connection_event(poll.registry(), connection, event)?
                        } else {
                            // Sporadic events happen.
                            false
                        };
                        if done {
                            connections.remove(&token);
                        }
                        // Since the server just shuts down the connection, let's
                        // just exit from our event loop.
                    }
                    // We don't expect any events with tokens other than those we provided.
                    _ => {
                        println!("hh");
                    }
                }
            }
        }
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}
