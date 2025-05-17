use actix_web::web;
use validator::{Validate, ValidationErrors};

use crate::error::{AppError, AppResult};

// Helper function to validate JSON input
pub fn validate_json<T>(json: web::Json<T>) -> Result<T, AppError>
where
    T: Validate,
{
    match json.validate() {
        Ok(_) => Ok(json.into_inner()),
        Err(e) => {
            let error_message = format_validation_errors(&e);
            Err(AppError::validation_error(error_message))
        }
    }
}

// Helper function to format validation errors
fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut error_message = String::from("Validation failed: ");
    
    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = error.message.as_ref().map_or_else(
                || format!("Invalid value for field '{}'", field),
                |msg| format!("{} for field '{}'", msg, field),
            );
            error_message.push_str(&message);
            error_message.push_str("; ");
        }
    }
    
    error_message
}

// Validate email format
pub fn validate_email(email: &str) -> AppResult<()> {
    let email_regex = regex::Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
    )
    .unwrap();
    
    if !email_regex.is_match(email) {
        return Err(AppError::validation_error("Invalid email format"));
    }
    
    Ok(())
}
