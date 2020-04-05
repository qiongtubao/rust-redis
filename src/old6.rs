//mio ={version = "0.6.21"}
use mio::{Events, Poll, Token,Ready};
fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let channel_sender = event_loop.channel();
    thread::spawn(move || {
        while(true) {
            channel_sender.send(IoMessage::Notify);
            thread::sleep_ms(5*1000);
        }
        channel_sender.send(IoMessage::End);
    });
    let timeout = event_loop.timeout(Token(123), Duration::from_millis(3000)).unwrap();
    let mut handler = MioHandler::new();
    let _ = event_loop.run(&mut handler).unwrap();
}
pub enum IoMessage {
    Notify,
    End,
}
pub struct MioHandler {

}
impl MioHandler {
    pub fn new() -> Self {
        MioHandler{}
    }
}
impl Handler for MioHandler {
    type Timeout = Token;
    type Message = IoMessage;
    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: Ready) {

    }
    fn notify(&mut self, event_loop: &mut EventLoop<Self>, msg: Self::Message) {
        match msg {
            IoMessage::Notify => {
                println!("channel notify");
            }
            IoMessage::End => {
                event_loop.shutdown();
            }
        }
    }
    fn timeout(&mut self, event_loop: &mut EventLoop<Self>, timeout: Self::Timeout) {
        match timeout {
            Token(123) => println!("time out."),
            Token(_) => {},
        }
    }
    fn interrupted(&mut self, event_loop: &mut EventLoop<Self>) {

    }
    fn tick(&mut self, event_loop: &mut EventLoop<Self>) {
    }


}