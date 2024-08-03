use eyre::{eyre, Result};

pub const BLOCK_SIZE: i32 = 16 * 1024;

// All the remaining messages in the protocol take the form of <length prefix><message ID><payload>

#[derive(Clone, Debug)]
pub struct Message {
    // The length prefix is a four byte big-endian value
    pub prefix: i32,

    // The message ID is a single decimal byte
    pub id: MESSAGE,

    // The payload is message dependent.
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(id: MESSAGE, payload: Vec<u8>) -> Self {
        Self {
            prefix: payload.len() as i32 + 1,
            id,
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [
            self.prefix.to_be_bytes().as_slice(),
            [self.id.clone() as u8; 1].as_slice(),
            self.payload.as_slice(),
        ]
        .concat()
    }
}

/// Peer Messages
/// All non-keepalive messages start with a single byte which gives their type.
/// https://www.bittorrent.org/beps/bep_0003.html#peer-messages
#[derive(PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum MESSAGE {
    BITFIELD = 5,
    INTERESTED = 2,
    UNCHOKE = 1,
    REQUEST = 6,
    PIECE = 7,
}

impl TryFrom<u8> for MESSAGE {
    type Error = eyre::Error;

    fn try_from(value: u8) -> Result<Self> {
        Err(eyre!("Invalid message ID: {value}"))
    }
}
