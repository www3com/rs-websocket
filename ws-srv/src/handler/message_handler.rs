use crate::model::message::{AppState, SendMessage};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum::http::Response;
use tracing;
use crate::model::r::R;

/// RESTful 接口：接收消息后，通过广播信道发送给所有已连接的 WebSocket 客户端
pub async fn send_message(
    State(state): State<AppState>,
    Json(payload): Json<SendMessage>,
) -> R<String> {
    let connections = state.connections.lock().await;

    let mut success_count = 0;
    let total_users = payload.user_ids.len();

    // 遍历所有目标用户
    for user_id in &payload.user_ids {
        if let Some(user_conns) = connections.get(user_id) {
            // 向该用户的所有客户端发送消息
            for (client_id, tx) in user_conns {
                if let Err(e) = tx.send(payload.message.clone()).await {
                    tracing::error!("Failed to send message to client {}: {}", client_id, e);
                } else {
                    success_count += 1;
                }
            }
        } else {
            tracing::warn!("User {} not found", user_id);
        }
    }

    if success_count == total_users {
        return R::ok_with("Message sent to all users".to_string());
    }else { return R::err(555, format!(
        "Message sent to {}/{} users",
        success_count, total_users
    )) }

}
