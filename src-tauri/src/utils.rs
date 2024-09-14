use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    code: u16,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 创建成功的响应
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 200,
            message: None,
            data: Some(data),
        }
    }

    // 创建失败的响应
    // pub fn error(code: u16, message: &str) -> Self {
    //     ApiResponse {
    //         code,
    //         message: Some(message.to_string()),
    //         data: None,
    //     }
    // }
}
