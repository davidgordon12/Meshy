use std::fs::{read_to_string, read_dir};
use std::io::{Result, Write};
use std::net::TcpStream;
use std::path::Path;

pub fn copy_file_into_buffer(stream: &mut TcpStream, dir_path: String) -> Result<()> {
    let path = Path::new(&dir_path[..]);
    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                copy_file_into_buffer(stream, entry.path().into_os_string().into_string().unwrap())?;
            } else {
                let contents = read_to_string(entry.path())?;
                let _ = stream.write(contents.as_bytes());
            }
        }
    }
    Ok(())
}
