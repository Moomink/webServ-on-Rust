use std::{
    collections::HashMap,
    env, fs,
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    process,
    str::{self, FromStr},
};

use infer;

#[derive(Debug)]
enum RequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl RequestMethod {
    fn from_str(method: &str) -> Result<RequestMethod, &str> {
        match method {
            "GET" => Ok(RequestMethod::GET),
            "HEAD" => Ok(RequestMethod::HEAD),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            "DELETE" => Ok(RequestMethod::DELETE),
            "CONNECT" => Ok(RequestMethod::CONNECT),
            "OPTIONS" => Ok(RequestMethod::OPTIONS),
            "TRACE" => Ok(RequestMethod::TRACE),
            &_ => Err("Selected method is not found in enum."),
        }
    }

    fn to_string(&self) -> &str {
        match self {
            RequestMethod::GET => "GET",
            RequestMethod::HEAD => "HEAD",
            RequestMethod::POST => "POST",
            RequestMethod::PUT => "PUT",
            RequestMethod::DELETE => "DELETE",
            RequestMethod::CONNECT => "CONNECT",
            RequestMethod::OPTIONS => "OPTIONS",
            RequestMethod::TRACE => "TRACE",
            _ => "",
        }
    }
}

impl Default for RequestMethod {
    fn default() -> Self {
        RequestMethod::GET
    }
}

//TODO HttpResponseクラスも作る
#[derive(Default, Debug)]
struct HttpRequest {
    method: RequestMethod,
    version: f32,
    uri: String,
    header: HttpHeader,
}

type HttpHeader = HashMap<String, String>;

impl HttpRequest {
    fn new() -> Self {
        return HttpRequest::default();
    }

    fn header(&mut self, header: HttpHeader) {
        self.header = header;
    }

    fn uri(&mut self, uri: &str) {
        self.uri = uri.to_string();
    }

    fn version(&mut self, version: f32) {
        self.version = version
    }

    fn method(&mut self, method: &str) {
        self.method = RequestMethod::from_str(method).unwrap()
    }

    fn from_buffer(buf: &[u8]) -> HttpRequest {
        let mut req: Vec<&str> = str::from_utf8(&buf).unwrap().split("\r\n").collect();

        // 空白の行を削除
        req.remove(req.len() - 1);
        req.remove(req.len() - 1);

        let first_line: Vec<&str> = req.swap_remove(0).split(" ").collect();

        //TODO catch error
        let request_version =
            f32::from_str(first_line[2].split("/").collect::<Vec<&str>>()[1]).unwrap();

        let mut headers = HttpHeader::new();

        for raw_header in req.iter() {
            let parsed_header: Vec<&str> = raw_header.splitn(2, ": ").collect();
            headers.insert(parsed_header[0].to_string(), parsed_header[1].to_string());
        }

        let mut request_struct = HttpRequest::new();

        request_struct.method(first_line[0]);
        request_struct.uri(first_line[1]);
        request_struct.version(request_version);
        request_struct.header(headers);

        return request_struct;
    }
}

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

fn stream_handler(mut stream: &TcpStream) {
    let mut buffer: [u8; 1000] = [0u8; 1000];

    let mut response: String = String::new();

    stream.read(&mut buffer).unwrap();

    let request = HttpRequest::from_buffer(&buffer);
    println!("{:?}", request);

    let res_header: &str = "
HTTP/1.0 200 OK
Content-Type:text/html;charset=utf-8;

";
    let req: Vec<&str> = str::from_utf8(&buffer).unwrap().split("\r\n").collect();

    // uri split
    let mut uri: Vec<&str> = req[0].split(" ").collect::<Vec<&str>>()[1]
        .split("/")
        .collect();

    uri.remove(0);

    //TODO ファイルタイプの特定
    //let ftype = infer::get_from_path(String::from("www/") + uri[0]).expect("file type expected");

    let binding = match fs::read_to_string(String::from("www/") + uri[0]) {
        Ok(data) => data,
        Err(_) => "<html>
<head>
<title>404 Not Found</title>
</head>
<body>
No file
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

    println!("Respose data [{}]", response);
    match stream.write(response.as_bytes()) {
        Ok(_) => {
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("shutdown error");
        }
        Err(e) => println!("couldn't send to client: {e:?}"),
    }
}
