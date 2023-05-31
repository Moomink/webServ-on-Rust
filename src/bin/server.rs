use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
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

#[derive(Debug, Clone)]
enum PayloadType {
    Text(String),
    Binary(Vec<u8>),
    None,
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
        }
    }
}

impl Default for RequestMethod {
    fn default() -> Self {
        RequestMethod::GET
    }
}

#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    version: f32, //TODO 桁数調整
    reason: String,
    header: HttpHeader,
    payload: PayloadType,
}

impl HttpResponse {
    fn new() -> Self {
        Self {
            status_code: 0,
            version: 0.0,
            reason: "".to_string(),
            header: HttpHeader::default(),
            payload: PayloadType::None,
        }
    }

    fn status_code(&mut self, code: u16) {
        self.status_code = code;
    }

    fn version(&mut self, version: f32) {
        self.version = version;
    }

    fn reason(&mut self, reason: &str) {
        self.reason = reason.to_string();
    }

    fn header(&mut self, header: HttpHeader) {
        self.header = header;
    }

    fn payload(&mut self, payload: PayloadType) {
        self.payload = PayloadType::None;
        self.payload = payload.clone();
    }

    fn as_bytes(&mut self) -> Vec<u8> {
        let mut messages: String = format!(
            "HTTP/{} {} {}\n",
            self.version, self.status_code, self.reason
        );
        let mut headers: Vec<String> = Vec::new();
        for (k, v) in self.header.iter() {
            headers.push(format!("{k}: {v}"));
        }
        headers.push("\n\n".to_string());

        messages.extend(headers);

        let mut bytes = messages.into_bytes();

        match self.payload.clone() {
            PayloadType::Text(data) => bytes.extend(data.as_bytes()),
            PayloadType::Binary(data) => bytes.extend(data),
            PayloadType::None => (),
        };

        return bytes;
    }
}

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

    stream.read(&mut buffer).unwrap();

    let request = HttpRequest::from_buffer(&buffer);
    println!("{:?}", request);

    let mut response = HttpResponse::new();

    response.status_code(200);
    response.reason("OK");
    response.version(1.1);
    let mut response_header: HttpHeader = HttpHeader::new();

    let file_path = format!("www{}", request.uri);

    match fs::read(file_path.clone()) {
        Ok(data) => {
            let ftype = infer::get_from_path(file_path)
                .expect("file read successfully")
                .expect("file type is known");
            response_header.insert("Content-Type".to_string(), ftype.mime_type().to_string());

            if ftype.matcher_type() == infer::MatcherType::Text {
                let string = String::from_utf8(data).expect("Convert Failed.");
                response.payload(PayloadType::Text(string));
            } else {
                response.payload(PayloadType::Binary(data));
            };
        }
        Err(_) => {
            response.status_code(404);
            response.reason("File not found.");
        }
    };

    response.header(response_header);

    println!("Respose data [{:?}]", response);
    match stream.write(&response.as_bytes()) {
        Ok(_) => {
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("shutdown error");
        }
        Err(e) => println!("couldn't send to client: {e:?}"),
    }
}
