use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GalleryItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_id: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: bool,
    pub gallery_category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GalleryItemResponse {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub display_order: Option<i32>,
    pub is_featured: bool,
    pub gallery_category: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGalleryItemRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub display_order: Option<i32>,
    pub gallery_category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGalleryItemRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_id: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub gallery_category: Option<String>,
}
