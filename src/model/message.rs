use std::collections::HashMap;
use tokio::sync::mpsc;
use serde::{Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub connections: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub client_id: String,  // 添加客户端 ID
    pub message: String,
}