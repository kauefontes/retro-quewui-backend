use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{env, path::Path};
use anyhow::Result;
use std::fs;

pub type DbPool = Pool<Sqlite>;

/// Initialize the SQLite database connection pool
pub async fn init_db() -> Result<DbPool> {
    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data.db".to_string());
    
    // Create database if it doesn't exist
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        log::info!("Creating database {}", database_url);
        Sqlite::create_database(&database_url).await?;
    }
    
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    // Run migrations manually
    log::info!("Running database migrations");
    run_migrations(&pool).await?;
    
    log::info!("Database initialized successfully");
    Ok(pool)
}

/// Run migrations manually by executing SQL files
async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    let migrations_dir = Path::new("migrations");
    
    if !migrations_dir.exists() {
        return Err(anyhow::anyhow!("Migrations directory not found"));
    }
    
    // Get all SQL files in the migrations directory
    let mut migration_files = Vec::new();
    for entry in fs::read_dir(migrations_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "sql") {
            migration_files.push(path);
        }
    }
    
    // Sort migration files by name
    migration_files.sort();
    
    // Execute each migration file
    for path in migration_files {
        let sql = fs::read_to_string(&path)?;
        log::info!("Executing migration: {:?}", path.file_name().unwrap());
        sqlx::query(&sql).execute(pool).await?;
    }
    
    Ok(())
}
