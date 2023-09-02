use std::str::{self};

use crate::request::{HttpHeader, PayloadType};

#[derive(Debug)]
pub struct HttpResponse {
    status_code: u16,
    version: f32, //TODO 桁数調整
    reason: String,
    header: HttpHeader,
    payload: PayloadType,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            status_code: 0,
            version: 0.0,
            reason: "".to_string(),
            header: HttpHeader::default(),
            payload: PayloadType::None,
        }
    }

    pub fn status_code(&mut self, code: u16) {
        self.status_code = code;
    }

    pub fn version(&mut self, version: f32) {
        self.version = version;
    }

    pub fn reason(&mut self, reason: &str) {
        self.reason = reason.to_string();
    }

    pub fn header(&mut self, header: HttpHeader) {
        self.header = header;
    }

    pub fn payload(&mut self, payload: PayloadType) {
        self.payload = PayloadType::None;
        self.payload = payload.clone();
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut messages: String = format!(
            "HTTP/{} {} {}\n",
            self.version, self.status_code, self.reason
        );
        let mut headers: Vec<String> = Vec::new();
        for (k, v) in self.header.iter() {
            headers.push(format!("{k}: {v}\n"));
        }
        headers.push("\n".to_string());

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
