use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream_handler(stream);
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}

fn stream_handler(mut stream: TcpStream) {
    match stream.write("I'm conneced!".as_bytes()) {
        Ok(buf_n) => {
            println!("send [{}].", buf_n);
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("shutdown error");
        }
        Err(e) => println!("couldn't send to client: {e:?}"),
    }
}
