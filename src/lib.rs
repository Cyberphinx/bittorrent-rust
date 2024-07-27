use serde::{Deserialize, Serialize};

pub mod decode;
pub mod encode;
pub mod parse;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub length: i64,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Torrent {
    pub info: Info,
    #[serde(rename = "announce")]
    pub announce_url: String,
}
