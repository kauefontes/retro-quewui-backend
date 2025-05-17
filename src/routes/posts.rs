use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::error::{AppError, AppResult};
use crate::models::post::{Post, get_mock_posts};

/// Get all blog posts
///
/// Returns a list of all blog posts.
#[utoipa::path(
    get,
    path = "/posts",
    tag = "posts",
    responses(
        (status = 200, description = "List of all blog posts retrieved successfully", body = Vec<Post>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/posts")]
pub async fn get_all_posts() -> impl Responder {
    let posts = get_mock_posts();
    HttpResponse::Ok().json(posts)
}

/// Get blog post by ID
///
/// Returns a single blog post with the specified ID.
#[utoipa::path(
    get,
    path = "/posts/{id}",
    tag = "posts",
    params(
        ("id" = String, Path, description = "Post unique identifier")
    ),
    responses(
        (status = 200, description = "Post found", body = Post),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/posts/{id}")]
pub async fn get_post_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let posts = get_mock_posts();
    
    match posts.iter().find(|p| p.id == id) {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body(format!("Post with ID {} not found", id)),
    }
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreatePostRequest {
    /// Post title
    pub title: String,
    /// Publication date (format: YYYY-MM-DD)
    pub date: String,
    /// List of tags for the post
    pub tags: Vec<String>,
    /// Short excerpt or summary of the post
    pub excerpt: String,
    /// Full content of the post (Markdown format)
    pub content: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdatePostRequest {
    /// Post title
    pub title: Option<String>,
    /// Publication date (format: YYYY-MM-DD)
    pub date: Option<String>,
    /// List of tags for the post
    pub tags: Option<Vec<String>>,
    /// Short excerpt or summary of the post
    pub excerpt: Option<String>,
    /// Full content of the post (Markdown format)
    pub content: Option<String>,
}

/// Create a new blog post
///
/// Creates a new blog post with the provided details.
/// Requires authentication.
#[utoipa::path(
    post,
    path = "/posts",
    tag = "posts",
    security(
        ("jwt_auth" = [])
    ),
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Blog post created successfully", body = Post),
        (status = 400, description = "Invalid post data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/posts")]
pub async fn create_post(
    post_req: web::Json<CreatePostRequest>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let post = Post::new(
        post_req.title.clone(),
        post_req.date.clone(),
        post_req.tags.clone(),
        post_req.excerpt.clone(),
        post_req.content.clone(),
    );
    
    // In a real application, you would save this to a database
    info!("Created new blog post: {}", post.title);
    
    Ok(HttpResponse::Created().json(post))
}

/// Update an existing blog post
///
/// Updates an existing blog post with the specified ID.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/posts/{id}",
    tag = "posts",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("id" = String, Path, description = "Post unique identifier")
    ),
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Blog post updated successfully", body = Post),
        (status = 400, description = "Invalid post data"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/posts/{id}")]
pub async fn update_post(
    path: web::Path<String>,
    post_req: web::Json<UpdatePostRequest>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let posts = get_mock_posts();
    
    // Find the post to update
    let existing_post = posts.iter().find(|p| p.id == id)
        .ok_or_else(|| {
            info!("Post with ID {} not found for update", id);
            AppError::not_found(format!("Post with ID {} not found", id))
        })?;
    
    // Create updated post
    let updated_post = Post {
        id: existing_post.id.clone(),
        title: post_req.title.clone().unwrap_or_else(|| existing_post.title.clone()),
        date: post_req.date.clone().unwrap_or_else(|| existing_post.date.clone()),
        tags: post_req.tags.clone().unwrap_or_else(|| existing_post.tags.clone()),
        excerpt: post_req.excerpt.clone().unwrap_or_else(|| existing_post.excerpt.clone()),
        content: post_req.content.clone().unwrap_or_else(|| existing_post.content.clone()),
    };
    
    // In a real application, you would update this in a database
    info!("Updated blog post with ID: {}", id);
    
    Ok(HttpResponse::Ok().json(updated_post))
}

/// Delete a blog post
///
/// Deletes the blog post with the specified ID.
/// Requires authentication.
#[utoipa::path(
    delete,
    path = "/posts/{id}",
    tag = "posts",
    security(
        ("jwt_auth" = [])
    ),
    params(
        ("id" = String, Path, description = "Post unique identifier")
    ),
    responses(
        (status = 204, description = "Blog post deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/posts/{id}")]
pub async fn delete_post(
    path: web::Path<String>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let posts = get_mock_posts();
    
    // Check if the post exists
    let post_exists = posts.iter().any(|p| p.id == id);
    
    if !post_exists {
        info!("Post with ID {} not found for deletion", id);
        return Err(AppError::not_found(format!("Post with ID {} not found", id)));
    }
    
    // In a real application, you would delete this from a database
    info!("Deleted blog post with ID: {}", id);
    
    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_posts)
       .service(get_post_by_id)
       .service(create_post)
       .service(update_post)
       .service(delete_post);
}
