use std::{
    io::Write,
    net::{TcpListener, TcpStream}, thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| send_response(stream));
                    println!("accepted new connection");
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}

fn send_response(mut stream: TcpStream) {
    if let Err(e) = stream.write_all(b"+PONG\r\n") {
        println!("Couldn't send response {e}");
    }
}
