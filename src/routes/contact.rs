use actix_web::{post, web, HttpResponse, Responder};
use log::info;

use crate::models::contact::{ContactMessage, ContactResponse};

#[post("/contact")]
pub async fn submit_contact_form(message: web::Json<ContactMessage>) -> impl Responder {
    info!(
        "Contact form submission received from {} ({})",
        message.name, message.email
    );
    
    // In a real application, you would:
    // 1. Validate the email format
    // 2. Check for spam
    // 3. Store in database or send email
    // 4. Handle potential errors
    
    // For now, we'll just log and return success
    HttpResponse::Ok().json(ContactResponse::success())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_contact_form);
}
