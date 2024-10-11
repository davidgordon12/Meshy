mod server;

fn main() -> std::io::Result<()> {
    let res = server::bind_server();
    Ok(())
} 
