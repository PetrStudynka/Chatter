use chrono::prelude::*;
use message::Message;
use std::io::*;
use std::time::Duration;
use std::{io::stdin, net::TcpStream, thread};
use std::{
    io::ErrorKind,
    io::Write,
    sync::mpsc::{channel, TryRecvError},
};
use thread::sleep;
use cmp::*;

const MSG_SIZE: usize = 256;
const PORT: usize = 33333;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Username and IP addr must be specified");
    }

    let username = args[1].to_owned();
    let addr = format!("{}:{}", args[2].to_owned(), PORT);

    let mut stream = TcpStream::connect(addr).expect("failed to connect to the address");
    let mut stream: CMPStream<Message> = CMPStream::from_stream(stream);

    let (sender, receiver) = channel::<String>();

    thread::spawn(move || loop {

        match stream.get() {
            GetResult::Ok(msg) => {
                println!("{}", msg);
            }
            GetResult::Empty => (),
            GetResult::Error(_) => {
                panic!("Failed to get msg from CMPStream");
            }
        }

        match receiver.try_recv() {
            Ok(mut content) => {
                let _ = content.pop(); //LF

                let time = Utc::now();
                let time = time.format("%H:%M:%S").to_string();

                let message = Message::new(content.to_string(), time, username.clone());
                match stream.send(message) {
                    SendResult::Ok => {
                        println!("Me: {}",content)
                    }
                    SendResult::Error(_) => {
                        panic!("Failed to send message")
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                panic!("Channel has been closed");
            }
            Err(TryRecvError::Empty) => (),
        }
        sleep(Duration::from_millis(300));
    });

    loop {
        let mut buffer = String::with_capacity(MSG_SIZE);
        stdin()
            .read_line(&mut buffer)
            .expect("failed to read input");

        if buffer.starts_with("quit") {
            break;
        }

        sender
            .send(buffer)
            .expect("failed to write msg to the stream");
    }
}
