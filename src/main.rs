use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

const ADDRESS: &str = "127.0.0.1:8888";

fn main() {
    // map to hold per-client streams
    let clients = Arc::new(Mutex::new(HashMap::<String, TcpStream>::new()));

    // create channel
    let (tx, tr): (Sender<String>, Receiver<String>) = mpsc::channel();

    // broadcast
    let cliens_ref_for_broadcast = clients.clone();
    thread::spawn(move || broadcast(tr, cliens_ref_for_broadcast));

    let listner = TcpListener::bind(ADDRESS).expect("failed bind");
    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        clients.try_lock().unwrap().insert(
            stream.peer_addr().unwrap().to_string(),
            stream.try_clone().expect("failed clone stream"),
        );
        // println!("{:?}", room);

        // request hander per client
        let clients_ref = clients.clone();
        let tx_ref = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            handle_connection(&mut stream, tx_ref);
            clients_ref
                .try_lock()
                .unwrap()
                .remove(&stream.peer_addr().unwrap().to_string());
        });
    }
}

fn handle_connection(stream: &mut TcpStream, tx: Sender<String>) {
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
                    let s = String::from_utf8_lossy(&buf[..]);
                    println!("request({}) {} {:?}", client_address, &s, buf);
                    tx.send(s.to_string()).unwrap();
                    print!("send message {}", &s);
                }
            }
            Result::Err(_) => {
                println!("handle_connection() read error");
            }
        }

        // todo (invalid utf8 data error handling)
        // and move to "match ret Result Ok"
        let reply = String::from("hello ") + str::from_utf8(&buf).unwrap();
        stream.write(reply.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn broadcast(tr: Receiver<String>, clients_ref: Arc<Mutex<HashMap<String, TcpStream>>>) {
    loop {
        println!("waiting receive message....");
        let msg = tr.recv().unwrap();
        println!("received msg {}", msg);
        for stream in clients_ref.try_lock().unwrap().values_mut() {
            stream.write(msg.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
