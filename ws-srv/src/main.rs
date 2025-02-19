use axum::extract::{ws, State};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use ws_srv::handler::auth::{extract_token, verify_token};
use ws_srv::handler::{message_handler, socket_handler};
use ws_srv::model::message::AppState;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState {
        connections: Arc::new(Mutex::new(HashMap::new())),
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
    headers: HeaderMap,
    ws: ws::WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let token = match extract_token(&headers) {
        Some(token) => token,
        None => {
            return StatusCode::FORBIDDEN.into_response()
        }
    };
    
    // 验证 token 并获取 user_id
    let user_id = match verify_token(&token) {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Invalid token: {:?}", token);
            return (StatusCode::FORBIDDEN, "Invalid token").into_response();
        }
    };

    ws.on_upgrade(move |socket| socket_handler::handle_socket(socket, state, user_id))
}
