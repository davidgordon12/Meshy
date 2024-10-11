use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1337")?;
    let mut res = Vec::<u8>::new();
    let x = String::from("hey");
    let _ = stream.write(x.as_bytes());
    let _ = stream.read(&mut res);
    Ok(())
} // the stream is closed here
