#![allow(unused_imports)]

use std::{io::{Read, Write}, net::TcpListener};
use anyhow::Result;

use bytes::{BufMut, BytesMut};

use crate::model::wire_protocol::{self, ApiVersionArray, Body, Header, Response};
use log::{info, warn, error, debug};
use env_logger::Env;

pub async fn handle_stream(stream: &mut std::net::TcpStream) -> Result<()> {
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

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::model::request::request_api_versions;
use crate::model::structs::{Builder, KafkaError, RespMessage, Streamable};
use crate::model::structs::KafkaError::StringError;

pub async fn handle_connection(mut stream: TcpStream) {
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    //let mut cursor = 0;

    // Read first 4 bytes (request_api_key + request_api_version)
    //stream.read_buf(&mut buffer).await.expect("Reading the request, parsing it now..");



    while let Ok(read_len) = stream.read_buf(&mut buffer).await {
        info!("buffer = {:02x?} \t read_len = {:?}", buffer, read_len);

        let (int_bytes, rest) = buffer.split_at(size_of::<i32>());
        let req_size = i32::from_be_bytes(int_bytes.try_into().unwrap());

        let (int_bytes, rest) = rest.split_at(size_of::<i16>());
        let request_api_key = i16::from_be_bytes(int_bytes.try_into().unwrap());

        let (int_bytes, rest) = rest.split_at(size_of::<i16>());
        let request_api_version = i16::from_be_bytes(int_bytes.try_into().unwrap());

        let (int_bytes, rest) = rest.split_at(size_of::<i32>());
        let correlation_id = i32::from_be_bytes(int_bytes.try_into().unwrap());

        let _body = if rest[0] != 0 {
            let temp_rest = rest.to_vec();
            let (int_bytes, inner_rest) = temp_rest.split_at(size_of::<i16>());
            let string_len = i16::from_be_bytes(int_bytes.try_into().unwrap());
            let body_string = String::from_utf8(inner_rest.to_vec());
            info!("read ({:?}) => {:?}", string_len, body_string);
            body_string.ok()
        } else { None };

        let body_vec = if rest[0] == 0 { None } else { Some(rest.to_vec()) };

        info!("req_size = {:?}", req_size);
        info!("request_api_key = {:?}", request_api_key);
        info!("request_api_version = {:?}", request_api_version);
        info!("correlation_id = {:?}", correlation_id);

        // let mut new_resp = RespMessage::new();
        // new_resp.change_correlation_id(correlation_id);
        //
        // // Calcul du message_size
        // let mut message_size = 4;
        // if new_resp.has_body() {
        //     message_size += new_resp.body_len() as i32;
        // }
        // new_resp.change_message_size(message_size);

        if let Ok(resp) = process_request(request_api_key, request_api_version, correlation_id, body_vec).await {
            let resp_bytes = resp.to_hex();

            debug!("resp_message = {:?}", resp);
            debug!("bytes = {:02x?}", resp_bytes);

            stream.write_all(resp_bytes.as_slice())
                .await
                .expect("Error writing in the provided TcpStream");
        }

        buffer.clear();
    }


}

pub async fn process_request(request_api_key: i16, request_api_version: i16, correlation_id: i32, _request: Option<Vec<u8>>) -> Result<RespMessage, KafkaError> {
    match request_api_key {
        18 => {
            request_api_versions(request_api_version, correlation_id).await
        }
        _ => {
            Err(StringError("Error matching the request_api_key in fn process_request()".to_string()))
        }
    }
}