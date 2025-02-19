use std::collections::HashMap;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing;
use crate::model::message::AppState;
use axum::extract::ws::{Message,  WebSocket};
use uuid::Uuid;

/// 处理 WebSocket 连接的逻辑
///
/// 该函数拆分了连接为发送端和接收端；同时订阅共享的广播信道，
/// 当 RESTful 接口发送消息时，会通过该信道接收并将消息发送给当前客户端。
pub async fn handle_socket(socket: WebSocket, state: AppState, user_id: u64) {
    let client_id = Uuid::new_v4().to_string();

    println!("New WebSocket connection: {}", client_id);

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // 创建两个消息通道：一个用于广播消息，一个用于 pong 消息
    let (broadcast_tx, mut broadcast_rx) = mpsc::channel::<String>(100);
    let (pong_tx, mut pong_rx) = mpsc::channel::<Vec<u8>>(100);

    // 将广播通道存储到 HashMap 中
    {
        let mut connections = state.connections.lock().await;
        connections
            .entry(user_id.clone())
            .or_insert_with(HashMap::new)
            .insert(client_id.clone(), broadcast_tx.clone());
    }

    // 统一的消息发送任务
    let send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = broadcast_rx.recv() => {
                    if ws_sender.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
                Some(payload) = pong_rx.recv() => {
                    if ws_sender.send(Message::Pong(payload.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // 主循环：处理客户端发来的消息
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                tracing::info!("Received text from client: {}", text);
                // 这里可以添加处理文本消息的逻辑
            }
            Ok(Message::Binary(_)) => {
                tracing::debug!("Received binary message, ignoring");
            }
            Ok(Message::Ping(payload)) => {
                if pong_tx.send(payload.into()).await.is_err() {
                    tracing::error!("Failed to send pong message");
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                tracing::debug!("Received pong message");
            }
            Ok(Message::Close(_)) => {
                tracing::info!("Client {} closed the connection", client_id);
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error for client {}: {}", client_id, e);
                break;
            }
        }
    }

    // 等待发送任务完成
    if let Err(e) = send_task.await {
        tracing::error!("Send task error for client {}: {}", client_id, e);
    }

    // 连接关闭时清理
    {
        let mut connections = state.connections.lock().await;
        if let Some(user_conns) = connections.get_mut(&user_id) {
            user_conns.remove(&client_id);
            // 如果用户没有其他连接了，则移除用户记录
            if user_conns.is_empty() {
                connections.remove(&user_id);
            }
        }
        tracing::info!("Removed client {} for user {} from connections", client_id, user_id);
    }
}