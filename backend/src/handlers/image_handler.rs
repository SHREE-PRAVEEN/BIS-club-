use actix_web::{web, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::{Image, ImageMetadata, ImageUploadResponse, UpdateImageRequest};
use crate::AppState;
use crate::utils;

// Upload image with multipart form data
pub async fn upload_image(
    state: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut image_data = Vec::new();
    let mut file_name = String::new();
    let mut content_type = String::from("application/octet-stream");
    let mut category: Option<String> = None;
    let mut description: Option<String> = None;

    // Process multipart form
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::MultipartError(e.to_string()))?;
        let field_name = field.name().to_string();

        if field_name == "file" {
            // Extract filename
            let content_disposition = field.content_disposition();
            if let Some(filename) = content_disposition.get_filename() {
                file_name = filename.to_string();
            }

            // Get content type
            if let Some(ct) = field.content_type() {
                content_type = ct.to_string();
            }

            // Read file data
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| AppError::MultipartError(e.to_string()))?;
                image_data.extend_from_slice(&data);

                if image_data.len() > state.config.max_file_size {
                    return Ok(HttpResponse::PayloadTooLarge()
                        .json(serde_json::json!({
                            "error": "File size exceeds maximum allowed size",
                            "max_size": state.config.max_file_size
                        })));
                }
            }
        } else if field_name == "category" {
            let mut cat_data = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| AppError::MultipartError(e.to_string()))?;
                cat_data.extend_from_slice(&data);
            }
            category = Some(String::from_utf8_lossy(&cat_data).to_string());
        } else if field_name == "description" {
            let mut desc_data = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| AppError::MultipartError(e.to_string()))?;
                desc_data.extend_from_slice(&data);
            }
            description = Some(String::from_utf8_lossy(&desc_data).to_string());
        }
    }

    if image_data.is_empty() {
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "No file uploaded"})));
    }

    if !utils::is_valid_image_type(&content_type) {
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "Invalid file type. Only images are allowed"})));
    }

    let file_size = image_data.len() as i32;

    // Insert into database
    match sqlx::query_as::<_, ImageMetadata>(
        "INSERT INTO images (image_name, image_data, content_type, file_size, category, description)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, image_name, content_type, file_size, category, description, uploaded_at, updated_at"
    )
    .bind(&file_name)
    .bind(&image_data)
    .bind(&content_type)
    .bind(file_size)
    .bind(&category)
    .bind(&description)
    .fetch_one(&state.db)
    .await
    {
        Ok(image) => {
            log::info!("✅ Image uploaded: {} (ID: {})", file_name, image.id);
            Ok(HttpResponse::Ok().json(ImageUploadResponse {
                id: image.id,
                image_name: image.image_name,
                file_size: image.file_size,
                content_type: image.content_type,
                category: image.category,
                uploaded_at: image.uploaded_at,
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to insert image: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to upload image"})))
        }
    }
}

// Get single image by ID (returns binary data)
pub async fn get_image(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let image_id = id.into_inner();

    match sqlx::query_as::<_, Image>(
        "SELECT id, image_name, image_data, content_type, file_size, category, description, uploaded_at, updated_at
         FROM images WHERE id = $1"
    )
    .bind(image_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(image) => {
            log::info!("✅ Image retrieved: ID {}", image_id);
            Ok(HttpResponse::Ok()
                .content_type(&image.content_type)
                .insert_header(("Content-Disposition".to_string(), format!("inline; filename=\"{}\"", image.image_name)))
                .body(image.image_data))
        }
        Err(sqlx::Error::RowNotFound) => {
            log::warn!("⚠️  Image not found: ID {}", image_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Image not found"})))
        }
        Err(e) => {
            log::error!("❌ Database error: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Internal server error"})))
        }
    }
}

// List all images with optional filtering
pub async fn list_images(
    state: web::Data<AppState>,
    query: web::Query<ListImagesQuery>,
) -> Result<HttpResponse, Error> {
    let category = query.category.clone();
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let result = if let Some(cat) = category {
        sqlx::query_as::<_, ImageMetadata>(
            "SELECT id, image_name, content_type, file_size, category, description, uploaded_at, updated_at
             FROM images WHERE category = $1 ORDER BY uploaded_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(&cat)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    } else {
        sqlx::query_as::<_, ImageMetadata>(
            "SELECT id, image_name, content_type, file_size, category, description, uploaded_at, updated_at
             FROM images ORDER BY uploaded_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.db)
        .await
    };

    match result {
        Ok(images) => {
            let total = images.len() as i64;
            log::info!("✅ Retrieved {} images", images.len());
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "data": images,
                "total": total,
                "page": page,
                "page_size": page_size
            })))
        }
        Err(e) => {
            log::error!("❌ Failed to fetch images: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to fetch images"})))
        }
    }
}

// Delete image by ID
pub async fn delete_image(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let image_id = id.into_inner();

    match sqlx::query("DELETE FROM images WHERE id = $1")
        .bind(image_id)
        .execute(&state.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            log::info!("✅ Image deleted: ID {}", image_id);
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({
                    "success": true,
                    "message": "Image deleted successfully"
                })))
        }
        Ok(_) => {
            log::warn!("⚠️  Image not found: ID {}", image_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Image not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to delete image: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to delete image"})))
        }
    }
}

// Update image metadata
pub async fn update_image(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    req: web::Json<UpdateImageRequest>,
) -> Result<HttpResponse, Error> {
    let image_id = id.into_inner();

    match sqlx::query_as::<_, ImageMetadata>(
        "UPDATE images SET category = COALESCE($1, category), 
         description = COALESCE($2, description), updated_at = CURRENT_TIMESTAMP
         WHERE id = $3
         RETURNING id, image_name, content_type, file_size, category, description, uploaded_at, updated_at"
    )
    .bind(&req.category)
    .bind(&req.description)
    .bind(image_id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(image)) => {
            log::info!("✅ Image updated: ID {}", image_id);
            Ok(HttpResponse::Ok().json(image))
        }
        Ok(None) => {
            log::warn!("⚠️  Image not found: ID {}", image_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Image not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to update image: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to update image"})))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ListImagesQuery {
    pub category: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}
