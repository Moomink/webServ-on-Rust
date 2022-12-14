use std::{env, io::Read, net::TcpStream, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("example: ./client ip_address port");
        process::exit(1);
    }

    let port = args.remove(2);
    let ip_address = args.remove(1);

    let mut buffer: [u8; 30] = [0; 30];

    println!("connect IP:port [{}:{}]", ip_address, port);

    match TcpStream::connect(ip_address + ":" + &port) {
        Ok(mut stream) => {
            println!("Connected !");
            stream.read(&mut buffer).expect("read error");
            println!("Buffer: {}", String::from_utf8(buffer.to_vec()).unwrap())
        }
        Err(e) => println!("couldn't connect: {:?}", e),
    }
}
