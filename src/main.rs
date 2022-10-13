use std::io;

mod server;
mod method;
mod request;

fn main() -> io::Result<()>{

    let mut server = server::Server::new("127.0.0.1:8080".to_string(), 8080);
    server.run();

    Ok(())
}
