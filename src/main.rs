use std::net::TcpListener;

mod connection;
mod multithreading;


fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = multithreading::ThreadPool::new(4);

    for stream in listener.incoming().take(100) {

        let stream = stream.unwrap();
        pool.execute(|| {
            connection::handle_connection(stream);
        });
    }

    println!("Shutting down gracefully");
}

