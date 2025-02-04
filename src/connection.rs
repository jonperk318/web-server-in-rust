use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpStream},
};


pub fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if request_line == "GET /page HTTP/1.1" {
        ("HTTP/1.1 200 OK", "public/page.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
