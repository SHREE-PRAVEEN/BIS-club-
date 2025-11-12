use actix_web::{web, HttpResponse, Error};
use sqlx::PgPool;

use crate::models::{TeamMember, TeamMemberResponse, CreateTeamMemberRequest, UpdateTeamMemberRequest};
use crate::AppState;

// Create a new team member
pub async fn create_team_member(
    state: web::Data<AppState>,
    req: web::Json<CreateTeamMemberRequest>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, TeamMember>(
        "INSERT INTO team_members (name, position, bio, email, phone, display_order, is_active)
         VALUES ($1, $2, $3, $4, $5, $6, true)
         RETURNING id, name, position, bio, email, phone, image_id, display_order, is_active, created_at, updated_at"
    )
    .bind(&req.name)
    .bind(&req.position)
    .bind(&req.bio)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(req.display_order)
    .fetch_one(&state.db)
    .await
    {
        Ok(member) => {
            log::info!("✅ Team member created: {}", member.name);
            Ok(HttpResponse::Created().json(member_to_response(&member)))
        }
        Err(e) => {
            log::error!("❌ Failed to create team member: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to create team member"})))
        }
    }
}

// Get all team members
pub async fn list_team_members(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as::<_, TeamMember>(
        "SELECT id, name, position, bio, email, phone, image_id, display_order, is_active, created_at, updated_at
         FROM team_members WHERE is_active = true ORDER BY display_order ASC NULLS LAST"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(members) => {
            let responses: Vec<TeamMemberResponse> = members
                .iter()
                .map(|m| member_to_response(m))
                .collect();
            
            log::info!("✅ Retrieved {} team members", responses.len());
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "data": responses,
                "total": responses.len()
            })))
        }
        Err(e) => {
            log::error!("❌ Failed to fetch team members: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to fetch team members"})))
        }
    }
}

// Get single team member by ID
pub async fn get_team_member(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let member_id = id.into_inner();

    match sqlx::query_as::<_, TeamMember>(
        "SELECT id, name, position, bio, email, phone, image_id, display_order, is_active, created_at, updated_at
         FROM team_members WHERE id = $1"
    )
    .bind(member_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(member) => {
            log::info!("✅ Retrieved team member: ID {}", member_id);
            Ok(HttpResponse::Ok().json(member_to_response(&member)))
        }
        Err(sqlx::Error::RowNotFound) => {
            log::warn!("⚠️  Team member not found: ID {}", member_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Team member not found"})))
        }
        Err(e) => {
            log::error!("❌ Database error: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Internal server error"})))
        }
    }
}

// Update team member
pub async fn update_team_member(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    req: web::Json<UpdateTeamMemberRequest>,
) -> Result<HttpResponse, Error> {
    let member_id = id.into_inner();

    match sqlx::query_as::<_, TeamMember>(
        "UPDATE team_members SET
            name = COALESCE($1, name),
            position = COALESCE($2, position),
            bio = COALESCE($3, bio),
            email = COALESCE($4, email),
            phone = COALESCE($5, phone),
            image_id = COALESCE($6, image_id),
            display_order = COALESCE($7, display_order),
            is_active = COALESCE($8, is_active),
            updated_at = CURRENT_TIMESTAMP
         WHERE id = $9
         RETURNING id, name, position, bio, email, phone, image_id, display_order, is_active, created_at, updated_at"
    )
    .bind(&req.name)
    .bind(&req.position)
    .bind(&req.bio)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(req.image_id)
    .bind(req.display_order)
    .bind(req.is_active)
    .bind(member_id)
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(member)) => {
            log::info!("✅ Team member updated: ID {}", member_id);
            Ok(HttpResponse::Ok().json(member_to_response(&member)))
        }
        Ok(None) => {
            log::warn!("⚠️  Team member not found: ID {}", member_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Team member not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to update team member: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to update team member"})))
        }
    }
}

// Delete team member
pub async fn delete_team_member(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let member_id = id.into_inner();

    match sqlx::query("DELETE FROM team_members WHERE id = $1")
        .bind(member_id)
        .execute(&state.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            log::info!("✅ Team member deleted: ID {}", member_id);
            Ok(HttpResponse::Ok()
                .json(serde_json::json!({
                    "success": true,
                    "message": "Team member deleted successfully"
                })))
        }
        Ok(_) => {
            log::warn!("⚠️  Team member not found: ID {}", member_id);
            Ok(HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Team member not found"})))
        }
        Err(e) => {
            log::error!("❌ Failed to delete team member: {}", e);
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to delete team member"})))
        }
    }
}

// Helper function to convert TeamMember to TeamMemberResponse
fn member_to_response(member: &TeamMember) -> TeamMemberResponse {
    TeamMemberResponse {
        id: member.id,
        name: member.name.clone(),
        position: member.position.clone(),
        bio: member.bio.clone(),
        email: member.email.clone(),
        phone: member.phone.clone(),
        image_url: member.image_id.map(|id| format!("/api/images/{}", id)),
        display_order: member.display_order,
        is_active: member.is_active,
        created_at: member.created_at,
    }
}
