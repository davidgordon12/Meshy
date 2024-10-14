use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};

use crate::copy::*;

const INPUT_BUFFER_MAX_SIZE: usize = 1024;

pub fn bind_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4996")?;

    let stream = listener.accept();
    match stream {
        Ok((mut stream, _)) => Ok(handle_request(&mut stream)?),
        Err(x) => Err(x),
    }
}

fn handle_request(stream: &mut TcpStream) -> Result<()> {
    let mut buff = [0; INPUT_BUFFER_MAX_SIZE];

    let mut result = String::from("");

    {
        let sz: usize = stream.read(&mut buff)?;
        let res = &buff[..sz];
        result = String::from_utf8(res.to_vec()).unwrap_or(String::from(""));
    }

    match copy_file_into_buffer(result) {
        Ok(x) => Ok(x),
        Err(x) => Err(x),
    }
}
