use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use validator::Validate;
use utoipa::ToSchema;

use crate::config::database::DbPool;
use crate::error::AppResult;
use crate::models::contact::{ContactMessage, ContactResponse};
use crate::validation::{validate_json, validate_email};

#[derive(Debug, Validate, serde::Deserialize, ToSchema)]
struct ValidatedContactMessage {
    /// Sender's full name (2-100 characters)
    #[validate(length(min = 2, max = 100, message = "Name must be between 2 and 100 characters"))]
    name: String,
    
    /// Sender's email address (must be valid email format)
    #[validate(email(message = "Invalid email format"))]
    email: String,
    
    /// Message content (10-1000 characters)
    #[validate(length(min = 10, max = 1000, message = "Message must be between 10 and 1000 characters"))]
    message: String,
}

/// Submit contact form
///
/// Submits a contact form message for review.
/// All fields are validated before processing.
#[utoipa::path(
    post,
    path = "/contact",
    tag = "contact",
    request_body = ValidatedContactMessage,
    responses(
        (status = 200, description = "Contact form submitted successfully", body = ContactResponse),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/contact")]
pub async fn submit_contact_form(
    form: web::Json<ValidatedContactMessage>,
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    // Validate the form
    let form = validate_json(form)?;
    
    // Additional validation if needed
    validate_email(&form.email)?;
    
    info!(
        "Contact form submission received from {} ({})",
        form.name, form.email
    );
    
    // In a real application, you would:
    // 1. Store in database
    let contact = ContactMessage::new(
        form.name.clone(),
        form.email.clone(),
        form.message.clone(),
    );
    
    // TODO: Store in database
    // let repo = ContactRepository::new(db.get_ref().clone());
    // repo.create(contact).await?;
    
    // 2. Send email notification
    // TODO: Implement email sending
    
    Ok(HttpResponse::Ok().json(ContactResponse::success()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_contact_form);
}
