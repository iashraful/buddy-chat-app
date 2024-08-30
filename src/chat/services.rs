use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use futures::StreamExt;

pub async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            println!("Message from client: {}", msg.clone().into_text().unwrap());
            msg
        } else {
            // client disconnected
            return;
        };

        if sender.send(Message::from("Test")).await.is_err() {
            // client disconnected
            return;
        }
    }
}
