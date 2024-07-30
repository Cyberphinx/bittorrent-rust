use std::collections::HashMap;

use eyre::Result;
use tokio::net::TcpStream;

pub struct Downloader;

impl Downloader {
    pub async fn download_a_piece(
        output_path: &str,
        peer: &mut TcpStream,
        dictionary: &HashMap<Vec<u8>, serde_bencode::value::Value>,
        piece: &i32,
    ) -> Result<()> {
        // let mut peer = tokio_util::codec::Framed::new(peer, MessageFramer);
        todo!()
    }
}
