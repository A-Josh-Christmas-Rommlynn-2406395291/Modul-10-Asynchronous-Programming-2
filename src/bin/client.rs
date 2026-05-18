use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

const WS_URI: &str = "ws://127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        match ClientBuilder::from_uri(Uri::from_static(WS_URI))
            .connect()
            .await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to connect to ws://127.0.0.1:8080: {e}");
                return Ok(());
            }
        };

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    loop {
        tokio::select! {
            // Read a line from stdin and send it to the server
            line = stdin.next_line() => {
                match line {
                    Ok(Some(l)) => {
                        if let Err(e) = ws_stream.send(Message::text(l)).await {
                            eprintln!("failed to send websocket message: {e}");
                            break;
                        }
                    }
                    Ok(None) => {
                        // EOF on stdin
                        break;
                    }
                    Err(e) => {
                        eprintln!("stdin error: {e}");
                        break;
                    }
                }
            }

            // Receive a message from the server and display it
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            if let Some(txt) = msg.as_text() {
                                println!("[Josh's Computer - From another client]: {}", txt);
                            }
                        } else if msg.is_binary() {
                            println!("<binary {} bytes>", msg.as_payload().len());
                        }
                        // ignore other message types
                    }
                    Some(Err(e)) => {
                        eprintln!("websocket receive error: {e}");
                        break;
                    }
                    None => {
                        // connection closed
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}