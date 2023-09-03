use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::{self},
    thread,
};

use crate::{
    request::{HttpHeader, HttpRequest, PayloadType},
    response::HttpResponse,
};

pub struct HttpHandler {
    port: u16,
    ip_address: String,
    worker_num: u8,
}

impl HttpHandler {
    pub fn new() -> Self {
        HttpHandler {
            port: 80,
            ip_address: "0.0.0.0".to_string(),
            worker_num: 1,
        }
    }

    pub fn port(&mut self, port_number: u16) {
        self.port = port_number;
    }

    pub fn ip_address(&mut self, ip_address: &str) {
        self.ip_address = ip_address.to_string();
    }

    pub fn worker_num(&mut self, worker_num: u8) {
        self.worker_num = worker_num;
    }

    pub fn start(&mut self) {
        let bind_ip = format!("{}:{}", self.ip_address, self.port.to_string());

        let listener = TcpListener::bind(&bind_ip).unwrap();

        println!("connected IP:port [{}]", bind_ip);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("from {} receice.", stream.peer_addr().unwrap());
                    let _handle = thread::spawn(move || {
                        stream_handler(&stream);
                    });
                }
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }
    }
}

pub fn stream_handler(mut stream: &TcpStream) {
    loop {
        let mut buffer: Vec<u8> = Vec::new();
        let mut tmp_buffer: [u8; 1] = [0; 1];

        loop {
            stream.read(&mut tmp_buffer).unwrap();
            buffer.push(tmp_buffer[0]);
            if buffer.len() >= 8 {
                if buffer.ends_with(b"\r\n\r\n") {
                    break;
                }
            }
        }

        let request = HttpRequest::from_buffer(&buffer);
        println!("{:?}", request);

        let mut response = HttpResponse::new();

        response.status_code(200);
        response.reason("OK");
        response.version(1.1);
        let mut response_header: HttpHeader = HttpHeader::new();

        let uri = request.get_uri();
        let file_path = if uri.as_str() == "/" {
            "www/index.html".to_string()
        } else {
            format!("www{}", uri)
        };

        match fs::read(file_path.clone()) {
            Ok(data) => {
                let ftype = infer::get_from_path(file_path)
                    .expect("file read successfully")
                    .expect("file type is known");
                response_header.insert("Content-Type".to_string(), ftype.mime_type().to_string());

                let file_size = data.len();
                response_header.insert("Content-Length".to_string(), file_size.to_string());

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

        let header = request.get_header();
        let current_connection = match header.get("Connection") {
            Some(v) => match (*v).as_str() {
                "keep-alive" => "keep-alive",
                "close" | &_ => "close",
            },
            None => "None",
        };
        if current_connection != "None" {
            response_header.insert("Connection".to_string(), current_connection.to_string());
        }

        response.header(response_header);

        println!("{:?}", response);
        match stream.write(&response.as_bytes()) {
            Ok(_) => {
                match current_connection {
                    "keep-alive" => continue,
                    "None" | &_ => {
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("shutdown error");
                        println!("from {} shutdown success!.", stream.peer_addr().unwrap());
                        break;
                    }
                };
            }
            Err(e) => println!("couldn't send to client: {e:?}"),
        }
    }
}
