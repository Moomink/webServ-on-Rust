use std::{
    env,
    net::{TcpListener, TcpStream},
    process,
};

use tcp_web::{RequestMethod, PayloadType, HttpResponse, HttpRequest, HttpHeader, stream_handler};


fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("example: {} <ip_address> <port>", args[0]);
        process::exit(1);
    }

    let port = args.remove(2);
    let ip_address = args.remove(1);

    let bind_ip = ip_address + ":" + &port;

    let listener = TcpListener::bind(&bind_ip).unwrap();

    println!("connected IP:port [{}]", bind_ip);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("from {} receice.", stream.peer_addr().unwrap());
                stream_handler(&stream);
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}
