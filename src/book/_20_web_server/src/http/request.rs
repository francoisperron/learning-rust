#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub url: String
}

impl Request {
    pub fn from(request_line: &str) -> Request {
        let mut parts = request_line.split_whitespace();
        Request {
            method: Method::from(parts.next().unwrap()),
            url: parts.next().unwrap().to_string()
        }
    }
}

#[derive(Debug)]
pub enum Method {
    Get
}

impl Method {
    pub fn from(method: &str) -> Method {
        match method {
            "GET" => Method::Get,
            _ => panic!()
        }
    }
}