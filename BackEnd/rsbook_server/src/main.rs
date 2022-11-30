use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener.incoming().for_each(|stream| {
        let stream = stream.unwrap();
        handle_connection(stream);
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_response: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_response);

    let request_line = http_response.iter().next().unwrap();

    let response = if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line, length, contents
        )
    } else {
        unimplemented!()
    };

    stream.write_all(response.as_bytes()).unwrap();
}
