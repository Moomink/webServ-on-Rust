pub struct HttpRequest {
    method: Method,
    version: f32,
    uri: String,
    header: Method,
}

pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}
