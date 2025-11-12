pub mod image;
pub mod team;
pub mod event;
pub mod gallery;

pub use image::*;
pub use team::*;
pub use event::*;
pub use gallery::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        SuccessResponse {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn with_message(data: T, message: String) -> Self {
        SuccessResponse {
            success: true,
            data: Some(data),
            message: Some(message),
        }
    }

    pub fn no_data() -> Self {
        SuccessResponse {
            success: true,
            data: None,
            message: None,
        }
    }
}
