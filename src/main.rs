mod model;
mod router;
mod handler;

use axum::{middleware, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use dotenvy::dotenv;
use tokio::sync::broadcast;
use crate::handler::{message_handler, socket_handler};
use crate::model::message::AppState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // 创建一个广播信道，用于向所有 WebSocket 客户端发送消息
    let (tx, _) = broadcast::channel::<String>(100);
    let state = AppState {
        connections: Arc::new(Mutex::new(HashMap::new()))
    };

    // 构建路由：
    // - GET /ws 升级为 WebSocket 连接
    // - POST /send 由 message_handler 模块处理
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/send", post(message_handler::send_message))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// WebSocket 升级处理函数，将连接交由 socket_handler 模块处理
async fn ws_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| socket_handler::handle_socket(socket, state))
}