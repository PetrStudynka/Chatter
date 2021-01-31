use core::panic;
use mpsc::channel;
use std::{borrow::BorrowMut, net::*};
use std::sync::mpsc::TryRecvError;
use std::sync::*;
use std::thread;
use std::time::Duration;
use thread::sleep;
use cmp::*;
use message::Message;

const MSG_SIZE: usize = 1;
const MAX_CLIENTS: usize = 5;

const PORT: usize = 33333;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("IP addr must be specified");
    }

    let addr = format!("{}:{}", args[1].to_owned(), PORT);

    let listener = TcpListener::bind(addr).unwrap();
    let (tx, rx) = channel::<CMPStream<Message>>();
    let mut clients: Vec<CMPStream<Message>> = Vec::with_capacity(MAX_CLIENTS);

    thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(stream) => {
                if clients.len() <= MAX_CLIENTS {
                    clients.push(stream)
                } else {
                    println!("Server cannot handle more than {} clients.", MAX_CLIENTS)
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                panic!("Channel disconnected")
            }
        }

        if clients.len() > 1 {
            for (i,client) in clients.iter().enumerate() {
                match client.get() {
                    GetResult::Ok(msg) => {
                        clients.iter().enumerate().for_each(|(j,mut stream)| {
                            if i != j {
                                stream.send(msg.clone());
                            }
                        });
                    }
                    GetResult::Empty => (),
                    GetResult::Error(_) => {
                        panic!("Failed to get msg from CMPStream");
                    }
                }
            }
        }

        sleep(Duration::from_millis(300));
    });

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                let stream: CMPStream<Message> = CMPStream::from_stream(stream);
                tx.send(stream).unwrap()
            },
            Err(_) => {
                println!("Failed to accept a connection");
            }
        }
    }
}
