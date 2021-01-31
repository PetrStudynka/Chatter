use cmp::Transferable;
use serde::{Deserialize, Serialize};
use std::fmt;

pub const SIZE: usize = 256;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    content: String,
    time: String,
    user: String,
}

impl Message {
    pub fn new(content: String, time: String, user: String) -> Message {
        Message {
            content,
            time,
            user,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}: {}", self.user, self.time, self.content)
    }
}

impl Transferable for Message {

    fn from_bytes(vec: Vec<u8>) -> Self {
       bincode::deserialize(&vec).unwrap()
        
    }

    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /* 
    #[test]
    fn serialize_deserialize_flow() {
        let content = "Message content".to_string();
        let time = "01:10:10".to_string();
        let user = "Username".to_string();
        let mut msg: Message = Message::new(content.to_owned(), time.to_owned(), user.to_owned());

        let binary = Message::into_bin(&mut msg).ok();
        assert!(binary.is_some());

        let binary = binary.unwrap();
        assert!(binary.len() > 0);

        let result_msg = Message::from_bin(binary).unwrap();

        assert_eq!(result_msg.content, content);
        assert_eq!(result_msg.time, time);
        assert_eq!(result_msg.user, user);
    }
    */
}
