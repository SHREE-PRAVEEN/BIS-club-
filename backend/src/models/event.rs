use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub image_id: Option<i32>,
    pub event_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub location: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub image_url: Option<String>,
    pub event_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub location: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub event_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEventRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub image_id: Option<i32>,
    pub event_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub location: Option<String>,
    pub is_published: Option<bool>,
}
