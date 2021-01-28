use mpsc::channel;
use core::panic;
use std::net::*;
use std::sync::*;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;
use std::io::*;
use thread::sleep;

const MSG_SIZE: usize = 1;
const MAX_CLIENTS: usize = 5;

const PORT: usize = 33333;

fn main() {

    let args: Vec<String> = std::env::args().collect();


    if args.len() < 2 {
        panic!("IP addr must be specified");
    }

    let addr = format!("{}:{}",args[1].to_owned(),PORT);

    let listener = TcpListener::bind(addr).unwrap();
    let (tx, rx) = channel::<TcpStream>();
    let mut clients: Vec<TcpStream> = Vec::with_capacity(MAX_CLIENTS);
    let mut buffer = [0; MSG_SIZE];

    thread::spawn(move || loop {

        match rx.try_recv() {
            Ok(stream) => {
                stream.set_nonblocking(true).unwrap();
                if clients.len() <= MAX_CLIENTS {
                    clients.push(stream)
                } else {
                    //TODO full capacity
                }
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {
                panic!("Channel disconnected")
            }
        }

        if clients.len() > 1 {
            for (i,mut client) in &mut clients.iter().enumerate() {
                match client.read_exact(&mut buffer) {
                    Ok(_) => {
                        let count = buffer[0] as usize;
                        let mut msg_buffer = vec![0;count]; 
                        client.read_exact(&mut msg_buffer).unwrap();

                        let mut result_msg: Vec<u8> = Vec::with_capacity(count);
                        result_msg.push(count as u8);
                        result_msg.append(&mut msg_buffer);
                        
                        clients.iter().enumerate().for_each(|(j, mut stream)| {
                             if i != j {
                                 stream.write_all(&result_msg).unwrap();
                             }
                        });             

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