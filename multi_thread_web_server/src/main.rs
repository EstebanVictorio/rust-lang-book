use multi_thread_web_server_pool::WorkerPool;
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

enum Route {
    Home,
    Sleep,
    NotFound,
}

fn route(request: &Vec<String>) -> Route {
    let route_line = request.into_iter().next().unwrap();
    let route: Vec<_> = route_line.split(' ').collect();
    let path = route[PATH];

    match path {
        "/" => Route::Home,
        "/sleep" => Route::Sleep,
        _ => Route::NotFound,
    }
}

fn method(request: &Vec<String>) -> Method {
    let route_line = request.into_iter().next().unwrap();
    let route: Vec<_> = route_line.split(' ').collect();
    let method = route[METHOD];

    if method == "GET" {
        return Method::GET;
    }

    Method::POST
}

fn main() {
    let pool = WorkerPool::new(4);
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

        pool.execute(move |id: usize| {
            let request = handle_connection(&mut stream);
            let method = method(&request);
            let route = route(&request);
            let status = match method {
                Method::GET => (200, "OK"),
                _ => (404, "NOT FOUND"),
            };
            let (code, msg) = status;

            let page = match (method, route) {
                (Method::GET, Route::Home) => fs::read_to_string("src/index.html").unwrap(),
                (Method::GET, Route::Sleep) => {
                    thread::sleep(Duration::from_secs(5));
                    fs::read_to_string("src/heavy-task.html").unwrap()
                }
                _ => fs::read_to_string("src/404.html").unwrap(),
            };
            let length = page.len();

            stream
                .write_fmt(format_args!(
                    "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
                    code, msg, length, page
                ))
                .unwrap();

            println!("Connection established!");
            println!("Worker {id} finished task.");
            Ok(())
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

    // println!("Request: {:?}", request);
    request
}
