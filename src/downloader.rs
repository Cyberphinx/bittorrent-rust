use eyre::Result;
use sha1::{Digest, Sha1};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    peer_message::{Message, BLOCK_SIZE, MESSAGE},
    TorrentResponse,
};

pub struct Downloader;

impl Downloader {
    pub async fn download(
        output_path: &str,
        peer: &mut TcpStream,
        torrent: &TorrentResponse,
        piece: &i32,
    ) -> Result<()> {
        let pieces = Downloader::get_pieces(peer).await?;

        if pieces.contains(piece) {
            let loaded_piece = Downloader::load_piece(peer, piece, torrent).await?;

            let hash_from_file = Downloader::get_piece_hash(*piece, torrent);

            let mut hasher = Sha1::new();

            hasher.update(&loaded_piece);

            let real_hash: [u8; 20] = hasher.finalize().into();

            assert_eq!(hash_from_file, real_hash);

            let mut file = File::create(output_path).await.unwrap();

            file.write_all(loaded_piece.as_slice()).await.unwrap();
        }

        Ok(())
    }

    async fn get_pieces(peer: &mut TcpStream) -> Result<Vec<i32>> {
        let bitfield = Downloader::receive(peer).await?;

        assert_eq!(bitfield.id, MESSAGE::BITFIELD);

        let bitfield = bitfield.payload;
        let bitset: Vec<char> = bitfield
            .into_iter()
            .flat_map(|byte| format!("{:b}", byte).chars().collect::<Vec<char>>())
            .collect();

        let pieces = bitset
            .into_iter()
            .enumerate()
            .filter(|(_, b)| b.eq(&'1'))
            .map(|(i, _)| i as i32)
            .collect::<Vec<i32>>();

        Ok(pieces)
    }

    pub fn get_piece_hash(piece: i32, torrent: &TorrentResponse) -> [u8; 20] {
        let hashes: Vec<&[u8]> = torrent.info.pieces.chunks(20).collect();
        hashes[piece as usize].try_into().unwrap()
    }

    pub async fn load_piece(
        peer: &mut TcpStream,
        index: &i32,
        torrent: &TorrentResponse,
    ) -> Result<Vec<u8>> {
        Downloader::send(peer, Message::new(MESSAGE::INTERESTED, vec![])).await?;

        let unchoke = Downloader::receive(peer).await?;

        assert_eq!(unchoke.id, MESSAGE::UNCHOKE);

        // Get size of the piece
        let file_size = torrent.info.length as i32;
        let piece_len: i32 = torrent.info.piece_length as i32;
        let piece_size = piece_len.min(file_size - piece_len * index);

        println!("Piece size: {piece_size}");

        // Break the piece into blocks of 16 kiB (16 * 1024 bytes)
        // and send a request message for each block
        let mut piece: Vec<u8> = Vec::new();
        let mut remain = piece_size;
        let mut offset = 0;

        loop {
            if remain == 0 {
                // all blocks have been loaded
                break;
            }

            let block_size = BLOCK_SIZE.min(remain);

            let response = Downloader::load_block(peer, index, offset, block_size).await?;

            remain -= block_size;
            offset += block_size;

            println!("Remain {remain}");

            let block = &response.payload[8..];

            assert_eq!(response.prefix as usize - 9, block.len());

            piece.append(&mut block.to_vec());
        }

        Ok(piece)
    }

    async fn load_block(
        peer: &mut TcpStream,
        index: &i32,
        begin: i32,
        length: i32,
    ) -> Result<Message> {
        // request: <len=0013><id=6><index><begin><length>
        println!("Index: {index}");
        println!("Begin {begin}");
        println!("Length {length}");

        let payload: Vec<u8> = [
            index.to_be_bytes().as_slice(),
            begin.to_be_bytes().as_slice(),
            length.to_be_bytes().as_slice(),
        ]
        .concat();

        let request = Message::new(MESSAGE::REQUEST, payload);

        println!("Request {:?}", request);

        Downloader::send(peer, request).await?;

        let block = Downloader::receive(peer).await?;

        assert_eq!(block.id, MESSAGE::PIECE);

        Ok(block)
    }

    async fn send(peer: &mut TcpStream, message: Message) -> Result<()> {
        peer.write_all(message.to_bytes().as_slice()).await?;
        Ok(())
    }

    async fn receive(peer: &mut TcpStream) -> Result<Message> {
        let prefix = Downloader::read_prefix(peer).await?;
        let id = Downloader::read_message_id(peer).await?;
        let payload = Downloader::read_payload(peer, prefix).await?;

        Ok(Message {
            prefix,
            id,
            payload,
        })
    }

    async fn read_prefix(session: &mut TcpStream) -> Result<i32> {
        let mut buf = [0u8; 4];
        session.read_exact(&mut buf).await?;
        Ok(i32::from_be_bytes(buf))
    }

    async fn read_message_id(session: &mut TcpStream) -> Result<MESSAGE> {
        let mut buf = [0u8; 1];
        session.read_exact(&mut buf).await?;
        let id = MESSAGE::try_from(buf[0])?;
        Ok(id)
    }

    async fn read_payload(session: &mut TcpStream, prefix: i32) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; prefix as usize - 1];
        session.read_exact(&mut buf).await?;
        Ok(buf)
    }
}
