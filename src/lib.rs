use serde::{Deserialize, Serialize};

pub mod decode;
pub mod encode;
pub mod parse;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    name: String,
    length: u64,
    #[serde(rename = "piece length")]
    piece_length: u64,
    pieces: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Torrent {
    info: Info,
    #[serde(rename = "announce")]
    announce_url: String,
}
