#![allow(unused_imports)]
use std::{io::{Write}, net::TcpListener};
use anyhow::Result;

use bytes::{BufMut, BytesMut};

use crate::model::wire_protocol::{self, Header, Response};
use log::{info, warn, error};
use env_logger::Env;

mod model;

fn main() -> Result<()>{
    // env_logger::init();
    env_logger::Builder::new()
    .filter_level(log::LevelFilter::Info)
    .init();

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    info!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    info!("listenting on remote address: {}", listener.local_addr()?);
    
    info!("starting listener!!");
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                // return WireProtocol::Response::new(8, "v0", "some random body");
                // let mut _stream = _stream;
                _stream.write_all(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();

                let message_size = 1i32;
                let correlation_id: i32 = 7i32;

                let mut buf = BytesMut::with_capacity(8);

                buf.put_i32(message_size);
                buf.put_i32(correlation_id);

                _stream.write_all(&buf)?;

                let header = Header {
                    correlation_id: correlation_id,
                };

                let header2 = Header::new(correlation_id)?;

                let response = Response {
                    message_size: message_size,
                    header: header,
                    body: String::new(),
                };

                let _response2 = Response::new(message_size, header2, String::new());

                let mut res = Vec::new();

                res.extend_from_slice(&response.message_size.to_be_bytes());
                res.extend_from_slice(&response.header.correlation_id.to_be_bytes());

                let _ = _stream.write_all(&res);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
