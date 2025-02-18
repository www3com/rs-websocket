use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
#[derive(Serialize)]
pub struct R<T: Serialize> {
    pub code: u16,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> R<T> {
    #[allow(dead_code)]
    pub fn new(code: u16, msg: impl Into<String>, data: T) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: Some(data),
        }
    }
    pub fn ok() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: "".into(),
            data: None,
        }
    }
    pub fn ok_with(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: "".into(),
            data: Some(data),
        }
    }

    #[allow(dead_code)]
    pub fn ok_with_msg(msg: impl Into<String>) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: msg.into(),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn ok_with_msg_data(msg: impl Into<String>, data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: msg.into(),
            data: Some(data),
        }
    }

    pub fn err(code: u16, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: None, // 直接使用 None，避免反序列化问题
        }
    }
}

impl<T: Serialize> IntoResponse for R<T> {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        let code = StatusCode::from_u16(self.code).unwrap();
        match code {
            StatusCode::OK => (StatusCode::OK, body).into_response(),
            StatusCode::NOT_FOUND => (StatusCode::NOT_FOUND, body).into_response(),
            StatusCode::INTERNAL_SERVER_ERROR => {
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            StatusCode::BAD_REQUEST => (StatusCode::BAD_REQUEST, body).into_response(),
            StatusCode::UNAUTHORIZED => (StatusCode::UNAUTHORIZED, body).into_response(),
            _ => (StatusCode::from_u16(self.code).unwrap(), body).into_response(),
        }
    }
}
