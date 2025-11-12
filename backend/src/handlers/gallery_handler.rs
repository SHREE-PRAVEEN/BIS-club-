use actix_web::{web, HttpResponse, Error};

use crate::models::{GalleryItem, GalleryItemResponse, CreateGalleryItemRequest, UpdateGalleryItemRequest};
use crate::AppState;

// Create a new gallery item
pub async fn create_gallery_item(
    state: web::Data<AppState>,
    req: web::Json<CreateGalleryItemRequest>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, GalleryItem>(
        "INSERT INTO gallery (title, description, display_order, gallery_category, is_featured)
         VALUES ($1, $2, $3, $4, false)
         RETURNING id, title, description, image_id, display_order, is_featured, gallery_category, created_at, updated_at"
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(req.display_order)
    .bind(&req.gallery_category)
    .fetch_one(&state.db)
    .await
    {
        Ok(item) => {
            log::info!("✅ Gallery item created: {:?}", item.title);
            Ok(HttpResponse::Created().json(gallery_to_response(&item)))
        }
        Err(e) => {
            log::error!("❌ Failed to create gallery item: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to create gallery item"})))
        }
    }
}

// Get all gallery items
pub async fn list_gallery(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, GalleryItem>(
        "SELECT id, title, description, image_id, display_order, is_featured, gallery_category, created_at, updated_at
         FROM gallery ORDER BY display_order ASC NULLS LAST"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(items) => {
            let responses: Vec<GalleryItemResponse> = items
                .iter()
                .map(|item| gallery_to_response(item))
                .collect();
            
            log::info!("✅ Retrieved {} gallery items", responses.len());
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "data": responses,
                "total": responses.len()
            })))
        }
        Err(e) => {
            log::error!("❌ Failed to fetch gallery items: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to fetch gallery items"})))
        }
    }
}

// Get single gallery item by ID
pub async fn get_gallery_item(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item_id = id.into_inner();

    match sqlx::query_as::<_, GalleryItem>(
        "SELECT id, title, description, image_id, display_order, is_featured, gallery_category, created_at, updated_at
         FROM gallery WHERE id = $1"
    )
    .bind(item_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(item) => {
            log::info!("✅ Retrieved gallery item: ID {}", item_id);
            Ok(HttpResponse::Ok().json(gallery_to_response(&item)))
        }
        Err(sqlx::Error::RowNotFound) => {
            log::warn!("⚠️  Gallery item not found: ID {}", item_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Gallery item not found"})))
        }
        Err(e) => {
            log::error!("❌ Database error: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Internal server error"})))
        }
    }
}

// Update gallery item
pub async fn update_gallery_item(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    req: web::Json<UpdateGalleryItemRequest>,
) -> Result<HttpResponse, Error> {
    let item_id = id.into_inner();

    match sqlx::query_as::<_, GalleryItem>(
        "UPDATE gallery SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            image_id = COALESCE($3, image_id),
            display_order = COALESCE($4, display_order),
            is_featured = COALESCE($5, is_featured),
            gallery_category = COALESCE($6, gallery_category),
            updated_at = CURRENT_TIMESTAMP
         WHERE id = $7
         RETURNING id, title, description, image_id, display_order, is_featured, gallery_category, created_at, updated_at"
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(req.image_id)
    .bind(req.display_order)
    .bind(req.is_featured)
    .bind(&req.gallery_category)
    .bind(item_id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(item)) => {
            log::info!("✅ Gallery item updated: ID {}", item_id);
            Ok(HttpResponse::Ok().json(gallery_to_response(&item)))
        }
        Ok(None) => {
            log::warn!("⚠️  Gallery item not found: ID {}", item_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Gallery item not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to update gallery item: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to update gallery item"})))
        }
    }
}

// Delete gallery item
pub async fn delete_gallery_item(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item_id = id.into_inner();

    match sqlx::query("DELETE FROM gallery WHERE id = $1")
        .bind(item_id)
        .execute(&state.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            log::info!("✅ Gallery item deleted: ID {}", item_id);
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({
                    "success": true,
                    "message": "Gallery item deleted successfully"
                })))
        }
        Ok(_) => {
            log::warn!("⚠️  Gallery item not found: ID {}", item_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Gallery item not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to delete gallery item: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to delete gallery item"})))
        }
    }
}

// Helper function to convert GalleryItem to GalleryItemResponse
fn gallery_to_response(item: &GalleryItem) -> GalleryItemResponse {
    GalleryItemResponse {
        id: item.id,
        title: item.title.clone(),
        description: item.description.clone(),
        image_url: item.image_id.map(|id| format!("/api/images/{}", id)),
        display_order: item.display_order,
        is_featured: item.is_featured,
        gallery_category: item.gallery_category.clone(),
        created_at: item.created_at,
    }
}
