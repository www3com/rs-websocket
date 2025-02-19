use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use serde::Deserialize;

// 用于存储每个用户的所有客户端连接
pub type UserConnections = HashMap<String, mpsc::Sender<String>>;
pub type ConnectionMap = HashMap<u64, UserConnections>;

#[derive(Clone)]
pub struct AppState {
    pub connections: Arc<Mutex<ConnectionMap>>,
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub user_ids: Vec<u64>,
    pub message: String,
}