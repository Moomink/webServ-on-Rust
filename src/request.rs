use std::{
    collections::HashMap,
    str::{self, FromStr},
};

#[derive(Debug)]
pub enum RequestMethod {
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
    pub fn from_str(method: &str) -> Result<RequestMethod, &str> {
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

    pub fn to_string(&self) -> &str {
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

#[derive(Debug, Clone)]
pub enum PayloadType {
    Text(String),
    Binary(Vec<u8>),
    None,
}

#[derive(Default, Debug)]
pub struct HttpRequest {
    method: RequestMethod,
    version: f32,
    uri: String,
    header: HttpHeader,
}

pub type HttpHeader = HashMap<String, String>;

impl HttpRequest {
    pub fn new() -> Self {
        return HttpRequest::default();
    }

    pub fn header(&mut self, header: HttpHeader) {
        self.header = header;
    }

    pub fn get_header(&self) -> &HashMap<String, String> {
        return &self.header;
    }

    pub fn uri(&mut self, uri: &str) {
        self.uri = uri.to_string();
    }

    pub fn get_uri(&self) -> &String {
        return &self.uri;
    }

    pub fn version(&mut self, version: f32) {
        self.version = version
    }

    pub fn get_version(&self) -> f32 {
        return self.version;
    }

    pub fn method(&mut self, method: &str) {
        self.method = RequestMethod::from_str(method).unwrap()
    }

    pub fn from_buffer(buf: &[u8]) -> HttpRequest {
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
