use std::{
    env,
    io::Write,
    net::{TcpListener, TcpStream},
    process,
};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("example: ./server ip_address port");
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
                stream_handler(stream);
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}

fn stream_handler(mut stream: TcpStream) {
    let response: &str = "
HTTP/1.0 200 OK
Content-Type:text/html;charset=utf-8;

<html>
<head>
<title>My Server</title>
</head>
<body>
HOGE test
</body>
</html>
";

    match stream.write(response.as_bytes()) {
        Ok(buf_n) => {
            println!("send [{}].", buf_n);
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("shutdown error");
        }
        Err(e) => println!("couldn't send to client: {e:?}"),
    }
}
