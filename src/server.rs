use std::{fmt, io};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


struct Clients{
    client_socket: TcpStream,
}
pub struct Server{
    addr: String,
    port: u32,
    intro_page: Websites,
    venoxis_page: Websites,
    lakeshire_page: Websites,
    clients: Vec<Box<Clients>>,
}
impl Server {
    pub fn new(addr: String, port: u32) -> Server {
        Self {
            addr,
            port,
            intro_page: Websites::Intro("/app/intro.html".to_string()),
            venoxis_page: Websites::Venoxis("/app/intro.html".to_string()),
            lakeshire_page: Websites::Lakeshire("/app/intro.html".to_string()),
            clients: vec![],
        }
}
pub fn run(&mut self) {
    println!("Listening on {}", self.addr);
    let listener = TcpListener::bind(&self.addr);
    match listener{
        Ok(connection) => {
            loop{
                match connection.accept(){
                    Ok((mut stream,addr)) => {
                        let mut buffer = [0;1024];
                        match stream.read(&mut buffer){
                            Ok(stream_read) => {
                                println!("stream_write: {:?}",stream_read);

                                // stream_read
                            }
                            Err(err) => {
                                eprintln!("Could not write data to stream {}",err);
                                continue;
                            }
                        }
                        //debug print
                        // println!("{:?}{}",stream,addr);
                    }
                    Err(err) => {
                        continue;
                    }
                };
            }
        }
        Err(_) => { }
    }
}
    // {
    //     Ok(connection) => {connection}
    //     Err(_) => {eprintln!(_)}
    // };

    //     Ok((listen_sock)) => {
    //         println!("connection established");
    //     }
    //     _ => return (),
    // };
    // loop {
    //     for connection in listener.incoming(){
    //         let client_connection = connection.unwrap();
    //     }
    // }

}

// impl Display for Server {
//     fn fmt(&self, f:&mut fmt::formatter) -> std::io::Result<()> {
//         write!(f,"{}:{}",self.addr,self.port)
//     }
// }

pub enum Websites{
    Intro(String),
    Venoxis(String),
    Lakeshire(String),
}