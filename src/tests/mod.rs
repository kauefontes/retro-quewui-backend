#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use sqlx::sqlite::SqlitePoolOptions;
    
    use crate::routes;
    
    async fn setup_test_db() -> sqlx::Pool<sqlx::Sqlite> {
        // Create in-memory database for testing
        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");
        
        // Run migrations
        sqlx::query("CREATE TABLE IF NOT EXISTS projects (id TEXT PRIMARY KEY, title TEXT NOT NULL)")
            .execute(&db_pool)
            .await
            .expect("Failed to create test table");
        
        db_pool
    }
    
    #[actix_web::test]
    async fn test_health_endpoint() {
        // Setup
        let db_pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db_pool.clone()))
                .configure(routes::health::config)
        ).await;
        
        // Act
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        
        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
    }
    
    #[actix_web::test]
    async fn test_get_projects() {
        // Setup
        let db_pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db_pool.clone()))
                .configure(routes::projects::config)
        ).await;
        
        // Act
        let req = test::TestRequest::get().uri("/projects").to_request();
        let resp = test::call_service(&app, req).await;
        
        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
