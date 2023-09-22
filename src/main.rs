use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use web_server::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7676").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let data: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|value| value.len() > 0)
        .collect();
    let path = path_reader(&data[0]);
    println!("path requested is:{}", path);

    let (body, version) = match path {
        "/" => (
            fs::read_to_string("src/html/index.html").unwrap(),
            "HTTP/1.1 200 OK",
        ),
        "/help" => {
            thread::sleep(Duration::from_millis(5000));
            (
                fs::read_to_string("src/html/help.html").unwrap(),
                "HTTP/1.1 200 OK",
            )
        }
        _ => (
            fs::read_to_string("src/html/404.html").unwrap(),
            "HTTP/1.1 200 OK",
        ),
    };
    let body_length = body.len();
    match path {
        "/" => {
            let response = format!("{version}\r\nContent-Length: {body_length}\r\n\r\n{body}");
            stream.write_all(response.as_bytes()).unwrap();
        }
        "/help" => {
            let response = format!("{version}\r\nContent-Length: {body_length}\r\n\r\n{body}");
            stream.write_all(response.as_bytes()).unwrap();
        }
        _ => {
            let response = format!("{version}\r\nContent-Length: {body_length}\r\n\r\n{body}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn path_reader(path: &str) -> &str {
    let data = path;
    let mut a = data.split_whitespace();
    a.next();
    a.next().unwrap()
}

