use std::fs::read_to_string;
use std::io::{Result, Write};
use std::net::TcpStream;

pub fn copy_file_into_buffer(mut stream: TcpStream, dir_path: String) -> Result<()> {
    let contents = read_to_string(dir_path)?;
    let _ = stream.write(contents.as_bytes());
    Ok(())
}
