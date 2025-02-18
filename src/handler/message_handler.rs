use crate::model::message::{AppState, SendMessage};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing;

/// RESTful 接口：接收消息后，通过广播信道发送给所有已连接的 WebSocket 客户端
pub async fn send_message(
    State(state): State<AppState>,
    Json(payload): Json<SendMessage>,
) -> impl IntoResponse {
    let connections = state.connections.lock().await;
    
    if let Some(sender) = connections.get(&payload.client_id) {
        match sender.send(payload.message).await {
            Ok(_) => StatusCode::OK,
            Err(e) => {
                tracing::error!("Send error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    } else {
        StatusCode::NOT_FOUND
    }
}
