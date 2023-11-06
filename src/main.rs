use log::{debug, info};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use web_server::ThreadPool;

const ADDRESS: &str = "localhost";
const PORT: &str = "7878";
const HELLO_PAGE: &str = "web_pages/hello.html";
const NOT_FOUND_PAGE: &str = "web_pages/404.html";
const THREAD_NUM: usize = 4;

fn main() {
    pretty_env_logger::init();

    let listener = TcpListener::bind(format!("{ADDRESS}:{PORT}")).unwrap();
    let pool = ThreadPool::build(THREAD_NUM).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", HELLO_PAGE),
        _ => ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_PAGE),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
