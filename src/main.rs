use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn main() {
    let result = TcpListener::bind("127.0.0.1:8888");

    match result {
        Result::Ok(listner) => {
            for stream in listner.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream)
            }
        }
        Result::Err(_e) => {
            println!("NG: bind()")
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    println!("request {}", String::from_utf8_lossy(&buf[..]));

    let reply = String::from("hello ") + str::from_utf8(&buf).unwrap();
    stream.write(reply.as_bytes()).unwrap();
    stream.flush().unwrap();
}
