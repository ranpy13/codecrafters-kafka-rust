#![allow(unused_imports)]

mod model;
mod utils;

use std::{io::{Read, Write}, net::TcpListener};
use anyhow::Result;

use bytes::{BufMut, BytesMut};

use crate::model::wire_protocol::{self, ApiVersionArray, Body, Header, Response};
use crate::utils::handler::handle_stream;
use log::{info, warn, error, debug};
use env_logger::Env;



fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092")?;
    info!("Listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("Accepted new connection");

                std::thread::spawn(move || {
                    if let Err(e) = handle_stream(&mut stream) {
                        error!("Connection error: {:?}", e);
                    }
                });
            }
            Err(e) => {
                error!("Accept error: {}", e);
            }
        }
    }

    Ok(())
}

