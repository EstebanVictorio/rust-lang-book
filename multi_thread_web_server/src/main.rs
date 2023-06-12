use std::{
    fs,
    io::{prelude::*, BufReader, Write},
    net::TcpListener,
    net::TcpStream,
};

const HOST: &'static str = "127.0.0.1";
const PORT: &'static str = "8000";

enum Method {
    GET,
    POST,
}

fn method(request: &Vec<String>) -> Method {
    let method_line = request.into_iter().next().unwrap();

    if method_line == "GET / HTTP/1.1" {
        return Method::GET;
    }

    Method::POST
}

fn main() {
    let address = String::from(HOST) + ":" + PORT;
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind to address: {}", e);
        }
    };

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                panic!("Failed to establish a connection: {}", e);
            }
        };

        let request = handle_connection(&mut stream);
        let method = method(&request);
        let status = match method {
            Method::GET => (200, "OK"),
            Method::POST => (404, "NOT FOUND"),
        };
        let (code, msg) = status;
        println!(
            "Method: {:?}",
            match method {
                Method::GET => "GET",
                Method::POST => "POST",
            }
        );
        let page = match method {
            Method::GET => fs::read_to_string("src/index.html").unwrap(),
            Method::POST => fs::read_to_string("src/404.html").unwrap(),
        };
        let length = page.len();
        stream
            .write_fmt(format_args!(
                "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
                code, msg, length, page
            ))
            .unwrap();

        println!("Connection established!");
    }
}

fn handle_connection(stream: &mut TcpStream) -> Vec<String> {
    let buffer = BufReader::new(stream);
    let request: Vec<_> = buffer
        .lines()
        .map(|r| r.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:?}", request);
    request
}
