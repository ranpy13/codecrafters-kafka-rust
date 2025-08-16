#![allow(unused_imports)]

pub mod model;
pub mod utils;

use std::sync::Arc;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use crate::utils::handler::handle_connection;

use log::{info, warn, error, debug};
use env_logger::Env;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Logs from your program will appear here!");

    let listener = TcpListener::bind("localhost:9092").await.unwrap();


    // loop {
    //     let (socket, _addr) = listener.accept().await.expect("NOK pour accept");
    //     tokio::spawn(
    //         async move {
    //             process_client_socket(socket)
    //                 .await
    //             }
    //     );
    //     tokio::spawn({
    //         async move {
    //             handle_connection(socket).await
    //         }
    //     });
    // }
    //
    // async fn process_client_socket(mut socket: TcpStream) -> anyhow::Result<()> {
    //     socket
    //         .write(vec![0, 0, 0, 0, 0, 0, 0, 7].as_slice())
    //         .await?;
    //     Ok(())
    // }

    // Main loop
    loop {

        debug!("Spawning a thread with a new stream for each new connection");
        match listener.accept().await {
            Ok((stream, _)) => {
                info!("accepted new connection");
                // tokio::spawn({
                //     async move {
                //         handle_connection(stream).await
                //     }
                // });
                handle_connection(stream).await
            },

            Err(e) => {
                error!("error: {}", e);
            }
        }
    }
}