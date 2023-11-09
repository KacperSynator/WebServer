use log::{debug, error};

use std::{
    fs,
    net::{TcpListener, TcpStream},
};

use web_server::{
    http_request::HttpMethod,
    http_response::{HttpResponse, HttpStatus},
    server::parse_request,
    ThreadPool,
};

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

fn handle_connection(stream: TcpStream) {
    let request = parse_request(&stream);

    if let Err(e) = request {
        error!("Request parse failed with {e}");
        return;
    }

    let request = request.unwrap();
    debug!("received: {:?}", request);

    let (filename, status) = match (request.method, request.path.as_str()) {
        (HttpMethod::Get, "/") => (HELLO_PAGE, HttpStatus::Ok),
        _ => (NOT_FOUND_PAGE, HttpStatus::NotFound),
    };

    let data = fs::read_to_string(filename);

    if let Err(e) = data {
        error!("Failed to read data: {e}");
        return;
    }

    let response = HttpResponse::from_html(status, request.protocol, data.unwrap());

    if let Err(e) = response.write(&stream) {
        error!("Failed to send response: {e}");
    }
}
