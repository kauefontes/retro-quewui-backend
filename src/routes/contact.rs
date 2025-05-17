use actix_web::{post, get, delete, web, HttpResponse, Responder};
use log::info;
use validator::Validate;
use utoipa::ToSchema;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::AppResult;
use crate::models::contact::{ContactMessage, ContactResponse, ContactRepository};
use crate::validation::validate_json;

#[derive(Debug, Validate, serde::Deserialize, ToSchema)]
struct ValidatedContactMessage {
    /// Sender's full name (2-100 characters)
    #[validate(length(min = 2, max = 100, message = "Name must be between 2 and 100 characters"))]
    name: String,
    
    /// Sender's email address (must be valid email format)
    #[validate(email(message = "Invalid email format"))]
    email: String,
    
    /// Message content (10-5000 characters)
    #[validate(length(min = 10, max = 5000, message = "Message must be between 10 and 5000 characters"))]
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
    let form = match validate_json(form) {
        Ok(form) => form,
        Err(e) => {
            info!("Contact form validation failed: {}", e);
            return Err(e);
        }
    };
    
    // We've already validated the email with the validator crate's #[validate(email)]
    // attribute, so we're removing the redundant custom validation
    
    info!(
        "Contact form submission received from {} ({})",
        form.name, form.email
    );
    
    // Create contact message
    let contact = ContactMessage::new(
        form.name.clone(),
        form.email.clone(),
        form.message.clone(),
    );
    
    // Store in database
    let repo = ContactRepository::new(db.get_ref().clone());
    
    // Log the attempt to create a contact message
    info!("Attempting to store contact message in database");
    
    // Create the contact message and handle errors
    let result = repo.create(contact).await;
    
    match result {
        Ok(_) => {
            info!("Contact message stored successfully");
            Ok(HttpResponse::Ok().json(ContactResponse::success()))
        },
        Err(e) => {
            info!("Failed to store contact message: {}", e);
            // Forward the error to the error handling middleware
            Err(e)
        }
    }
}

/// Get all contact messages
///
/// Returns a list of all contact form submissions.
/// Requires authentication.
#[utoipa::path(
    get,
    path = "/admin/messages",
    tag = "contact",
    security(("jwt" = [])),
    responses(
        (status = 200, description = "List of all contact messages", body = Vec<ContactMessage>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/admin/messages")]
pub async fn get_all_messages(
    _user: AuthenticatedUser,
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let repo = ContactRepository::new(db.get_ref().clone());
    let messages = repo.get_all().await?;
    
    Ok(HttpResponse::Ok().json(messages))
}

/// Get contact message by ID
///
/// Returns a single contact message with the specified ID.
/// Requires authentication.
#[utoipa::path(
    get,
    path = "/admin/messages/{id}",
    tag = "contact",
    security(("jwt" = [])),
    params(
        ("id" = String, Path, description = "Message unique identifier")
    ),
    responses(
        (status = 200, description = "Message found", body = ContactMessage),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Message not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/admin/messages/{id}")]
pub async fn get_message_by_id(
    _user: AuthenticatedUser,
    path: web::Path<String>,
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ContactRepository::new(db.get_ref().clone());
    let message = repo.get_by_id(&id).await?;
    
    Ok(HttpResponse::Ok().json(message))
}

/// Delete a contact message
///
/// Deletes a contact message with the specified ID.
/// Requires authentication.
#[utoipa::path(
    delete,
    path = "/admin/messages/{id}",
    tag = "contact",
    security(("jwt" = [])),
    params(
        ("id" = String, Path, description = "Message unique identifier")
    ),
    responses(
        (status = 200, description = "Message deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Message not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/admin/messages/{id}")]
pub async fn delete_message(
    _user: AuthenticatedUser,
    path: web::Path<String>,
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let repo = ContactRepository::new(db.get_ref().clone());
    repo.delete(&id).await?;
    
    Ok(HttpResponse::Ok().json(ContactResponse::success()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_contact_form)
       .service(get_all_messages)
       .service(get_message_by_id)
       .service(delete_message);
}
