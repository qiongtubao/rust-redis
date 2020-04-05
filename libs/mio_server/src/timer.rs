use mio::{Waker, Token, Registry, event, Interest};
use std::sync::Arc;
use std::time::Duration;
use std::{thread, io};
use std::error::Error;
use crossbeam::channel::{bounded, Sender,RecvTimeoutError};


pub struct Interval {
    sender: Option<Sender<String>>,
    time: u64
}
pub const INTERVAL_TOKEN: Token = Token(0);
impl Interval {
    pub fn new(time: u64) -> Self {
        Interval {
            time,
            sender: None,
        }
    }
}

impl event::Source for Interval {
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        let (sender, receiver) = bounded::<String>(5);
        let waker = Waker::new(registry, token)?;
        self.sender = Some(sender);
        let t = Duration::from_millis(self.time);
        let handle = thread::spawn(move || {
            // Working hard, or hardly working?
            loop {
                match receiver.recv_timeout(t) {
                    Ok(_) => {

                    },
                    Err(RecvTimeoutError::Timeout) => {
                        // Now we'll wake the queue on the other thread.
                        waker.wake().expect("unable to wake");
                    },
                    Err(RecvTimeoutError::Disconnected) => {
                        return;
                    }
                }

            }
        });
        Ok(())
    }

    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()>{
        self.register(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        self.sender = None;
        Ok(())
    }
}