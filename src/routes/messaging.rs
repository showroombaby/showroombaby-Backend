use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};

pub async fn messaging_handler() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Erreur de bind");
    println!("Listening on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during websocket handshake");
            let (mut write, mut read) = ws_stream.split();
            println!("Nouvelle connexion");

            while let Some(msg) = read.next().await {
                let msg = msg.expect("Error in message");
                if msg.is_text() || msg.is_binary() {
                    write.send(Message::Text("Hello from server".to_string())).await.expect("Erreur lors de l'envoie du message");
                }
            }
        });
    }
}
