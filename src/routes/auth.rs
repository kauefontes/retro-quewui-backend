use actix_web::{post, web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::auth::{authenticate_user, AuthMiddleware, User};
use crate::error::{AppError, AppResult};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    success: bool,
    token: String,
    user: User,
}

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
