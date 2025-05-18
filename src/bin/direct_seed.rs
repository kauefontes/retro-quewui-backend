use std::error::Error;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Row};
use dotenv::dotenv;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();
    
    println!("Starting database seeding directly from CV data...");
    
    // Initialize database connection
    let database_url = "sqlite:data.db";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Seed profile data
    println!("Seeding profile data...");
    seed_profile(&pool).await?;
    
    // Seed skills data
    println!("Seeding skills data...");
    seed_skills(&pool).await?;
    
    // Seed experiences data
    println!("Seeding experiences data...");
    seed_experiences(&pool).await?;
    
    // Seed GitHub stats
    println!("Seeding GitHub stats data...");
    seed_github_stats(&pool).await?;

    println!("Seed data loaded successfully!");
    Ok(())
}

async fn seed_profile(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    // Bio entries
    let bio = json!([
        "Accomplished software engineer with over 9 years of experience specializing in frontend and backend development.",
        "Expert in React, Next.js, Typescript, and Rust, with a proven track record of leading cross-functional teams and driving agile practices.",
        "Demonstrated success in delivering high-quality software solutions, improving performance metrics, and streamlining workflows with modern cloud and containerization technologies.",
        "Ready to leverage my expertise and leadership skills to solve complex technical challenges."
    ]);

    // Social links
    let social_links = json!([
        {
            "title": "GitHub",
            "url": "https://github.com/kauefontes",
            "icon": "github"
        },
        {
            "title": "LinkedIn",
            "url": "https://linkedin.com/in/kauefontes",
            "icon": "linkedin"
        },
        {
            "title": "Email",
            "url": "mailto:kauefontes@outlook.com",
            "icon": "envelope"
        },
        {
            "title": "Phone",
            "url": "tel:+55-92-98138-6423",
            "icon": "phone"
        }
    ]);

    // Education
    let education = json!([
        {
            "degree": "Embedded Android Specialization",
            "institution": "UEA (State University of Amazonas)",
            "period": "2019-2020"
        },
        {
            "degree": "Bachelor's in Systems Analysis and Development",
            "institution": "Estácio de Sá University",
            "period": "2017-2018"
        },
        {
            "degree": "Systems Analysis and Development",
            "institution": "UEA (State University of Amazonas)",
            "period": "2013-2017"
        }
    ]);

    // Languages
    let languages = json!([
        {
            "name": "Portuguese",
            "level": "Native"
        },
        {
            "name": "English",
            "level": "Fluent"
        }
    ]);

    // Insert profile data
    sqlx::query(
        "INSERT INTO profiles (id, bio, social_links, education, languages) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind("profile1") // Unique ID for the profile
    .bind(bio.to_string())
    .bind(social_links.to_string())
    .bind(education.to_string())
    .bind(languages.to_string())
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_skills(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    // Frontend skills
    let frontend_items = json!([
        "React Native", "React", "Next.js", "Typescript", "Javascript", 
        "HTML", "CSS", "Redux", "Redux Toolkit", "MobX", "Hooks", "Design Patterns"
    ]);
    
    sqlx::query(
        "INSERT INTO skills (id, category, items) VALUES (?, ?, ?)"
    )
    .bind("frontend")
    .bind("Frontend")
    .bind(frontend_items.to_string())
    .execute(pool)
    .await?;

    // Backend skills
    let backend_items = json!([
        "Rust", ".NET", "NodeJS", "NestJS", "Prisma", "TypeORM", "RESTful API", 
        "SOAP", "Docker", "Kubernetes", "AWS", "Google Cloud", "Azure"
    ]);
    
    sqlx::query(
        "INSERT INTO skills (id, category, items) VALUES (?, ?, ?)"
    )
    .bind("backend")
    .bind("Backend")
    .bind(backend_items.to_string())
    .execute(pool)
    .await?;

    // DevOps & Tools skills
    let tools_items = json!([
        "Firebase", "GitHub Actions", "Jenkins", "Storybook", "Pipelines", 
        "Continuous Integration (CI)", "Continuous Deployment (CD)"
    ]);
    
    sqlx::query(
        "INSERT INTO skills (id, category, items) VALUES (?, ?, ?)"
    )
    .bind("devops")
    .bind("DevOps & Tools")
    .bind(tools_items.to_string())
    .execute(pool)
    .await?;

    // Mobile & IoT skills
    let mobile_iot_items = json!([
        "Android", "iOS", "Kotlin", "Java", "BLE", "Push Notifications", "ESP32", "FFmpeg"
    ]);
    
    sqlx::query(
        "INSERT INTO skills (id, category, items) VALUES (?, ?, ?)"
    )
    .bind("mobile")
    .bind("Mobile & IoT")
    .bind(mobile_iot_items.to_string())
    .execute(pool)
    .await?;

    // Agile & Management skills
    let agile_items = json!([
        "Kanban", "Scrum", "Agile Coaching", "Team Building", 
        "Risk Management", "Capacity Management", "Team Metrics"
    ]);
    
    sqlx::query(
        "INSERT INTO skills (id, category, items) VALUES (?, ?, ?)"
    )
    .bind("agile")
    .bind("Agile & Management")
    .bind(agile_items.to_string())
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_experiences(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    // BairesDev Experience
    let bairesdev_exp = create_experience_json(
        "BairesDev",
        "Senior Software Engineer",
        "2023-12",
        None, // Present
        "Working for the Hensall Coorp client, developing both frontend and backend solutions. For the Lenslock client, developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%.",
        vec![
            "React", "Redux", "Styled Components", "NestJS", "TypeORM", "Prisma", 
            "Jest", "GitHub Actions", "Docker", "Storybook", "Typescript"
        ],
        vec![
            "Built and maintained CI/CD pipelines using GitHub Actions, automating testing, linting, and deployments",
            "Leveraged containerization for local development environments, including databases and auxiliary services",
            "Developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp1")
    .bind(bairesdev_exp["company"].as_str().unwrap())
    .bind(bairesdev_exp["position"].as_str().unwrap())
    .bind(bairesdev_exp["start_date"].as_str().unwrap())
    .bind(bairesdev_exp["end_date"].as_str())
    .bind(bairesdev_exp["description"].as_str().unwrap())
    .bind(bairesdev_exp["technologies"].to_string())
    .bind(bairesdev_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    // MTST Experience
    let mtst_exp = create_experience_json(
        "MTST Technology Center",
        "Coordinator Developer",
        "2021-08",
        None, // Present (Volunteer)
        "Designed and implemented a video and image processing service in Rust, using FFmpeg to manipulate videos and audio codecs. Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects.",
        vec![
            "Rust", "FFmpeg", "ESP32", "React Native", "Docker", "Kubernetes", "IoT"
        ],
        vec![
            "Designed and implemented a video and image processing service in Rust, using FFmpeg to manipulate videos and audio codecs",
            "Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects",
            "Introduced CI/CD workflows using Docker and Kubernetes, streamlining deployments and improving efficiency"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp2")
    .bind(mtst_exp["company"].as_str().unwrap())
    .bind(mtst_exp["position"].as_str().unwrap())
    .bind(mtst_exp["start_date"].as_str().unwrap())
    .bind(mtst_exp["end_date"].as_str())
    .bind(mtst_exp["description"].as_str().unwrap())
    .bind(mtst_exp["technologies"].to_string())
    .bind(mtst_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    // AB InBev Experience
    let ab_inbev_exp = create_experience_json(
        "AB InBev",
        "Agilist",
        "2022-03",
        Some("2023-11"),
        "Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements, increasing customer engagement by 35%. Implemented Kanban methodology, improving task visibility and team productivity by 150%.",
        vec![
            "Kanban", "Agile", "Scrum", "Team Management", "Feature Activation", "Customer Engagement"
        ],
        vec![
            "Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements",
            "Increased customer engagement by 35% through strategic feature releases",
            "Implemented Kanban methodology, improving task visibility and team productivity by 150%"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp3")
    .bind(ab_inbev_exp["company"].as_str().unwrap())
    .bind(ab_inbev_exp["position"].as_str().unwrap())
    .bind(ab_inbev_exp["start_date"].as_str().unwrap())
    .bind(ab_inbev_exp["end_date"].as_str().unwrap())
    .bind(ab_inbev_exp["description"].as_str().unwrap())
    .bind(ab_inbev_exp["technologies"].to_string())
    .bind(ab_inbev_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    // ParaChegar Experience
    let para_chegar_exp = create_experience_json(
        "ParaChegar",
        "Mobile Developer",
        "2023-01",
        Some("2023-09"),
        "Developed intuitive and user-friendly React Native applications. Integrated native modules for push notifications and location-based services, enhancing app functionality.",
        vec![
            "React Native", "Push Notifications", "Location Services", "Mobile Development", "UI/UX", "Native Modules"
        ],
        vec![
            "Developed intuitive and user-friendly React Native applications",
            "Integrated native modules for push notifications and location-based services",
            "Enhanced app functionality with custom native extensions"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp4")
    .bind(para_chegar_exp["company"].as_str().unwrap())
    .bind(para_chegar_exp["position"].as_str().unwrap())
    .bind(para_chegar_exp["start_date"].as_str().unwrap())
    .bind(para_chegar_exp["end_date"].as_str().unwrap())
    .bind(para_chegar_exp["description"].as_str().unwrap())
    .bind(para_chegar_exp["technologies"].to_string())
    .bind(para_chegar_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    // Eldorado Experience
    let eldorado_exp = create_experience_json(
        "Eldorado Research Institute",
        "Developer list",
        "2018-04",
        Some("2022-02"),
        "Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times. Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS. Designed and implemented continuous integration pipelines, reducing deployment time by 50%.",
        vec![
            ".NET", "Docker", "AWS", "Agile", "B2B", "B2C", "CI/CD", "Backend", "Frontend"
        ],
        vec![
            "Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times",
            "Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS",
            "Designed and implemented continuous integration pipelines, reducing deployment time by 50%"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp5")
    .bind(eldorado_exp["company"].as_str().unwrap())
    .bind(eldorado_exp["position"].as_str().unwrap())
    .bind(eldorado_exp["start_date"].as_str().unwrap())
    .bind(eldorado_exp["end_date"].as_str().unwrap())
    .bind(eldorado_exp["description"].as_str().unwrap())
    .bind(eldorado_exp["technologies"].to_string())
    .bind(eldorado_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    // SIDIA Experience
    let sidia_exp = create_experience_json(
        "SIDIA - Samsung Institute",
        "QA Analyst",
        "2015-11",
        Some("2018-03"),
        "Conducted QA testing for Samsung products.",
        vec![
            "QA Testing", "Mobile Testing", "Samsung Products", "Quality Assurance"
        ],
        vec![
            "Conducted comprehensive QA testing for Samsung products",
            "Ensured product quality through rigorous testing methodologies"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp6")
    .bind(sidia_exp["company"].as_str().unwrap())
    .bind(sidia_exp["position"].as_str().unwrap())
    .bind(sidia_exp["start_date"].as_str().unwrap())
    .bind(sidia_exp["end_date"].as_str().unwrap())
    .bind(sidia_exp["description"].as_str().unwrap())
    .bind(sidia_exp["technologies"].to_string())
    .bind(sidia_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    // INDT Experience
    let indt_exp = create_experience_json(
        "INDT (Nokia Institute)",
        "Junior Developer",
        "2015-05",
        Some("2015-10"),
        "Automated hardware and software testing processes.",
        vec![
            "Automated Testing", "Hardware Testing", "Software Testing", "Test Automation"
        ],
        vec![
            "Automated hardware and software testing processes",
            "Improved testing efficiency through automation"
        ]
    );
    
    sqlx::query(
        "INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("exp7")
    .bind(indt_exp["company"].as_str().unwrap())
    .bind(indt_exp["position"].as_str().unwrap())
    .bind(indt_exp["start_date"].as_str().unwrap())
    .bind(indt_exp["end_date"].as_str().unwrap())
    .bind(indt_exp["description"].as_str().unwrap())
    .bind(indt_exp["technologies"].to_string())
    .bind(indt_exp["highlights"].to_string())
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_github_stats(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    // Top languages
    let top_languages = json!([
        {
            "name": "Rust",
            "percentage": 35
        },
        {
            "name": "TypeScript",
            "percentage": 30
        },
        {
            "name": "JavaScript",
            "percentage": 20
        },
        {
            "name": "C#",
            "percentage": 10
        },
        {
            "name": "Other",
            "percentage": 5
        }
    ]);

    // Recent activity
    let recent_activity = json!([
        {
            "date": "2025-05-15",
            "message": "Implemented video processing service for the MTST Technology Center",
            "repo": "kauefontes/video-processor-rust"
        },
        {
            "date": "2025-05-10",
            "message": "Added React component library for Lenslock project",
            "repo": "kauefontes/lenslock-components"
        },
        {
            "date": "2025-05-05",
            "message": "Updated IoT dashboard for urban gardening project",
            "repo": "kauefontes/urban-garden-dashboard"
        },
        {
            "date": "2025-05-01",
            "message": "Fixed CI/CD pipeline for BairesDev project",
            "repo": "kauefontes/hensall-backend"
        },
        {
            "date": "2025-04-25",
            "message": "Added Rust documentation for backend service",
            "repo": "kauefontes/rust-backend-service"
        }
    ]);

    // Insert GitHub stats
    sqlx::query(
        "INSERT INTO github_stats (id, username, repo_count, followers, contributions, top_languages, recent_activity) 
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind("stats1")
    .bind("kauefontes")
    .bind(35)
    .bind(120)
    .bind(980)
    .bind(top_languages.to_string())
    .bind(recent_activity.to_string())
    .execute(pool)
    .await?;

    Ok(())
}

// Helper function to create a JSON representation of an experience
fn create_experience_json(
    company: &str,
    position: &str,
    start_date: &str,
    end_date: Option<&str>,
    description: &str,
    technologies: Vec<&str>,
    highlights: Vec<&str>
) -> serde_json::Value {
    json!({
        "company": company,
        "position": position,
        "start_date": start_date,
        "end_date": end_date,
        "description": description,
        "technologies": technologies,
        "highlights": highlights
    })
}
