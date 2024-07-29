use eyre::Result;
use std::env;

use bittorrent_rust::{
    decode::Decoder, encode::Encoder, handshake::Handshake, parse::Parser, peers::Peer,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        tracing::info!("Decode command started");
        let encoded_value = &args[2];
        let decoded_value = Decoder::decode_bencoded_value(encoded_value)?;
        tracing::info!("{}", decoded_value);
    } else if command == "info" {
        let file_name = &args[2];
        let torrent_dict = Parser::read_torrent_file(file_name)?;
        let torrent = Parser::parse_torrent_file(torrent_dict)?;
        println!("Tracker URL: {}", torrent.announce_url);
        println!("Length: {}", torrent.info.length);
        println!("Info Hash: {}", torrent.hash);
        println!("Piece Length: {}", torrent.info.piece_length);
        Parser::split_and_display_sha1_hashes(torrent.info.pieces);
    } else if command == "encode" {
        // let file_path = &args[2];
        let file_path = "examples/example_file.txt";
        let announce_url = "http://example.com/announce";
        let piece_length = 512 * 1024; // 512 KB
        Encoder::encode_file(file_path, announce_url, piece_length)?;
    } else if command == "peers" {
        let file_path = &args[2];
        let torrent_dict = Parser::read_torrent_file(file_path)?;
        Peer::discover_peers(torrent_dict).await?;
    } else if command == "handshake" {
        let file_path = &args[2];
        let peer_addr = &args[3];
        let peer = peer_addr.parse::<Peer>()?;
        let torrent_dict = Parser::read_torrent_file(file_path)?;
        Handshake::peer_handshake(torrent_dict, peer).await?;
    } else {
        tracing::info!("unknown command: {}", args[1]);
    }
    Ok(())
}
