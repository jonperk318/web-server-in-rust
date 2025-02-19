# Web Server in Rust
This is a multi-threaded web server built in Rust. Test it locally by following the instructions below.

## Setup
Install Rust
```bash
curl https://sh.rustup.rs -sSf | sh
```
Clone the repository
```bash
git clone https://github.com/jonperk318/web-server-in-rust.git
```

## Run the Server
Run using Cargo
```bash
cd web-server-in-rust
cargo run
```
Open [http://localhost:8080](http://localhost:8080) with a web browser to see the result.


![demo.png](./demo.png)


## Test the Server
Test the capabilities of the server with `curl`
```bash
curl -v localhost:8080/
```
Using two terminal processes, spam the command above after executing the command below. This should work up to 100 requests due to multi-threading.
```bash
curl -v localhost:8080/slow
```
Use the below command to get a 404.
```bash
curl -v localhost:3000/asdfghjkl
```


Finally, stop the server with `Ctrl+C`
