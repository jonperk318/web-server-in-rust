use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpStream},
};


pub fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" { // 200

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("public/index.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();

    } else { // 404

        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("public/404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();

    }

}
