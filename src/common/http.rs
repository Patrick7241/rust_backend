use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 成功返回，带数据
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    /// 成功返回，不带数据
    pub fn success_without_data() -> Self {
        ApiResponse {
            code: 0,
            msg: "success".to_string(),
            data: None,
        }
    }

    /// 错误返回
    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        ApiResponse {
            code,
            msg: msg.into(),
            data: None,
        }
    }
}