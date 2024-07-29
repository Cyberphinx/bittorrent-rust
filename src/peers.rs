pub struct Peer;
use std::{net::UdpSocket, time::Duration};

use eyre::{eyre, Context, ContextCompat, Result};
use reqwest::Client;
use serde_bencode::to_bytes;
use url::form_urlencoded;

use crate::{decode::Decoder, TrackerRequest};

impl Peer {
    pub async fn discover_peers(file_path: &str) -> Result<()> {
        let content = std::fs::read(file_path)?;
        let value: serde_bencode::value::Value = serde_bencode::from_bytes(content.as_slice())?;
        match value {
            serde_bencode::value::Value::Dict(d) => {
                let announce = Decoder::extract_string("announce", &d)?;
                let info = Decoder::extract_dict("info", &d)?;
                let info_hash = d.get(b"info".as_ref()).context("no info")?;

                let request = TrackerRequest {
                    peer_id: String::from("00112233445566778899"),
                    port: 6881,
                    uploaded: 0,
                    downloaded: 0,
                    left: Decoder::extract_int("piece length", &info)? as usize,
                    compact: 1,
                };

                let url_params = serde_urlencoded::to_string(&request)
                    .context("url-encode tracker parameters")?;

                let serialized_hash = to_bytes(info_hash)?;

                let bytes: &[u8] = &serialized_hash;

                if announce.starts_with("udp://") {
                    Peer::query_udp_tracker(&announce, &request, bytes).await?;
                } else {
                    Peer::query_http_tracker(&announce, &url_params, bytes).await?;
                }

                Ok(())
            }
            _ => Err(eyre!("Incorrect format, required dict")),
        }
    }

    async fn query_http_tracker(announce: &str, url_params: &str, info_hash: &[u8]) -> Result<()> {
        let tracker_url = format!(
            "{}?{}&info_hash={}",
            announce,
            url_params,
            &form_urlencoded::byte_serialize(info_hash).collect::<String>()
        );
        tracing::info!("Tracker url: {}", &tracker_url);

        let client = Client::new();
        let response = client
            .get(&tracker_url)
            .send()
            .await
            .context("query tracker")?;

        tracing::info!("Response: {:#?}", &response);

        let response_bytes = response.bytes().await.context("fetch tracker response")?;

        tracing::info!("Response bytes: {:#?}", &response_bytes);

        // let response: TrackerResponse =
        //     from_bytes(&response_bytes).context("parse tracker response")?;

        // println!("Interval: {}", response.interval);
        // for peer in &response.peers.0 {
        //     println!("{}:{}", peer.ip, peer.port);
        // }
        Ok(())
    }

    async fn query_udp_tracker(
        announce: &str,
        _request: &TrackerRequest,
        _info_hash: &[u8],
    ) -> Result<()> {
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

        Ok(())
    }

    pub async fn peer_handshake() {}
}
