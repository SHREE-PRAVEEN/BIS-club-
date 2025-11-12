use actix_web::{web, HttpResponse, Error};

use crate::models::{Event, EventResponse, CreateEventRequest, UpdateEventRequest};
use crate::AppState;

// Create a new event
pub async fn create_event(
    state: web::Data<AppState>,
    req: web::Json<CreateEventRequest>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, Event>(
        "INSERT INTO events (title, description, event_type, event_date, start_time, end_time, location, is_published)
         VALUES ($1, $2, $3, $4, $5, $6, $7, false)
         RETURNING id, title, description, event_type, image_id, event_date, start_time, end_time, location, is_published, created_at, updated_at"
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.event_type)
    .bind(req.event_date)
    .bind(req.start_time)
    .bind(req.end_time)
    .bind(&req.location)
    .fetch_one(&state.db)
    .await
    {
        Ok(event) => {
            log::info!("✅ Event created: {}", event.title);
            Ok(HttpResponse::Created().json(event_to_response(&event)))
        }
        Err(e) => {
            log::error!("❌ Failed to create event: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to create event"})))
        }
    }
}

// Get all events
pub async fn list_events(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, Event>(
        "SELECT id, title, description, event_type, image_id, event_date, start_time, end_time, location, is_published, created_at, updated_at
         FROM events WHERE is_published = true ORDER BY event_date DESC NULLS LAST"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(events) => {
            let responses: Vec<EventResponse> = events
                .iter()
                .map(|e| event_to_response(e))
                .collect();
            
            log::info!("✅ Retrieved {} events", responses.len());
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "data": responses,
                "total": responses.len()
            })))
        }
        Err(e) => {
            log::error!("❌ Failed to fetch events: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to fetch events"})))
        }
    }
}

// Get single event by ID
pub async fn get_event(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let event_id = id.into_inner();

    match sqlx::query_as::<_, Event>(
        "SELECT id, title, description, event_type, image_id, event_date, start_time, end_time, location, is_published, created_at, updated_at
         FROM events WHERE id = $1"
    )
    .bind(event_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(event) => {
            log::info!("✅ Retrieved event: ID {}", event_id);
            Ok(HttpResponse::Ok().json(event_to_response(&event)))
        }
        Err(sqlx::Error::RowNotFound) => {
            log::warn!("⚠️  Event not found: ID {}", event_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Event not found"})))
        }
        Err(e) => {
            log::error!("❌ Database error: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Internal server error"})))
        }
    }
}

// Update event
pub async fn update_event(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    req: web::Json<UpdateEventRequest>,
) -> Result<HttpResponse, Error> {
    let event_id = id.into_inner();

    match sqlx::query_as::<_, Event>(
        "UPDATE events SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            event_type = COALESCE($3, event_type),
            image_id = COALESCE($4, image_id),
            event_date = COALESCE($5, event_date),
            start_time = COALESCE($6, start_time),
            end_time = COALESCE($7, end_time),
            location = COALESCE($8, location),
            is_published = COALESCE($9, is_published),
            updated_at = CURRENT_TIMESTAMP
         WHERE id = $10
         RETURNING id, title, description, event_type, image_id, event_date, start_time, end_time, location, is_published, created_at, updated_at"
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.event_type)
    .bind(req.image_id)
    .bind(req.event_date)
    .bind(req.start_time)
    .bind(req.end_time)
    .bind(&req.location)
    .bind(req.is_published)
    .bind(event_id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(event)) => {
            log::info!("✅ Event updated: ID {}", event_id);
            Ok(HttpResponse::Ok().json(event_to_response(&event)))
        }
        Ok(None) => {
            log::warn!("⚠️  Event not found: ID {}", event_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Event not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to update event: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to update event"})))
        }
    }
}

// Delete event
pub async fn delete_event(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let event_id = id.into_inner();

    match sqlx::query("DELETE FROM events WHERE id = $1")
        .bind(event_id)
        .execute(&state.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            log::info!("✅ Event deleted: ID {}", event_id);
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({
                    "success": true,
                    "message": "Event deleted successfully"
                })))
        }
        Ok(_) => {
            log::warn!("⚠️  Event not found: ID {}", event_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Event not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to delete event: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to delete event"})))
        }
    }
}

// Helper function to convert Event to EventResponse
fn event_to_response(event: &Event) -> EventResponse {
    EventResponse {
        id: event.id,
        title: event.title.clone(),
        description: event.description.clone(),
        event_type: event.event_type.clone(),
        image_url: event.image_id.map(|id| format!("/api/images/{}", id)),
        event_date: event.event_date,
        start_time: event.start_time,
        end_time: event.end_time,
        location: event.location.clone(),
        is_published: event.is_published,
        created_at: event.created_at,
    }
}
