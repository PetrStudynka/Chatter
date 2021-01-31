//Chatter messaging protocol
//protocol over tcp
use std::{io::{ErrorKind, Read, Write}, marker::PhantomData, net::*,cell::RefCell};
pub enum SendResult {
    Ok,
    Error(TransportError),
}

pub enum TransportError {
    MaxPayloadSizeExceeded,
    Disconnected,
    Undefined,
}

pub enum GetResult<T> {
    Ok(T),
    Empty,
    Error(TransportError),
}
pub struct CMPStream<T: Transferable> {
    stream: TcpStream,
    buff: [u8;8],
    phantom: PhantomData<T>
}

impl<T: Transferable> CMPStream<T> {
    pub fn from_stream(stream: TcpStream) -> Self {
        stream.set_nonblocking(true).unwrap();
        let buff = [0;8]; 
        let phantom = PhantomData;
        CMPStream{stream,buff,phantom}
    }

    pub fn send(&mut self, content: T) -> SendResult {

        if content.to_bytes().len() > usize::MAX {
            return SendResult::Error(TransportError::MaxPayloadSizeExceeded)
        }

        let content = self.parse_into(content.to_bytes());

        match self.stream.write_all(content.as_slice()) {
            Ok(_) => SendResult::Ok,
            Err(e) if e.kind() == ErrorKind::Interrupted => SendResult::Ok,
            Err(_) => SendResult::Error(TransportError::Undefined),
        }
    }

    pub fn get(&mut self) -> GetResult<T> {
        match self.stream.read_exact(&mut self.buff) {
            Ok(_) => {
                let msg_size = usize::from_le_bytes(self.buff);
                if msg_size > 0 {
                    let mut msg_buff:Vec<u8> = vec![0;msg_size];
                    match self.stream.read_exact(&mut msg_buff) {
                        Ok(_) => {
                           let msg = Transferable::from_bytes(msg_buff);
                           GetResult::Ok(msg)
                        },
                        Err(e) if e.kind() == ErrorKind::WouldBlock => GetResult::Empty,
                        Err(_) => GetResult::Error(TransportError::Undefined),
                    }

                } else {
                    GetResult::Empty
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => GetResult::Empty,
            Err(_) => GetResult::Error(TransportError::Undefined),
        }

    }

    fn parse_into(&self,msg: Vec<u8>) -> Vec<u8> {
        let size = msg.len();
        let mut buffer: Vec<u8> = Vec::with_capacity(size + std::mem::size_of::<usize>());

        buffer.write_all(&size.to_ne_bytes()).unwrap();
        buffer.write_all(&msg.as_slice()).unwrap();
        buffer
    }
}

pub trait Transferable {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(vec: Vec<u8>) -> Self;
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn parse(){
        let msg = vec![1;3];
        let parsed_msg = CMPStream::parse_into(msg);

        assert_eq!([3,0,0,0,0,0,0,0,1,1,1],parsed_msg.as_slice());
    }
}

