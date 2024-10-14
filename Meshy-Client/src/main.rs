use std::{env, io::{prelude::*, Error}, net::{TcpListener, TcpStream}};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => return Err(Error::other("Please provide a folder path.")),
        2 => println!("Attempting to copy files from {}", args[1]),
        _ => return Err(Error::other("Too many arguments. Usage: Meshy path/to/dir")),
    }

    {
        let mut stream = TcpStream::connect("127.0.0.1:4996")?;
        let path = String::from("/home/akali/Developer/Meshy/temp");
        let _ = stream.write(path.as_bytes());
    }
    
    let listener = TcpListener::bind("127.0.0.1:6994")?;
    for stream in listener.incoming() {
        // each stream will come in file_path - file_content order from the client
    }
    Ok(())
}
