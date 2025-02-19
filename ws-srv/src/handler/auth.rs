use axum::http::HeaderMap;

pub fn verify_token(token: &str) -> Result<u64, &'static str> {
    // 验证 token 并返回 user_id
    // 这里应该实现实际的 JWT 验证逻辑
    if token.starts_with("Bearer ") {
        // 示例：从 token 中提取 user_id
        Ok(101) // 实际应该从 JWT 中解析出真实的 user_id
    } else {
        Err("Invalid token format")
    }
}

pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers.get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}