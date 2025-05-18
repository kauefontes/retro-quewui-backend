use std::process;
use std::error::Error;

mod scripts {
    pub mod seed_from_cv;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting database seeding from CV data...");
    
    if let Err(err) = scripts::seed_from_cv::seed_data().await {
        eprintln!("Error seeding database: {}", err);
        process::exit(1);
    }
    
    println!("Database seeding completed successfully!");
    Ok(())
}
