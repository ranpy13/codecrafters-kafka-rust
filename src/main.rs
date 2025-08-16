#![allow(unused_imports)]

use std::{io::{Read, Write}, net::TcpListener};
use anyhow::Result;

use bytes::{BufMut, BytesMut};

use crate::model::wire_protocol::{self, ApiVersionArray, Body, Header, Response};
use log::{info, warn, error, debug};
use env_logger::Env;

mod model;

fn main() -> Result<()>{
    // env_logger::init();
    env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    info!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    info!("listenting on remote address: {}", listener.local_addr()?);
    
    info!("starting listener!!");
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                info!("accepted new connection");
                // return WireProtocol::Response::new(8, "v0", "some random body");
                // let mut _stream = _stream;
                // _stream.write_all(&[0, 0, 0, 5, 0, 0, 0, 7]).unwrap();

                let mut request = [0u8; 39];
                _stream.read_exact(&mut request)?;

                debug!("Raw reqeust: {:?}", request);
                
                // let body: String = String::from("some random value");
                // let message_size = i32::from_be_bytes(request[0..4].try_into().unwrap());
                let request_api_key = i16::from_be_bytes(request[4..6].try_into().unwrap());
                let request_api_version= i16::from_be_bytes(request[6..8].try_into().unwrap());
                let correlation_id = i32::from_be_bytes(request[8..12].try_into().unwrap());
                
                let client_id: Option<String> = None;
                let tag_buffer: Vec<&str> = Vec::new();

                let error_code: i16 = if 0 < request_api_version && request_api_version >= 4 {35} else {0};

                let body = Body::new(
                    &error_code, 
                    &2, 
                    &ApiVersionArray::new(
                        &18,
                        &1,
                        &4, 
                        &0
                    ), 
                    &0i32, 
                    &0,
                );

                let message_size: i32 = body.to_bytes().len().try_into().unwrap(); 

                let header = Header {
                    request_api_key: request_api_key,
                    request_api_version: request_api_version,
                    correlation_id: correlation_id,
                    client_id: client_id,
                    tag_buffer: tag_buffer,
                };
                
                // let mut buf = BytesMut::with_capacity(192);
                
                // buf.put_i32(message_size);
                // buf.put_i32(correlation_id);

                // _stream.write_all(&buf)?;

                // let header2 = Header::new(&correlation_id)?;

                let response = Response {
                    message_size: message_size,
                    header: header,
                    body: body.clone(),
                };

                // let _response2 = Response::new(&message_size, &header2, &(body.clone()));

                let mut res = Vec::new();

                // res.extend_from_slice(&response.message_size.to_be_bytes());
                res.extend_from_slice(&response.header.correlation_id.to_be_bytes());

                info!("Message Size: {}", message_size);
                info!("Correlation id: {}", correlation_id);

                // res.extend_from_slice(&error_code.to_be_bytes());
                // res.extend_from_slice(&response.header.request_api_key.to_be_bytes());

                res.extend_from_slice(&response.body.error_code.to_be_bytes());
                res.extend_from_slice(&(response.body.array_length as u8).to_be_bytes());

                info!("Error Code: {}", error_code);
                info!("Array length: {}", body.array_length);
                
                res.extend_from_slice(&response.body.api_version_array.api_key.to_be_bytes());
                res.extend_from_slice(&response.body.api_version_array.min_version.to_be_bytes());
                res.extend_from_slice(&response.body.api_version_array.max_version.to_be_bytes());
                res.extend_from_slice(&response.body.api_version_array.tag_buffer.to_be_bytes());

                info!("Api Key: {}", body.api_version_array.api_key);
                info!("Api Min supported version: {}", body.api_version_array.min_version);
                info!("Api Max supported version: {}", body.api_version_array.max_version);
                info!("Api tag buffer: {}", body.api_version_array.tag_buffer);

                res.extend_from_slice(&response.body.throttle_time.to_be_bytes());
                res.extend_from_slice(&response.body.tag_buffer.to_be_bytes());

                info!("Throttle Time: {}", body.throttle_time);
                info!("Body tag buffer: {}", body.tag_buffer);
            

                // _stream.write(&res.len().to_be_bytes())?;
                let mut payload = Vec::new();

                payload.extend_from_slice(&(res.len() as i32).to_be_bytes());
                payload.extend_from_slice(&res);
                let _ = _stream.write_all(&res);
                // let _ = _stream.write_all(&response.to_bytes());

                debug!("Response bytes: {:?}", res);
            }
            Err(e) => {
                error!("error: {}", e);
            }
        }
    }

    Ok(())
}
