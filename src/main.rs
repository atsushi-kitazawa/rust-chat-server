use std::{
    collections::{HashMap},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::{Arc, Mutex},
    thread,
};

const ADDRESS: &str = "127.0.0.1:8888";

fn main() {
    let room = Arc::new(Mutex::new(HashMap::<String, TcpStream>::new()));

    let listner = TcpListener::bind(ADDRESS).expect("failed bind");
    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        room.try_lock().unwrap().insert(
            stream.peer_addr().unwrap().to_string(),
            stream.try_clone().expect("failed clone stream"),
        );
        // println!("{:?}", room);

        // request hander per client
        let room_ref = room.clone();
        thread::spawn(move || {
            handle_connection(&mut stream);
            room_ref
                .try_lock()
                .unwrap()
                .remove(&stream.peer_addr().unwrap().to_string());
        });

        // broadcast thread start
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let client_address = stream.peer_addr().unwrap().to_string();
    loop {
        let mut buf = [0; 512];
        let ret = stream.read(&mut buf);
        match ret {
            Result::Ok(size) => {
                println!("size={}", size);
                if buf[0] == 255 || size == 0 {
                    // receive ctrl + c or ctrl + ]
                    break;
                } else {
                    println!(
                        "request({}) {}, {:?}",
                        client_address,
                        String::from_utf8_lossy(&buf[..]),
                        buf
                    );
                }
            }
            Result::Err(_) => {
                println!("handle_connection() read error");
            }
        }

        // todo (invalid utf8 data error handling)
        let reply = String::from("hello ") + str::from_utf8(&buf).unwrap();
        stream.write(reply.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}