use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use retro_quewui_backend::services::github_service::GitHubService;
use retro_quewui_backend::models::github_profile_repository::GitHubProfileRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Initialize database
    let db_pool = match retro_quewui_backend::config::database::init_db().await {
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
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
        
    // Frontend URL for CORS
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://192.168.15.153:5173".to_string());
    
    log::info!("Starting server at http://{}:{}", host, port);
    log::info!("API documentation available at http://{}:{}/docs", host, port);
    log::info!("Allowing CORS from: {}", frontend_url);
    
    // Initialize GitHub service
    // Obter credenciais do GitHub do ambiente
    let github_username = env::var("GITHUB_USERNAME").unwrap_or_else(|_| "github_user".to_string());
    let github_token = env::var("GITHUB_TOKEN").ok();
    
    let github_service = GitHubService::new(github_username, github_token);
    log::info!("GitHub service initialized successfully");
    
    // Create GitHub repository
    let github_repo = web::Data::new(
        GitHubProfileRepository::new(db_pool.clone(), github_service)
    );
    
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
            .app_data(github_repo.clone())
            // API Documentation
            .configure(retro_quewui_backend::docs::config)
            // API Routes
            .configure(retro_quewui_backend::routes::health::config)
            .configure(retro_quewui_backend::routes::auth::config)
            .configure(retro_quewui_backend::routes::admin::config)
            .configure(retro_quewui_backend::routes::projects::config)
            .configure(retro_quewui_backend::routes::experiences::config)
            .configure(retro_quewui_backend::routes::skills::config)
            .configure(retro_quewui_backend::routes::posts::config)
            .configure(retro_quewui_backend::routes::github_stats::config)
            .configure(retro_quewui_backend::routes::github_profile::configure)
            .configure(retro_quewui_backend::routes::contact::config)
            .configure(retro_quewui_backend::routes::profile::config)
    })
    .bind((host, port))?
    .run()
    .await
}
