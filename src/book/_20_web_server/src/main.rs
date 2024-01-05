use std::{fs, thread};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use _20_web_server::http::request::Request;
use _20_web_server::threads::thread_pool::ThreadPool;


fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(4);
    println!("Server running on {address}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| { handle(stream) });
    }
}

fn handle(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request = Request::from(&request_line);
    println!("{:?}", request);

    match request.url.as_str() {
        "/" => response(stream, "HTTP/1.1 200 OK", "index.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            response(stream, "HTTP/1.1 200 OK", "index.html")
        }
        _ => response(stream, "HTTP/1.1 404 NOT FOUND", "404.html"),
    }
}

fn response(mut stream: TcpStream, status: &str, file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, contents.len(), contents);

    stream.write_all(response.as_bytes()).unwrap();
}