use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => loop {
                let mut request_buffer = [0; 512];
                stream.read(&mut request_buffer).unwrap();
                let response_string = "+PONG\r\n".as_bytes();
                stream.write(response_string).unwrap();
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
