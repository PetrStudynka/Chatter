use std::{io::ErrorKind, io::Write, sync::mpsc::{TryRecvError, channel}};
use std::io::*;
use std::{io::stdin, net::TcpStream, thread};
use chrono::prelude::*;
use message::{Message};
use std::time::Duration;
use thread::sleep;

const MSG_SIZE: usize = 256;
const PORT: usize = 33333;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Username and IP addr must be specified");
    }

    let username = args[1].to_owned();
    let addr = format!("{}:{}",args[2].to_owned(),PORT);

    let mut stream = TcpStream::connect(addr).expect("failed to connect to the address");
    stream.set_nonblocking(true).expect("failed to set nonblocking mode");

    let (sender, receiver) = channel::<String>();


    thread::spawn(move || loop {
        let mut buffer = vec![0;1];

        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let count = buffer[0] as usize;
                let mut msg_buffer = vec![0;count]; 
                stream.read_exact(&mut msg_buffer).unwrap();
               
                let msg = Message::from_bin(msg_buffer).unwrap();
                println!("{}", msg);
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) if e.kind() == ErrorKind::Interrupted => println!("interupted"),
            Err(_) => {
                panic!("Lost connection with the server");
            }
        }

        match receiver.try_recv() {
            Ok(mut content) => {

                //TODO clean this dirt
                let _ = content.pop(); //LF

                let time = Utc::now();
                let time = time.format("%H:%M:%S").to_string();

                let mut message = Message::new(content, time, username.clone());
                let mut bin_message = message.into_bin().unwrap();
                let len = bin_message.len() as u8;
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize +1);
                buffer.push(len);
                buffer.append(&mut bin_message);
                
                match stream.write_all(&buffer) {
                    Ok(_) => (),
                    Err(_) => panic!("Lost connection with the server"),
                }
            },
            Err(TryRecvError::Disconnected) => {
               panic!("Channel has been closed");
            },
            Err(TryRecvError::Empty) => (),
        }
        sleep(Duration::from_millis(300));
        
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