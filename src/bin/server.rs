use std::{
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process, str,
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
                println!("from {} receice.", stream.peer_addr().unwrap());
                stream_handler(&stream);
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}

fn stream_handler(mut stream: &TcpStream) {
    let mut buffer: [u8; 1000] = [0u8; 1000];

    let mut response: String = String::new();

    let res_header: &str = "
HTTP/1.1 200 OK
Content-Type:text/html;charset=utf-8;

";

    stream.read(&mut buffer).unwrap();

    let req: Vec<&str> = str::from_utf8(&buffer).unwrap().split("\r\n").collect();

    println!("Request header [{}]", &req[0]);

    // uri split
    let mut uri: Vec<&str> = req[0].split(" ").collect::<Vec<&str>>()[1]
        .split("/")
        .collect();

    uri.remove(0);

    println!("URI: {:?}", uri);

    let binding = match fs::read_to_string(String::from("www/") + uri[0]) {
        Ok(data) => data,
        Err(_) => "<html>
<head>
<title>My Server</title>
</head>
<body>
FRONT test
</body>
</html>
"
        .to_string(),
    };

    let body: &str = match uri.is_empty() {
        true => {
            "<html>
<head>
<title>My Server</title>
</head>
<body>
FRONT test
</body>
</html>
"
        }
        false => binding.as_str(),
    };
    response.push_str(res_header);
    response.push_str(body);

    println!("Respose data{}", response);
    match stream.write(response.as_bytes()) {
        Ok(_) => {
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("shutdown error");
        }
        Err(e) => println!("couldn't send to client: {e:?}"),
    }
}
