use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

const LISTEN_ADDR: &str = "127.0.0.1:8080";

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // Subscribe to the broadcast channel to receive messages from other clients.
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Receive a message from the websocket client
            maybe_msg = ws_stream.next() => {
                match maybe_msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            if let Some(txt) = msg.as_text() {
                                let to_send = format!("{}: {}", addr, txt);
                                let _ = bcast_tx.send(to_send);
                            }
                        } else if msg.is_binary() {
                            let len = msg.as_payload().len();
                            let _ = bcast_tx.send(format!("{}: <binary {} bytes>", addr, len));
                        } else if msg.is_ping() || msg.is_pong() || msg.is_close() {
                            // ignore control messages
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Websocket error from {addr}: {e}");
                        break;
                    }
                    None => {
                        // client closed connection
                        break;
                    }
                }
            }

            // Receive a message from the broadcast channel and forward to this client
            result = bcast_rx.recv() => {
                match result {
                    Ok(text) => {
                        // Avoid echoing messages back to the original sender
                        if !text.starts_with(&format!("{}:", addr)) {
                            if let Err(e) = ws_stream.send(Message::text(text)).await {
                                eprintln!("failed to send to {addr}: {e}");
                                break;
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        // On lag we continue; messages were missed
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind(LISTEN_ADDR).await?;
    println!("listening on {LISTEN_ADDR}");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}