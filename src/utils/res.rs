use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub total: i64,
    pub page_num: i64,
    pub page_size: i64,
    pub list: Vec<T>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

pub fn success<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        code: 200,
        message: "Success".to_string(),
        data: Some(data),
    })
}

pub fn error(message: &str) -> Json<ApiResponse<()>> {
    Json(ApiResponse {
        code: 400,
        message: message.to_string(),
        data: None,
    })
}
