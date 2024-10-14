mod server;
mod copy;

fn main() -> std::io::Result<()> {
    let res = server::bind_server();
    match res {
        Ok(x) =>  Ok(x),
        Err(x) => Err(x),
    }
} 
