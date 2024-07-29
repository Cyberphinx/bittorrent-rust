use std::path::PathBuf;

use eyre::{eyre, ContextCompat, Result};
use sha1::{Digest, Sha1};

use crate::{decode::Decoder, Info, TorrentResponse};

pub struct Parser;
impl Parser {
    pub fn parse_torrent_file<T>(file_path: T) -> Result<TorrentResponse>
    where
        T: Into<PathBuf>,
    {
        let content = std::fs::read(file_path.into())?;
        let value: serde_bencode::value::Value = serde_bencode::from_bytes(content.as_slice())?;

        match value {
            serde_bencode::value::Value::Dict(d) => {
                let announce = Decoder::extract_string("announce", &d)?;
                let info = Decoder::extract_dict("info", &d)?;

                let info_hash = d.get(b"info".as_ref()).context("no info")?;
                let hash = hex::encode(Sha1::digest(serde_bencode::to_bytes(info_hash)?));

                Ok(TorrentResponse {
                    announce_url: announce,
                    info: Info {
                        length: Decoder::extract_int("length", &info)?,
                        name: Decoder::extract_string("name", &info)?,
                        piece_length: Decoder::extract_int("piece length", &info)?,
                        pieces: Decoder::extract_bytes("pieces", &info)?,
                    },
                    hash,
                })
            }
            _ => Err(eyre!("Incorrect format, required dict")),
        }
    }

    pub fn split_and_display_sha1_hashes(pieces: Vec<u8>) {
        // Ensure the length of pieces is a multiple of 20
        assert!(
            pieces.len() % 20 == 0,
            "The length of pieces must be a multiple of 20"
        );

        println!("Piece Hashes:");

        // Iterate over the pieces in chunks of 20 bytes
        for chunk in pieces.chunks(20) {
            // Convert the chunk into a hexadecimal string
            let hex_string = hex::encode(chunk);
            println!("{}", hex_string);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_file_torrent() {
        let file_path = "examples/example_file.torrent";
        let decoded_value = Parser::parse_torrent_file(file_path).unwrap();
        assert_eq!(decoded_value.announce_url, "http://example.com/announce");
        assert_eq!(decoded_value.info.length, 71);
        assert_eq!(
            decoded_value.hash,
            "18b1a85b911619d6872c902dc26958dc60287382"
        );
    }
}
