use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1337")?;

    stream.write(&[1, 2, 3, 4, 5, 6])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
