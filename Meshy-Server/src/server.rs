use std::{io::Read, net::{TcpListener, TcpStream}};

fn handle_request(mut stream: TcpStream) {
    let mut buff = [0; 16];

    let size: usize = stream.read(&mut buff).unwrap_or(0);
    let result = String::from_utf8(buff.to_vec()).unwrap_or("".to_string());

    if size == 0 || result == "" { return }

    println!("{}", result)
    
}

pub fn bind_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_request(stream?);
    }

    Ok(())
}
