//
// This is a simple echo server that would be replaced by your Python server
//

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::{self, net::TcpListener};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:1337").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut websocket = accept_async(stream).await.unwrap();

            while let Some(msg) = websocket.next().await {
                match msg {
                    Ok(msg) => match msg {
                        Message::Close(_) => break,
                        _ => websocket.send(msg).await.unwrap(),
                    },
                    Err(e) => {
                        if let Error::ConnectionClosed = e {
                            break;
                        }
                    }
                }
            }
        });
    }
}
