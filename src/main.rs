use eyre::Result;
use std::env;

use bittorrent_rust::{
    decode::Decoder, downloader::Downloader, encode::Encoder, handshake::Handshake, parse::Parser,
    peers::Peer,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            tracing::info!("Decode command started");
            let encoded_value = &args[2];
            let decoded_value = Decoder::decode_bencoded_value(encoded_value)?;
            tracing::info!("{}", decoded_value);
        }
        "info" => {
            let file_name = &args[2];
            let torrent_dict = Parser::read_torrent_file(file_name)?;
            let torrent = Parser::parse_torrent_file(&torrent_dict)?;
            tracing::info!("Tracker URL: {}", torrent.announce_url);
            tracing::info!("Length: {}", torrent.info.length);
            tracing::info!("Info Hash: {}", torrent.hash);
            tracing::info!("Piece Length: {}", torrent.info.piece_length);
            Parser::split_and_display_sha1_hashes(torrent.info.pieces);
        }
        "encode" => {
            // let file_path = &args[2];
            let file_path = "examples/example_file.txt";
            let announce_url = "http://example.com/announce";
            let piece_length = 512 * 1024; // 512 KB
            Encoder::encode_file(file_path, announce_url, piece_length)?;
        }
        "peers" => {
            let file_path = &args[2];
            let torrent_dict = Parser::read_torrent_file(file_path)?;
            Peer::discover_peers(&torrent_dict).await?;
        }
        "handshake" => {
            let file_path = &args[2];
            let peer_addr = &args[3];
            let peer = peer_addr.parse::<Peer>()?;
            let torrent_dict = Parser::read_torrent_file(file_path)?;
            Handshake::peer_handshake(&torrent_dict, peer).await?;
        }
        "download_piece" => {
            let output_path = &args[2];
            let file_path = &args[3];
            let piece_index = &args[4];
            let torrent_dict = Parser::read_torrent_file(file_path)?;
            let torrent_file = Parser::parse_torrent_file(&torrent_dict)?;
            let tracker_response = Peer::discover_peers(&torrent_dict).await?;
            let (mut peer, _handshake) =
                Handshake::peer_handshake(&torrent_dict, tracker_response.peers.into()).await?;
            Downloader::download_a_piece(
                output_path,
                &mut peer,
                &torrent_file,
                &piece_index.parse::<i32>()?,
            )
            .await?;
        }
        "download" => {
            let output_path = &args[2];
            let file_path = &args[3];
            let torrent_dict = Parser::read_torrent_file(file_path)?;
            let torrent_file = Parser::parse_torrent_file(&torrent_dict)?;
            let tracker_response = Peer::discover_peers(&torrent_dict).await?;
            let (mut peer, _handshake) =
                Handshake::peer_handshake(&torrent_dict, tracker_response.peers.into()).await?;
            Downloader::download_complete_pieces(output_path, &mut peer, &torrent_file).await?;
            tracing::info!("Downloaded {} to {}", file_path, output_path);
        }
        _ => tracing::info!("unknown command: {}", args[1]),
    }

    Ok(())
}
