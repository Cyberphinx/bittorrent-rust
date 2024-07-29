use std::{collections::HashMap, path::PathBuf};

use eyre::{eyre, ContextCompat, Result};
use serde_bencode::to_bytes;
use sha1::{Digest, Sha1};

use crate::{decode::Decoder, Info, TorrentResponse};

pub struct Parser;
impl Parser {
    pub fn read_torrent_file<T>(
        file_path: T,
    ) -> Result<HashMap<Vec<u8>, serde_bencode::value::Value>>
    where
        T: Into<PathBuf>,
    {
        let content = std::fs::read(file_path.into())?;
        let value: serde_bencode::value::Value = serde_bencode::from_bytes(content.as_slice())?;

        match value {
            serde_bencode::value::Value::Dict(d) => Ok(d),
            _ => Err(eyre!("Incorrect format, required dict")),
        }
    }

    pub fn parse_torrent_file(
        dictionary: HashMap<Vec<u8>, serde_bencode::value::Value>,
    ) -> Result<TorrentResponse> {
        let announce = Decoder::extract_string("announce", &dictionary)?;
        let info = Decoder::extract_dict("info", &dictionary)?;

        let info_hash = dictionary.get(b"info".as_ref()).context("no info")?;
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

    pub fn get_info_hash_array(info_hash: &serde_bencode::value::Value) -> Result<[u8; 20]> {
        let serialized_hash = to_bytes(&info_hash)?;
        let sha1_hash = Sha1::digest(serialized_hash);
        let info_hash_array: [u8; 20] = sha1_hash
            .as_slice()
            .try_into()
            .map_err(|_| eyre!("Hash length mismatch"))?;
        Ok(info_hash_array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_file_torrent() {
        let file_path = "sample.torrent";
        let torrent_dict = Parser::read_torrent_file(file_path).unwrap();
        let decoded_value = Parser::parse_torrent_file(torrent_dict).unwrap();
        assert_eq!(
            decoded_value.announce_url,
            "http://bittorrent-test-tracker.codecrafters.io/announce"
        );
        assert_eq!(decoded_value.info.length, 92063);
        assert_eq!(
            decoded_value.hash,
            "d69f91e6b2ae4c542468d1073a71d4ea13879a7f"
        );
    }
}
