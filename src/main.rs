use std::net::TcpListener;

mod connection;


fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        connection::handle_connection(stream);
    }
}

