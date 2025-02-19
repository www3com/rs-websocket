use futures::{SinkExt, StreamExt};
use std::error::Error;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // WebSocket 服务器地址
    let url = Url::parse("ws://localhost:3000/ws")?;

    // 设置认证 token（这里使用示例 token，实际使用时需要替换为有效的 token）
    let token = "your_auth_token_here";

    // 构建请求头，添加 token
    let mut request = http::Request::builder()
        .uri(url.as_str())
        .header("Authorization", format!("Bearer {}", token))
        .body(())?;

    println!("正在连接到服务器...");

    // 连接到 WebSocket 服务器
    let (ws_stream, _) = connect_async(request).await?;
    println!("已成功连接到服务器");

    let (mut write, mut read) = ws_stream.split();

    // 启动发送消息的任务
    let send_task = tokio::spawn(async move {
        loop {
            // 这里可以实现发送消息的逻辑
            // 例如：从标准输入读取消息并发送
            let message = Message::Text(Utf8Bytes::from("Hello from client"));
            if let Err(e) = write.send(message).await {
                println!("发送消息失败: {}", e);
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    // 启动接收消息的任务
    let receive_task = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => match msg {
                    Message::Text(text) => println!("收到文本消息: {}", text),
                    Message::Binary(data) => println!("收到二进制消息: {} bytes", data.len()),
                    Message::Ping(_) => println!("收到 ping"),
                    Message::Pong(_) => println!("收到 pong"),
                    Message::Close(_) => {
                        println!("服务器关闭连接");
                        break;
                    }
                    Message::Frame(_) => println!("收到原始帧"),
                },
                Err(e) => {
                    println!("接收消息错误: {}", e);
                    break;
                }
            }
        }
    });

    // 等待任务完成
    tokio::try_join!(send_task, receive_task)?;

    Ok(())
}
