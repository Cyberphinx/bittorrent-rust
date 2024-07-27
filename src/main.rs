use std::env;

use bittorrent_rust::decode::decode_bencoded_value;

fn main() {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        println!("Decode command started");

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value).unwrap();
        tracing::info!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
