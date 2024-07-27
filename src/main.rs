use eyre::Result;
use std::env;

use bittorrent_rust::{decode::Decoder, encode::Encoder, parse::Parser};

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        println!("Decode command started");

        let encoded_value = &args[2];
        let decoded_value = Decoder::decode_bencoded_value(encoded_value)?;
        tracing::info!("{}", decoded_value);
    } else if command == "info" {
        let file_name = &args[2];
        let torrent = Parser::parse_torrent_file(file_name)?;
        println!("Tracker URL: {}", torrent.announce_url);
        println!("Length: {}", torrent.info.length);
    } else if command == "encode" {
        // let file_path = &args[2];
        let file_path = "examples/example_file.txt";
        let announce_url = "http://example.com/announce";
        let piece_length = 512 * 1024; // 512 KB

        Encoder::encode_file(file_path, announce_url, piece_length)?;
    } else {
        println!("unknown command: {}", args[1]);
    }
    Ok(())
}
