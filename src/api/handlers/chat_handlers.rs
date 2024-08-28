use axum::{extract::ws::WebSocketUpgrade, response::Response};

use crate::chat::services::handle_socket;

pub async fn chat(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}
