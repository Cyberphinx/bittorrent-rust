use std::{
    fs::File,
    io::{Read, Write},
};

use eyre::Result;
use sha1::{Digest, Sha1};

use crate::{Info, RequestTorrent};

pub struct Encoder;
impl Encoder {
    pub fn encode_file(file_path: &str, announce_url: &str, piece_length: i64) -> Result<()> {
        // Read the file
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Calculate piece hashes
        let mut pieces = Vec::new();
        for chunk in buffer.chunks(piece_length as usize) {
            let mut hasher = Sha1::new();
            hasher.update(chunk);
            pieces.extend_from_slice(&hasher.finalize());
        }

        // Create the torrent info
        let info = Info {
            name: file_path.to_string(),
            length: buffer.len() as i64,
            piece_length,
            pieces,
        };

        // Create the torrent
        let torrent = RequestTorrent {
            info,
            announce_url: announce_url.to_string(),
        };

        // Serialize to bencode
        let bencoded = serde_bencode::to_bytes(&torrent).unwrap();

        // Write to file
        let mut output_file = File::create("examples/example_file.torrent")?;
        output_file.write_all(&bencoded)?;

        println!("Torrent file created successfully!");

        Ok(())
    }

    // pub fn encode_element(announce_url: &str, buffer: &mut Vec<u8>, piece_length: i64) {
    //     // Calculate piece hashes
    //     let mut pieces = Vec::new();
    //     for chunk in buffer.chunks(piece_length as usize) {
    //         let mut hasher = Sha1::new();
    //         hasher.update(chunk);
    //         pieces.extend_from_slice(&hasher.finalize());
    //     }

    //     // Create the torrent info
    //     let info = Info {
    //         name: file_path.to_string(),
    //         length: buffer.len() as i64,
    //         piece_length,
    //         pieces,
    //     };

    //     // Create the torrent
    //     let torrent = RequestTorrent {
    //         info,
    //         announce_url: announce_url.to_string(),
    //     };

    //     // Serialize to bencode
    //     let bencoded = serde_bencode::to_bytes(&torrent).unwrap();

    //     // Write to file
    //     let mut output_file = File::create("examples/example_file.torrent")?;
    //     output_file.write_all(&bencoded)?;

    //     println!("Torrent file created successfully!");

    //     Ok(())
    // }
}
