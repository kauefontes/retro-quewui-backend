use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod auth;
mod config;
mod error;
mod models;
mod routes;
mod validation;

#[cfg(test)]
mod tests;

// No need to import DbPool here as it's not directly used

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Initialize database
    let db_pool = match config::database::init_db().await {
        Ok(pool) => {
            log::info!("Database connection established successfully");
            pool
        },
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };
    
    // Get host and port from environment or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
        
    // Frontend URL for CORS
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    
    log::info!("Starting server at http://{}:{}", host, port);
    log::info!("Allowing CORS from: {}", frontend_url);
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);
        
        // Create app data with database pool
        let app_data = web::Data::new(db_pool.clone());
            
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(app_data)
            // API Routes
            .configure(routes::health::config)
            .configure(routes::auth::config)
            .configure(routes::admin::config)
            .configure(routes::projects::config)
            .configure(routes::experiences::config)
            .configure(routes::skills::config)
            .configure(routes::posts::config)
            .configure(routes::github_stats::config)
            .configure(routes::contact::config)
            .configure(routes::profile::config)
    })
    .bind((host, port))?
    .run()
    .await
}
