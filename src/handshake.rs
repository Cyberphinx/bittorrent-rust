use crate::{parse::Parser, peers::Peer};
use eyre::{Context, ContextCompat, Result};
use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[repr(C)]
pub struct Handshake {
    pub length: u8,
    pub bittorrent: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            length: 19,
            bittorrent: *b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let bytes = self as *mut Self as *mut [u8; std::mem::size_of::<Self>()];
        // Safety: Handshake is a POD with repr(c)
        let bytes: &mut [u8; std::mem::size_of::<Self>()] = unsafe { &mut *bytes };
        bytes
    }

    pub async fn peer_handshake(
        dictionary: &HashMap<Vec<u8>, serde_bencode::value::Value>,
        peer: Peer,
    ) -> Result<(TcpStream, Handshake)> {
        let info_hash_value = dictionary.get(b"info".as_ref()).context("no info")?;
        let info_hash = Parser::get_info_hash_array(info_hash_value)?;

        tracing::info!("ip: {}, port: {}", peer.0.ip(), peer.0.port());

        let mut peer = tokio::net::TcpStream::connect(peer.0)
            .await
            .context("connect to peer")?;

        let mut handshake = Handshake::new(info_hash, *b"00112233445566778899");

        {
            let handshake_bytes =
                &mut handshake as *mut Handshake as *mut [u8; std::mem::size_of::<Handshake>()];

            // Safety: Handshake is a POD (Plain Old Data) with repr(c)
            let handshake_bytes: &mut [u8; std::mem::size_of::<Handshake>()] =
                unsafe { &mut *handshake_bytes };

            peer.write_all(handshake_bytes)
                .await
                .context("write handshake")?;

            peer.read_exact(handshake_bytes)
                .await
                .context("read handshake")?;
        }

        assert_eq!(handshake.length, 19);

        assert_eq!(&handshake.bittorrent, b"BitTorrent protocol");

        println!("Peer ID: {}", hex::encode(handshake.peer_id));

        Ok((peer, handshake))
    }

    pub async fn get_handshake<'a>(
        peer: &mut TcpStream,
        handshake: &'a mut Handshake,
    ) -> Result<&'a mut Handshake> {
        {
            let handshake_bytes =
                handshake as *mut Handshake as *mut [u8; std::mem::size_of::<Handshake>()];

            // Safety: Handshake is a POD with repr(c)

            let handshake_bytes: &mut [u8; std::mem::size_of::<Handshake>()] =
                unsafe { &mut *handshake_bytes };

            peer.write_all(handshake_bytes)
                .await
                .context("write handshake")?;

            peer.read_exact(handshake_bytes)
                .await
                .context("read handshake")?;
        }
        Ok(handshake)
    }
}
