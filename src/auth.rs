use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, Error, FromRequest, HttpRequest,
};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::ToSchema;

use crate::error::{AppError, AppResult};

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// User name
    pub name: String,
    /// User role
    pub role: String,
    /// Expiration time (as UTC timestamp)
    pub exp: usize,
    /// Issued at (as UTC timestamp)
    pub iat: usize,
}

// User structure
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
   "id": "1",
   "name": "Admin User",
   "role": "admin"
}))]
pub struct User {
    /// Unique user identifier
    pub id: String,
    /// User's full name
    pub name: String,
    /// User's role (e.g., "admin", "user")
    pub role: String,
}

// Authentication middleware
pub struct AuthMiddleware;

impl AuthMiddleware {
    // Get JWT secret from environment or use default
    fn get_secret() -> String {
        env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret_for_development".to_string())
    }

    // Generate JWT token
    pub fn generate_token(user: &User) -> AppResult<String> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            exp: expiration,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Self::get_secret().as_bytes()),
        )
        .map_err(|e| AppError::internal_error(format!("Failed to generate token: {}", e)))?;

        Ok(token)
    }

    // Validate JWT token
    pub fn validate_token(token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(Self::get_secret().as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
}

// Extractor for authenticated user
pub struct AuthenticatedUser(pub User);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get authorization header
        let auth_header = req.headers().get(header::AUTHORIZATION);

        let auth_header = match auth_header {
            Some(header) => header,
            None => return ready(Err(ErrorUnauthorized("No authorization header"))),
        };

        // Parse Bearer token
        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid authorization header"))),
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Invalid authorization scheme")));
        }

        let token = &auth_str[7..]; // Remove "Bearer " prefix

        // Validate token
        let claims = match AuthMiddleware::validate_token(token) {
            Ok(claims) => claims,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid token"))),
        };

        // Create user from claims
        let user = User {
            id: claims.sub,
            name: claims.name,
            role: claims.role,
        };

        ready(Ok(AuthenticatedUser(user)))
    }
}

// Role-based authorization middleware
pub fn require_admin() -> impl Fn(AuthenticatedUser) -> Result<AuthenticatedUser, Error> {
    move |user: AuthenticatedUser| {
        if user.0.role == "admin" {
            Ok(user)
        } else {
            Err(ErrorUnauthorized("Admin access required"))
        }
    }
}

// Mock function to authenticate user (in a real app, this would check against a database)
pub async fn authenticate_user(username: &str, password: &str) -> AppResult<Option<User>> {
    // In a real application, you would check credentials against a database
    // For now, we'll use environment variables for credentials
    let admin_username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    if (username == admin_username && password == admin_password) {
        Ok(Some(User {
            id: "1".to_string(),
            name: "Kaue Fontes".to_string(),
            role: "admin".to_string(),
        }))
    } else {
        Ok(None)
    }
}
