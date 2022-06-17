use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    str,
};

fn main() {
    let result = TcpListener::bind("127.0.0.1:8888");

    match result {
        Result::Ok(listner) => {
            for stream in listner.incoming() {
                let stream = stream.unwrap();
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
        }
        Result::Err(_e) => {
            println!("NG: bind()")
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let client_address = stream.peer_addr().unwrap();
    loop {
        let mut buf = [0; 512];
        stream.read(&mut buf).unwrap();
        println!("request({}) {}", client_address, String::from_utf8_lossy(&buf[..]));

        let reply = String::from("hello ") + str::from_utf8(&buf).unwrap();
        stream.write(reply.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
