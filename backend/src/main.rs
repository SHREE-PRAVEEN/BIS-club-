use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod models;
mod handlers;
mod error;
mod config;
mod middleware;
mod utils;

use config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<AppConfig>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize logger
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let config = AppConfig::from_env();
    log::info!("🚀 Starting BIS Club Backend");
    log::info!("📊 Database URL: {}", mask_database_url(&config.database_url));
    log::info!("🔧 Max file size: {} bytes", config.max_file_size);

    // Create database connection pool
    let database_url = &config.database_url;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("❌ Failed to connect to PostgreSQL database");

    log::info!("✅ Database connection established");

    // Run migrations
    match sqlx::migrate!("./migrations")
        .run(&pool)
        .await {
        Ok(_) => log::info!("✅ Database migrations completed"),
        Err(e) => {
            log::error!("❌ Migration error: {}", e);
            panic!("Failed to run migrations");
        }
    }

    let app_state = AppState {
        db: pool.clone(),
        config: Arc::new(config.clone()),
    };

    let host = config.host.clone();
    let port = config.port;
    let bind_addr = format!("{}:{}", host, port);

    log::info!("🌐 Server starting on http://{}", bind_addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .wrap(middleware::RequestIdMiddleware)
            .configure(handlers::config)
            .default_service(web::route().to(handlers::not_found))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

fn mask_database_url(url: &str) -> String {
    // Hide password in logs
    if let Some(at_pos) = url.rfind('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            format!("{}:****@{}", &url[..colon_pos], &url[at_pos + 1..])
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    }
}
