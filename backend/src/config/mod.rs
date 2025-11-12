use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub max_file_size: usize,
    pub rust_log: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in environment");
        
        let host = env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid u16");
        
        let max_file_size = env::var("MAX_FILE_SIZE")
            .unwrap_or_else(|_| "10485760".to_string())
            .parse::<usize>()
            .expect("MAX_FILE_SIZE must be a valid usize");
        
        let rust_log = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        AppConfig {
            database_url,
            host,
            port,
            max_file_size,
            rust_log,
        }
    }
}
