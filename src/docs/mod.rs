use actix_web::{web, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Define the OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        // Health endpoint
        crate::routes::health::health_check,

        // Auth endpoints
        crate::routes::auth::login,

        // Project endpoints
        crate::routes::projects::get_all_projects,
        crate::routes::projects::get_project_by_id,
        crate::routes::projects::create_project,
        crate::routes::projects::update_project,
        crate::routes::projects::delete_project,

        // Experience endpoints
        crate::routes::experiences::get_all_experiences,
        crate::routes::experiences::get_experience_by_id,
        crate::routes::experiences::create_experience,
        crate::routes::experiences::update_experience,
        crate::routes::experiences::delete_experience,

        // Skills endpoint
        crate::routes::skills::get_all_skills,
        crate::routes::skills::create_skill,
        crate::routes::skills::update_skill,
        crate::routes::skills::delete_skill,

        // Posts endpoints
        crate::routes::posts::get_all_posts,
        crate::routes::posts::get_post_by_id,
        crate::routes::posts::create_post,
        crate::routes::posts::update_post,
        crate::routes::posts::delete_post,

        // GitHub stats endpoint
        crate::routes::github_stats::get_github_stats,
        crate::routes::github_stats::update_github_stats,
        crate::routes::github_stats::refresh_github_stats,

        // Profile endpoint
        crate::routes::profile::get_profile,
        crate::routes::profile::update_profile,

        // Contact endpoints
        crate::routes::contact::submit_contact_form,
        crate::routes::contact::get_all_messages,
        crate::routes::contact::get_message_by_id,
        crate::routes::contact::delete_message
    ),
    components(
        schemas(
            // Models
            crate::models::project::Project,
            crate::models::experience::Experience,
            crate::models::skill::Skill,
            crate::models::post::Post,
            crate::models::github_stats::GithubStats,
            crate::models::github_stats::TopLanguage,
            crate::models::github_stats::RecentActivity,
            crate::models::profile::Profile,
            crate::models::profile::SocialLink,
            crate::models::profile::Education,
            crate::models::profile::Language,
            crate::models::contact::ContactMessage,
            crate::models::contact::ContactResponse,

            // Request bodies
            crate::routes::projects::CreateProjectRequest,
            crate::routes::projects::UpdateProjectRequest,
            crate::routes::experiences::CreateExperienceRequest,
            crate::routes::experiences::UpdateExperienceRequest,
            crate::routes::skills::CreateSkillRequest,
            crate::routes::skills::UpdateSkillRequest,
            crate::routes::posts::CreatePostRequest,
            crate::routes::posts::UpdatePostRequest,
            crate::routes::github_stats::UpdateGithubStatsRequest,
            crate::routes::profile::UpdateProfileRequest,

            // Auth
            crate::auth::User,
            crate::auth::Claims,
            crate::routes::auth::LoginRequest,
            crate::routes::auth::LoginResponse,

            // Health
            crate::routes::health::HealthResponse,

            // Error
            crate::error::ErrorResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "admin", description = "Admin dashboard endpoints"),
        (name = "projects", description = "Project management endpoints"),
        (name = "experiences", description = "Professional experience endpoints"),
        (name = "skills", description = "Skills and technologies endpoints"),
        (name = "posts", description = "Blog post endpoints"),
        (name = "github-stats", description = "GitHub statistics endpoints"),
        (name = "profile", description = "User profile endpoints"),
        (name = "contact", description = "Contact form endpoints")
    ),
    info(
        title = "Retro Quewui Backend API",
        version = "0.1.0",
        description = "A production-ready Rust API backend for the retro-tech style 'Quewui' portfolio",
        license(
            name = "MIT",
            url = "https://github.com/yourusername/retro-quewui-backend/blob/main/LICENSE"
        ),
        contact(
            name = "Retro Quewui Team",
            url = "https://github.com/yourusername/retro-quewui-backend",
            email = "contact@example.com"
        )
    )
)]
pub struct ApiDoc;

// Function to configure the Swagger UI
pub fn configure_swagger_ui() -> SwaggerUi {
    let openapi = ApiDoc::openapi();
    
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", openapi)
}

// Serve OpenAPI JSON
pub async fn serve_openapi() -> impl Responder {
    HttpResponse::Ok().json(ApiDoc::openapi())
}

// Configure routes for API documentation
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/api-docs/openapi.json", web::get().to(serve_openapi))
       .service(configure_swagger_ui());
}
