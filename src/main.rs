#![allow(unused_imports)]
use std::{io::{Write}, net::TcpListener};
use anyhow::Result;

use bytes::{BufMut, BytesMut};

use crate::model::wire_protocol;
use log::{info, warn, error};
use env_logger::Env;

mod model;

fn main() -> Result<()>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
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

                let message_size = 0i32;
                let correlation_id: i32 = 7i32;

                let mut buf = BytesMut::with_capacity(8);

                buf.put_i32(message_size);
                buf.put_i32(correlation_id);

                _stream.write_all(&buf)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
