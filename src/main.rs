use std::net::TcpListener;

fn main() {
    let result = TcpListener::bind("127.0.0.1:8888");

    match result {
        Result::Ok(listner) => {
            accept(listner)
        },
        Result::Err(_e) => {println!("NG: bind()")},
    }
}

fn accept(listner: TcpListener) {
    let stream = listner.accept();
    match stream {
        Result::Ok((mut _stream, _)) => {println!("OK")},
        Result::Err(_e) => {println!("NG: accept()")},
    }
}