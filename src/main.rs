use std::{
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
    thread
};

    const OK_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
    const CREATED_HEADER: &str = "HTTP/1.1 201 CREATED\r\n\r\n";
    const ACCECPTED_HEADER: &str = "HTTP/1.1 202 ACCEPTED\r\n\r\n";
    const NO_CONTENT_HEADER: &str = "HTTP/1.1 204 NO CONTENT\r\n\r\n";

    const MOVED_PERMANENTLY_HEADER: &str = "HTTP/1.1 301 MOVED PERMANENTLY\r\n\r\n";

    const BAD_REQUEST_HEADER: &str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
    const UNAUTHORIZED_HEADER: &str = "HTTP/1.1 401 UNAUTHORIZED\r\n\r\n";
    const FORBIDDEN_HEADER: &str = "HTTP/1.1 403 FORBIDDEN\r\n\r\n";
    const NOT_FOUND_HEADER: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    const METHOD_NOT_ALLOWED_HEADER: &str = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parts: Vec<&str> = http_request[0].split_whitespace().collect();
    let (method, path, _version) = (parts[0], parts[1], parts[2]);

    match method {
        "GET" => {
            let response = match path {
                "/" => OK_HEADER,
                _ => NOT_FOUND_HEADER,
            };

            send_response(&mut stream, response);
        }
        _ => {
            let response = METHOD_NOT_ALLOWED_HEADER;
            
            send_response(&mut stream, response);
        }
    }
}

fn send_response(stream: &mut TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

