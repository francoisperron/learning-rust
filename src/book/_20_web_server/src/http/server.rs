use crate::http::request::Request;
use crate::threads::thread_pool::ThreadPool;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

pub struct Server {}

impl Server {
    pub fn start(base_path: &str) {
        let address = "127.0.0.1:7878";
        let listener = TcpListener::bind(address).unwrap();
        let pool = ThreadPool::new(4);
        println!("Server running on http://{address}");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let base_path = base_path.to_string();
            pool.execute(move || Server::handle(stream, &base_path));
        }
    }

    fn handle(mut stream: TcpStream, base_path: &str) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let request = Request::from(&request_line);
        println!("{:?}", request);

        match request.url.as_str() {
            "/" => Server::response(stream, "HTTP/1.1 200 OK", &format!("{}/index.html", base_path)),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                Server::response(stream, "HTTP/1.1 200 OK", &format!("{}/index.html", base_path))
            }
            _ => Server::response(stream, "HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    }

    fn response(mut stream: TcpStream, status: &str, file: &str) {
        let contents = fs::read_to_string(file).unwrap_or_else(|_| "file not found".to_string());
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, contents.len(), contents);

        stream.write_all(response.as_bytes()).unwrap();
    }
}
