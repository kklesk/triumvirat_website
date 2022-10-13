use std::{fmt, io};
use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::request::Request;

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
            match listener {
                Ok(connection) => {
                    loop {
                        match connection.accept() {
                            Ok((mut stream, addr)) => {
                                let mut buffer = [0; 1024];
                                // stream.read(&mut buffer).expect("Could not read buffer");
                                match stream.read(&mut buffer){
                                    Ok(_) => {
                                        println!("{}",String::from_utf8_lossy(&buffer));
                                        println!("Recieved a request: {:?} \nfrom IP: {}",stream,addr);
                                        // Request::try_from(&buffer as &[u8]);
                                        /* BEGINTest tryfrom function*/
                                        // match Request::try_from(&buffer[..]){
                                        //     Ok(request) => {println!("{}",request)},
                                        //     Err(err) => {eprintln!("{}",err)},
                                        // }
                                        // let test_req = Request::try_from(&buffer).unwrap();
                                        // println!("Recieved a request: {} ",test_req);
                                        // println!("Recieved a request: {} ",Request::try_from(&buffer.unwrap()));
                                        /* Test tryfrom function END */
                                    }
                                    Err(err) => eprintln!("Failed to read the request {}",err),
                                }
                            }
                            Err(err) => {
                                eprintln!("unknown request {}", err);
                                continue;
                            }
                        };
                    }
                }
                Err(_) => {}
            }
        }
    }
//     pub fn handler(buffer: &[u8;1024]){
//         // println!("{}",String::buffer)
//         // println!("{}",String::from_utf8_lossy(buffer));
//         println!("{:?}", buffer.into_bytes());
//         // match bb.read_bytes(){
//         //     Ok(header) => {
//         //         println!("{:?}", header);
//         //     }
//         //     Err(err) => {
//         //         eprintln!("error while reading req header {}", err);
//         //     }
//         // };
//         }
// }
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
//FIXME
// impl Display for Server {
//     fn fmt(&self, f:&mut fmt::Formatter) -> std::io::Result<()> {
//         write!(f,"{}:{}",self.addr,self.port)
//     }
// }

pub enum Websites{
    Intro(String),
    Venoxis(String),
    Lakeshire(String),
}