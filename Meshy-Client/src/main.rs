use std::{env, io::{prelude::*, Error}, net::{TcpListener, TcpStream}};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => return Err(Error::other("Please provide a folder path.")),
        2 => println!("Attempting to copy files from {}", args[1]),
        _ => return Err(Error::other("Too many arguments. Usage: Meshy <path>")),
    }

    {
        let mut stream = TcpStream::connect("127.0.0.1:4996")?;
        let path = String::from("/home/akali/Developer/Meshy/Meshy-Client/test");
        let _ = stream.write(path.as_bytes());
    }
    
    let listener = TcpListener::bind("127.0.0.1:6994")?;

    for stream in listener.incoming() {
        let mut buf = [0;1024];
        let sz = stream?.read(&mut buf)?;

        if sz == 0 {
            break 
        }

        let res = String::from_utf8(buf[..sz].to_vec()).unwrap();
        res.split("@#!dg*777;*");
        println!("{}", res);
    }

    Ok(())
}
