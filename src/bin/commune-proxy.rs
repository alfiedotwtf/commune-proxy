use clappers::Clappers;
use futures::{sink::SinkExt, stream::StreamExt};
use std::process::exit;
use tokio::{
    self,
    net::{TcpListener, TcpStream},
    select,
};
use tokio_tungstenite::{accept_async, client_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let clappers = Clappers::new()
        .set_singles(vec!["h|help", "l|listen", "u|upstream"])
        .build();

    if clappers.get_flag("help") {
        help();
    }

    if clappers.get_single("listen").is_empty() {
        eprintln!("Missing listen address");
        help();
    }

    if clappers.get_single("upstream").is_empty() {
        eprintln!("Missing upstream address");
        help();
    }

    let listen_address = clappers.get_single("listen");

    let listener = TcpListener::bind(listen_address.clone())
        .await
        .expect(format!("Failed to bind on {listen_address}").as_str());

    loop {
        let client_stream = match listener.accept().await {
            Ok((client_stream, _)) => client_stream,
            Err(_) => {
                // May want to log client connection error
                continue;
            }
        };

        let upstream_addr = clappers.get_single("upstream");

        tokio::spawn(async move {
            let mut client_websocket = accept_async(client_stream)
                .await
                .expect("Failed to accept client websocket");

            let upstream_stream = TcpStream::connect(upstream_addr.clone())
                .await
                .expect(format!("Failed to connect to upstream {upstream_addr}").as_str());

            let (mut upstream_websocket, _) =
                client_async(format!("ws://{upstream_addr}"), upstream_stream)
                    .await
                    .expect("Failed to connect to upstream websocket");

            loop {
                select! {
                    Some(client_msg) = client_websocket.next() => {
                        if let Ok(client_msg) = client_msg {
                            match client_msg {
                                Message::Close(_) => break,
                                _ => {
                                    upstream_websocket
                                        .send(client_msg)
                                        .await
                                        .expect("Failed to send message to upstream")
                                }
                            }
                        }
                    }
                    Some(upstream_msg) = upstream_websocket.next() => {
                        match upstream_msg {
                            Err(_) => {
                                // May want to log upstream read error
                                break
                            }
                            Ok(upstream_msg) => match upstream_msg {
                                Message::Close(_) => {
                                    // May want to log upstream closing
                                    break
                                }
                                _ => {
                                    client_websocket
                                        .send(upstream_msg)
                                        .await
                                        .unwrap()
                                }
                            },
                        }
                    },
                    else => break,
                }
            }
        });
    }
}

fn help() {
    eprintln!(
        "
commune-proxy

Usage:
    commune-proxy [options]

    Options:
        -h|help                Display this message
        -l|listen <ip:port>    Address and port to listen on
        -u|upstream <ip:port>  Address and port of the upstream Commune server"
    );

    exit(1);
}
