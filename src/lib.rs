use std::net::SocketAddrV4;

use serde::{Deserialize, Serialize};

pub mod decode;
pub mod encode;
pub mod parse;
pub mod peers;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub length: i64,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTorrent {
    pub info: Info,
    #[serde(rename = "announce")]
    pub announce_url: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTorrent {
    pub info: Info,
    #[serde(rename = "announce")]
    pub announce_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackerRequest {
    pub peer_id: String,
    pub port: u16,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub compact: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackerResponse {
    pub interval: usize,
    pub peers: Peers,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Peers(pub Vec<SocketAddrV4>);
