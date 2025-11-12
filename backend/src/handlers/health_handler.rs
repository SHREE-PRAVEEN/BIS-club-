use actix_web::HttpResponse;
use serde_json::json;
use chrono::Utc;

pub async fn health_check() -> HttpResponse {
    log::info!("Health check requested");
    
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "service": "BIS Club Backend",
        "version": "0.1.0"
    }))
}
