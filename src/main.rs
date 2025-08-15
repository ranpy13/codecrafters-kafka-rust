#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

use crate::model::WireProtocol;

mod model;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                // return WireProtocol::Response::new(8, "v0", "some random body");
                let mut _stream = _stream;
                _stream.write_all(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
