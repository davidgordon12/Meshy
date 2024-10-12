use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4996")?;
    let mut res = [0; 1024];
    let x = String::from("/home/akali/Developer/Meshy/temp/tmp.txt");
    let _ = stream.write(x.as_bytes());
    let _ = stream.read(&mut res);

    println!("Response:\n        {}", 
        String::from_utf8(res.to_vec()).unwrap_or(String::from("Error: Unable to get response from server")));

    Ok(())
}
