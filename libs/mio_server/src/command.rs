use std::io::{BufWriter, Write};
use mio::net::TcpStream;

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
    #[allow(unused_variables)]
    pub fn write<'a>(&self, write:  &'a mut Write) -> std::io::Result<()>{
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