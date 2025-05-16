use actix_web::{get, web, HttpResponse, Responder};

use crate::models::post::{Post, get_mock_posts};

#[get("/posts")]
pub async fn get_all_posts() -> impl Responder {
    let posts = get_mock_posts();
    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
pub async fn get_post_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let posts = get_mock_posts();
    
    match posts.iter().find(|p| p.id == id) {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body(format!("Post with ID {} not found", id)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_posts)
       .service(get_post_by_id);
}
