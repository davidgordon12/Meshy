use std::{io::Read, net::{TcpListener, TcpStream}};

fn handle_request(mut stream: TcpStream) {
    let mut buff = [0; 8];
    let _ = stream.read(&mut buff);
    println!("{:#?}", &buff);
}

pub fn bind_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_request(stream?);
    }

    Ok(())
}
