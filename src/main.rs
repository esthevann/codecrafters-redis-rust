use std::{
    io::{Write, Read},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                send_response(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn send_response(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        // Wait for the client to send us a message but ignore the content for now
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }
    }
    stream.write(b"+PONG\r\n").unwrap();
}
