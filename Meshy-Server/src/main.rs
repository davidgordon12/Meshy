mod server;
mod copy;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let res = server::bind_server();
    match res {
        Ok(x) =>  Ok(x),
        Err(x) => Err(x),
    }
} 
