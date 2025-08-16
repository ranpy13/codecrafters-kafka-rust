#![allow(unused_imports)]

use std::{io::{Read, Write}, net::TcpListener};
use anyhow::Result;

use bytes::{BufMut, BytesMut};

use crate::model::wire_protocol::{self, ApiVersionArray, Body, Header, Response};
use log::{info, warn, error, debug};
use env_logger::Env;

pub fn handle_stream(stream: &mut std::net::TcpStream) -> Result<()> {
    loop {
        let mut len_buf = [0u8; 4];

        // Read the message length
        if let Err(e) = stream.read_exact(&mut len_buf) {
            warn!("Stream closed or error: {:?}", e);
            break;
        }

        let msg_len = i32::from_be_bytes(len_buf);
        if msg_len <= 0 || msg_len > 10_000 {
            warn!("Invalid message length: {}", msg_len);
            break;
        }

        // Read the full request based on the length
        let mut request = vec![0u8; msg_len as usize];
        stream.read_exact(&mut request)?;

        debug!("Raw request: {:?}", request);

        // === PARSE REQUEST === //
        let request_api_key = i16::from_be_bytes(request[0..2].try_into()?);
        let request_api_version = i16::from_be_bytes(request[2..4].try_into()?);
        let correlation_id = i32::from_be_bytes(request[4..8].try_into()?);
        // NOTE: Skip client_id/tag_buffer parsing for now (if needed, adjust offsets)

        let error_code: i16 = if !(0..=4).contains(&request_api_version) {
            35
        } else {
            0
        };

        let body = Body::new(
            &error_code,
            &1,
            &ApiVersionArray::new(&18, &1, &4, &0),
            &0,
            &0,
        );

        let header = Header {
            request_api_key,
            request_api_version,
            correlation_id,
            client_id: None,
            tag_buffer: vec![],
        };

        let response = Response {
            message_size: 0, // unused in manual encoding
            header,
            body,
        };

        // === BUILD RESPONSE BYTES === //
        let mut res = Vec::new();
        res.extend_from_slice(&response.header.correlation_id.to_be_bytes());
        res.extend_from_slice(&response.body.error_code.to_be_bytes());
        res.push(response.body.array_length as u8);
        res.extend_from_slice(&response.body.api_version_array.api_key.to_be_bytes());
        res.extend_from_slice(&response.body.api_version_array.min_version.to_be_bytes());
        res.extend_from_slice(&response.body.api_version_array.max_version.to_be_bytes());
        res.push(response.body.api_version_array.tag_buffer);
        res.extend_from_slice(&response.body.throttle_time.to_be_bytes());
        res.push(response.body.tag_buffer);

        // Prepend the length prefix
        let mut payload = Vec::new();
        let len = res.len() as i32;
        payload.extend_from_slice(&len.to_be_bytes());
        payload.extend_from_slice(&res);

        stream.write_all(&payload)?;
        stream.flush()?;

        debug!("Sent response: {:?}", payload);
    }

    Ok(())
}
