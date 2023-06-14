use std::{
    fs,
    io::{prelude::*, BufReader, Write},
    net::TcpListener,
    net::TcpStream,
    thread,
    time::Duration,
};

const HOST: &'static str = "127.0.0.1";
const PORT: &'static str = "8000";
const METHOD: usize = 0;
const PATH: usize = 1;
const PROTOCOL: usize = 2;

enum Method {
    GET,
    POST,
}

struct Route {
    method: Method,
    path: String,
    handler: fn(),
}

struct Router {
    routes: Vec<Route>,
}

fn method(request: &Vec<String>) -> Method {
    let route_line = request.into_iter().next().unwrap();
    let route: Vec<_> = route_line.split(' ').collect();
    let method = route[METHOD];
    let path = route[PATH];

    if method == "GET" {
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

        thread::spawn(move || {
            let request = handle_connection(&mut stream);
            let method = method(&request);
            let status = match method {
                Method::GET => (200, "OK"),
                _ => (404, "NOT FOUND"),
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
                _ => fs::read_to_string("src/404.html").unwrap(),
            };
            let length = page.len();
            thread::sleep(Duration::from_secs(5));
            stream
                .write_fmt(format_args!(
                    "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
                    code, msg, length, page
                ))
                .unwrap();

            println!("Connection established!");
        });
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
