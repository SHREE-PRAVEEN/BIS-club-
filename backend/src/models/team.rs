use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TeamMember {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_id: Option<i32>,
    pub display_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMemberResponse {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_url: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamMemberRequest {
    pub name: String,
    pub position: String,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamMemberRequest {
    pub name: Option<String>,
    pub position: Option<String>,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub image_id: Option<i32>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}
