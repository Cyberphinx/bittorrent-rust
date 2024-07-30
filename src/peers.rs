use core::fmt;
use std::{
    collections::HashMap,
    net::{SocketAddrV4, UdpSocket},
    str::FromStr,
    time::Duration,
};

use eyre::{eyre, Context, ContextCompat, Result};
use reqwest::Client;
use url::form_urlencoded;

use crate::{decode::Decoder, parse::Parser, TrackerRequest, TrackerResponse};

pub struct Peer(pub SocketAddrV4);

// Implement the FromStr trait for Peer
impl FromStr for Peer {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the string into an Ipv4Addr
        let addr = SocketAddrV4::from_str(s)?;
        // Wrap the Ipv4Addr in a Peer and return it
        Ok(Peer(addr))
    }
}

// Implement Display trait for Peer to enable easy printing
impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Peer {
    pub async fn discover_peers(
        dictionary: &HashMap<Vec<u8>, serde_bencode::value::Value>,
    ) -> Result<TrackerResponse> {
        let announce = Decoder::extract_string("announce", dictionary)?;
        let info = Decoder::extract_dict("info", dictionary)?;

        // Extract the info hash into &[u8] from the dictionary
        let info_hash_value = dictionary.get(b"info".as_ref()).context("no info")?;
        let info_hash = Parser::get_info_hash_array(info_hash_value)?;

        // Compose the tracker request object
        let request = TrackerRequest {
            peer_id: String::from("00112233445566778899"),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: Decoder::extract_int("piece length", &info)? as usize,
            compact: 1,
        };

        let url_params =
            serde_urlencoded::to_string(&request).context("url-encode tracker parameters")?;

        if announce.starts_with("udp://") {
            // UDP protocol
            let response = Peer::query_udp_tracker(&announce, &request, &info_hash).await?;
            Ok(response)
        } else {
            // HTTP or HTTPS protocols
            let response = Peer::query_http_tracker(&announce, &url_params, &info_hash).await?;
            Ok(response)
        }
    }

    async fn query_http_tracker(
        announce: &str,
        url_params: &str,
        info_hash: &[u8; 20],
    ) -> Result<TrackerResponse> {
        // URL-encode the byte array
        let url_encoded_info_hash: String = form_urlencoded::byte_serialize(info_hash).collect();

        let tracker_url = format!(
            "{}?{}&info_hash={}",
            announce, url_params, url_encoded_info_hash
        );

        let client = Client::new();
        let response = client
            .get(&tracker_url)
            .send()
            .await
            .context("query tracker")?;

        let response_bytes = response.bytes().await.context("fetch tracker response")?;

        let response: TrackerResponse =
            serde_bencode::from_bytes(&response_bytes).context("parse tracker response")?;

        println!("Interval: {}", response.interval);
        for peer in &response.peers.0 {
            println!("{}:{}", peer.ip(), peer.port());
        }
        Ok(response)
    }

    async fn query_udp_tracker(
        announce: &str,
        _request: &TrackerRequest,
        _info_hash: &[u8; 20],
    ) -> Result<TrackerResponse> {
        let announce = announce.trim_start_matches("udp://");
        let parts: Vec<&str> = announce.split(':').collect();
        if parts.len() != 2 {
            return Err(eyre!("Invalid UDP announce URL"));
        }
        let host = parts[0];
        let port: u16 = parts[1]
            .parse()
            .context("Invalid port in UDP announce URL")?;

        tracing::info!("Host: {}, Port: {}", host, port);

        let socket = UdpSocket::bind("0.0.0.0:0").context("bind UDP socket")?;

        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .context("set read timeout")?;

        // Implement the UDP tracker protocol here
        // 1. Send connection request
        // 2. Receive connection response
        // 3. Send announce request
        // 4. Receive announce response

        // Example: Send a connection request
        let connection_request = [0u8; 16]; // Placeholder for actual connection request
        socket
            .send_to(&connection_request, (host, port))
            .context("send connection request")?;

        let mut buf = [0u8; 1024];
        let (_amt, _src) = socket
            .recv_from(&mut buf)
            .context("receive connection response")?;

        // Parse the connection response and proceed with announce request

        todo!()
    }
}
