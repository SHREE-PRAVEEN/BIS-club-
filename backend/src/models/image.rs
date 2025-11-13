use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: i32,
    pub image_name: String,
    #[serde(skip_serializing)]
    pub image_data: Vec<u8>,
    pub content_type: String,
    pub file_size: i32,
    pub category: Option<String>,
    pub description: Option<String>,
    pub uploaded_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub(crate) im: (),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImageMetadata {
    pub id: i32,
    pub image_name: String,
    pub content_type: String,
    pub file_size: i32,
    pub category: Option<String>,
    pub description: Option<String>,
    pub uploaded_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadResponse {
    pub id: i32,
    pub image_name: String,
    pub file_size: i32,
    pub content_type: String,
    pub category: Option<String>,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageURL {
    pub id: i32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImageRequest {
    pub category: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateImageRequest {
    pub category: Option<String>,
    pub description: Option<String>,
}
