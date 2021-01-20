use std::{io::ErrorKind, io::Write, sync::mpsc::{TryRecvError, channel}};
use std::io::*;
use std::{io::stdin, net::TcpStream, thread};

const MSG_SIZE: usize = 256;

fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:33333").expect("failed to connect to the addr");
    stream.set_nonblocking(true).expect("failed to set nonblocking mode");

    let (sender, receiver) = channel::<String>();

    let mut buffer = vec![0;MSG_SIZE];

    thread::spawn(move || loop {

        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                println!(": {}",String::from_utf8(buffer.to_owned()).unwrap())
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                panic!("Lost connection with the server");
            }
        }

        match receiver.try_recv() {
            Ok(result) => {
                let mut result = result.as_bytes().to_vec();
                result.resize(MSG_SIZE, 0);
                stream.write_all(&result).expect("failed to write to stream");
            },
            Err(TryRecvError::Disconnected) => {
               panic!("Channel has been closed");
            },
            Err(TryRecvError::Empty) => (),
        }

        
    });

    loop {
        let mut buffer = String::with_capacity(MSG_SIZE);
        stdin().read_line(&mut buffer).expect("failed to read input");
        
        if buffer.starts_with("quit") {
            break;
        }

        sender.send(buffer).expect("failed to write msg to the stream");
    }

}