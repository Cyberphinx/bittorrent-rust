use std::path::PathBuf;

use eyre::{eyre, Result};

use crate::{decode::Decoder, Info, Torrent};

pub struct Parser;
impl Parser {
    pub fn parse_torrent_file<T>(file_path: T) -> Result<Torrent>
    where
        T: Into<PathBuf>,
    {
        let content = std::fs::read(file_path.into())?;
        let value: serde_bencode::value::Value = serde_bencode::from_bytes(content.as_slice())?;

        match value {
            serde_bencode::value::Value::Dict(d) => {
                let announce = Decoder::extract_string("announce", &d)?;
                let info = Decoder::extract_dict("info", &d)?;

                Ok(Torrent {
                    announce_url: announce,
                    info: Info {
                        length: Decoder::extract_int("length", &info)?,
                        name: Decoder::extract_string("name", &info)?,
                        piece_length: Decoder::extract_int("piece length", &info)?,
                        pieces: Decoder::extract_bytes("pieces", &info)?,
                    },
                })
            }
            _ => Err(eyre!("Incorrect format, required dict")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_torrent() {
        let file_path = "examples/example_file.torrent";
        let decoded_value = Parser::parse_torrent_file(file_path).unwrap();
        assert_eq!(decoded_value.announce_url, "http://example.com/announce");
        assert_eq!(decoded_value.info.length, 71);
    }
}
