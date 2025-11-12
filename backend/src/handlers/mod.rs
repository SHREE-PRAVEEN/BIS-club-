pub mod image_handler;
pub mod team_handler;
pub mod event_handler;
pub mod gallery_handler;
pub mod health_handler;

use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Health check
            .route("/health", web::get().to(health_handler::health_check))
            
            // Image routes
            .service(
                web::scope("/images")
                    .route("", web::post().to(image_handler::upload_image))
                    .route("", web::get().to(image_handler::list_images))
                    .route("/{id}", web::get().to(image_handler::get_image))
                    .route("/{id}", web::delete().to(image_handler::delete_image))
                    .route("/{id}", web::put().to(image_handler::update_image))
            )
            
            // Team member routes
            .service(
                web::scope("/team-members")
                    .route("", web::post().to(team_handler::create_team_member))
                    .route("", web::get().to(team_handler::list_team_members))
                    .route("/{id}", web::get().to(team_handler::get_team_member))
                    .route("/{id}", web::put().to(team_handler::update_team_member))
                    .route("/{id}", web::delete().to(team_handler::delete_team_member))
            )
            
            // Event routes
            .service(
                web::scope("/events")
                    .route("", web::post().to(event_handler::create_event))
                    .route("", web::get().to(event_handler::list_events))
                    .route("/{id}", web::get().to(event_handler::get_event))
                    .route("/{id}", web::put().to(event_handler::update_event))
                    .route("/{id}", web::delete().to(event_handler::delete_event))
            )
            
            // Gallery routes
            .service(
                web::scope("/gallery")
                    .route("", web::post().to(gallery_handler::create_gallery_item))
                    .route("", web::get().to(gallery_handler::list_gallery))
                    .route("/{id}", web::get().to(gallery_handler::get_gallery_item))
                    .route("/{id}", web::put().to(gallery_handler::update_gallery_item))
                    .route("/{id}", web::delete().to(gallery_handler::delete_gallery_item))
            )
    );
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error": "Not found",
        "message": "The requested resource does not exist"
    }))
}
