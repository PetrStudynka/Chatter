use mpsc::channel;
use core::panic;
use std::net::*;
use std::sync::*;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;
use std::io::*;
use thread::sleep;

const MSG_SIZE: usize = 256;
const MAX_CLIENTS: usize = 5;

#[allow(dead_code)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:33333").unwrap();
    let (tx, rx) = channel::<TcpStream>();
    let mut clients: Vec<TcpStream> = Vec::with_capacity(5);
    let mut buffer = [0; MSG_SIZE];

    thread::spawn(move || loop {

        match rx.try_recv() {
            Ok(stream) => {
                stream.set_nonblocking(true).unwrap();
                clients.push(stream)
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {
                panic!("Channel disconnected")
            }
        }

        if clients.len() > 0 {
            for (i,mut client) in &mut clients.iter().enumerate() {
                match client.read_exact(&mut buffer) {
                    Ok(_) => {
                        println!("msg: {}", String::from_utf8(buffer.to_vec()).unwrap());
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        //println!("read needed to block");
                    },
                    
                    Err(_) => {
                        println!("Lost connection with a client");
                        clients.remove(i);
                        break;
                    }
                }
            }
        }

        sleep(Duration::from_millis(300));
    });

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => tx.send(stream).unwrap(),
            Err(_) => {
                println!("Failed to accept a connection");
            }
        }
    }
}