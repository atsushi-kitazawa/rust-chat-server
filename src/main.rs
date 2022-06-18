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
        let client_addr = stream.peer_addr().unwrap().to_string();
        println!("joined {}", client_addr);
        clients.try_lock().unwrap().insert(
            client_addr,
            stream.try_clone().expect("failed clone stream"),
        );
        
        // print client list
        println!("clients {:?}", clients.try_lock().unwrap().keys());

        // request hander per client
        let clients_ref = clients.clone();
        let tx_ref = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            handle_connection(&mut stream, tx_ref);
            let leave_client = &stream.peer_addr().unwrap().to_string();
            clients_ref
                .try_lock()
                .unwrap()
                .remove(leave_client);
            println!("leaved {}", leave_client);
        });
    }
}

fn handle_connection(stream: &mut TcpStream, tx: Sender<String>) {
    loop {
        let mut buf = [0; 512];
        let ret = stream.read(&mut buf);
        match ret {
            Result::Ok(size) => {
                println!("msg size={}", size);
                if buf[0] == 255 || size == 0 {
                    // receive ctrl + c or ctrl + ]
                    break;
                } else {
                    let s = String::from_utf8_lossy(&buf[..]);
                    // let client_address = stream.peer_addr().unwrap().to_string();
                    // println!("request({}) {} {:?}", client_address, &s, buf);

                    let client_address = stream.peer_addr().unwrap().to_string();
                    tx.send(format!("({}):{}", client_address, s.to_string())).unwrap();
                }
            }
            Result::Err(_) => {
                println!("handle_connection() read error");
            }
        }
    }
}

fn broadcast(tr: Receiver<String>, clients_ref: Arc<Mutex<HashMap<String, TcpStream>>>) {
    loop {
        let msg = tr.recv().unwrap();
        for stream in clients_ref.try_lock().unwrap().values_mut() {
            stream.write(msg.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
