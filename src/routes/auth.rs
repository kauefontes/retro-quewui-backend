use actix_web::{post, web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::auth::{authenticate_user, AuthMiddleware, User};
use crate::error::{AppError, AppResult};

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Username for authentication
    username: String,
    /// Password for authentication
    password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// Indicates if login was successful
    success: bool,
    /// JWT token for authenticated requests
    token: String,
    /// User information
    user: User,
}

/// Authenticate user and get JWT token
///
/// Authenticates a user with username and password, and returns a JWT token
/// that can be used for authorized API requests.
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authentication successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/auth/login")]
pub async fn login(login_req: web::Json<LoginRequest>) -> AppResult<impl Responder> {
    let user = authenticate_user(&login_req.username, &login_req.password).await
        .map_err(|e| {
            error!("Authentication error: {}", e);
            AppError::internal_error("Authentication failed")
        })?;
    
    match user {
        Some(user) => {
            let token = AuthMiddleware::generate_token(&user)
                .map_err(|e| {
                    error!("Token generation error: {}", e);
                    AppError::internal_error("Failed to generate token")
                })?;
            
            info!("User {} logged in successfully", user.name);
            
            Ok(HttpResponse::Ok().json(LoginResponse {
                success: true,
                token,
                user,
            }))
        },
        None => {
            info!("Failed login attempt for user: {}", login_req.username);
            Err(AppError::unauthorized("Invalid username or password"))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
