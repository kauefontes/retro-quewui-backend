use std::error::Error;
use sqlx::sqlite::SqlitePoolOptions;
use serde_json::json;

// Import the required modules from the main crate
use retro_quewui_backend::models::experience::Experience;
use retro_quewui_backend::models::experience_repository::ExperienceRepository;
use retro_quewui_backend::models::profile::{Profile, SocialLink, Education, Language};
use retro_quewui_backend::models::profile_repository::ProfileRepository;
use retro_quewui_backend::models::skill::{Skill};
use retro_quewui_backend::models::skill_repository::SkillRepository;
use retro_quewui_backend::models::github_stats::{GithubStats, TopLanguage, RecentActivity};
use retro_quewui_backend::models::github_stats_repository::GithubStatsRepository;
use retro_quewui_backend::models::repository::Repository;

pub async fn seed_data() -> Result<(), Box<dyn Error>> {
    // Initialize database connection
    println!("Connecting to database...");
    let database_url = "sqlite:data.db";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Create repositories
    let profile_repo = ProfileRepository::new(pool.clone());
    let experience_repo = ExperienceRepository::new(pool.clone());
    let skill_repo = SkillRepository::new(pool.clone());
    let github_stats_repo = GithubStatsRepository::new(pool.clone());

    // Seed profile data
    println!("Seeding profile data...");
    let profile = Profile {
        bio: vec![
            "Accomplished software engineer with over 9 years of experience specializing in frontend and backend development.".to_string(),
            "Expert in React, Next.js, Typescript, and Rust, with a proven track record of leading cross-functional teams and driving agile practices.".to_string(),
            "Demonstrated success in delivering high-quality software solutions, improving performance metrics, and streamlining workflows with modern cloud and containerization technologies.".to_string(),
            "Ready to leverage my expertise and leadership skills to solve complex technical challenges.".to_string(),
        ],
        social_links: vec![
            SocialLink {
                title: "GitHub".to_string(),
                url: "https://github.com/kauefontes".to_string(),
                icon: "github".to_string(),
            },
            SocialLink {
                title: "LinkedIn".to_string(),
                url: "https://linkedin.com/in/kauefontes".to_string(),
                icon: "linkedin".to_string(),
            },
            SocialLink {
                title: "Email".to_string(),
                url: "mailto:kauefontes@outlook.com".to_string(),
                icon: "envelope".to_string(),
            },
            SocialLink {
                title: "Phone".to_string(),
                url: "tel:+55-92-98138-6423".to_string(),
                icon: "phone".to_string(),
            },
        ],
        education: vec![
            Education {
                degree: "Embedded Android Specialization".to_string(),
                institution: "UEA (State University of Amazonas)".to_string(),
                period: "2019-2020".to_string(),
            },
            Education {
                degree: "Bachelor's in Systems Analysis and Development".to_string(),
                institution: "Estácio de Sá University".to_string(),
                period: "2017-2018".to_string(),
            },
            Education {
                degree: "Systems Analysis and Development".to_string(),
                institution: "UEA (State University of Amazonas)".to_string(),
                period: "2013-2017".to_string(),
            },
        ],
        languages: vec![
            Language {
                name: "Portuguese".to_string(),
                level: "Native".to_string(),
            },
            Language {
                name: "English".to_string(),
                level: "Fluent".to_string(),
            },
        ],
    };
    profile_repo.create(profile).await?;

    // Seed skills data
    println!("Seeding skills data...");
    let frontend_skills = Skill {
        category: "Frontend".to_string(),
        items: vec![
            "React Native".to_string(),
            "React".to_string(),
            "Next.js".to_string(),
            "Typescript".to_string(),
            "Javascript".to_string(),
            "HTML".to_string(),
            "CSS".to_string(),
            "Redux".to_string(),
            "Redux Toolkit".to_string(),
            "MobX".to_string(),
            "Hooks".to_string(),
            "Design Patterns".to_string(),
        ],
    };
    skill_repo.create(frontend_skills).await?;

    let backend_skills = Skill {
        category: "Backend".to_string(),
        items: vec![
            "Rust".to_string(),
            ".NET".to_string(),
            "NodeJS".to_string(),
            "NestJS".to_string(),
            "Prisma".to_string(),
            "TypeORM".to_string(),
            "RESTful API".to_string(),
            "SOAP".to_string(),
            "Docker".to_string(),
            "Kubernetes".to_string(),
            "AWS".to_string(),
            "Google Cloud".to_string(),
            "Azure".to_string(),
        ],
    };
    skill_repo.create(backend_skills).await?;

    let tools_skills = Skill {
        category: "DevOps & Tools".to_string(),
        items: vec![
            "Firebase".to_string(),
            "GitHub Actions".to_string(),
            "Jenkins".to_string(),
            "Storybook".to_string(),
            "Pipelines".to_string(),
            "Continuous Integration (CI)".to_string(),
            "Continuous Deployment (CD)".to_string(),
        ],
    };
    skill_repo.create(tools_skills).await?;

    let mobile_iot_skills = Skill {
        category: "Mobile & IoT".to_string(),
        items: vec![
            "Android".to_string(),
            "iOS".to_string(),
            "Kotlin".to_string(),
            "Java".to_string(),
            "BLE".to_string(),
            "Push Notifications".to_string(),
            "ESP32".to_string(),
            "FFmpeg".to_string(),
        ],
    };
    skill_repo.create(mobile_iot_skills).await?;

    let agile_skills = Skill {
        category: "Agile & Management".to_string(),
        items: vec![
            "Kanban".to_string(),
            "Scrum".to_string(),
            "Agile Coaching".to_string(),
            "Team Building".to_string(),
            "Risk Management".to_string(),
            "Capacity Management".to_string(),
            "Team Metrics".to_string(),
        ],
    };
    skill_repo.create(agile_skills).await?;

    // Seed experiences data
    println!("Seeding experiences data...");
    
    let bairesdev_exp = Experience::new(
        "BairesDev".to_string(),
        "Senior Software Engineer".to_string(),
        "2023-12".to_string(),
        None, // Present
        "Working for the Hensall Coorp client, developing both frontend and backend solutions. For the Lenslock client, developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%.".to_string(),
        vec![
            "React".to_string(),
            "Redux".to_string(),
            "Styled Components".to_string(),
            "NestJS".to_string(),
            "TypeORM".to_string(),
            "Prisma".to_string(),
            "Jest".to_string(),
            "GitHub Actions".to_string(),
            "Docker".to_string(),
            "Storybook".to_string(),
            "Typescript".to_string(),
        ],
        vec![
            "Built and maintained CI/CD pipelines using GitHub Actions, automating testing, linting, and deployments".to_string(),
            "Leveraged containerization for local development environments, including databases and auxiliary services".to_string(),
            "Developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%".to_string(),
        ],
    );
    experience_repo.create(bairesdev_exp).await?;

    let mtst_exp = Experience::new(
        "MTST Technology Center".to_string(),
        "Coordinator Developer".to_string(),
        "2021-08".to_string(),
        None, // Present (Volunteer)
        "Designed and implemented a video and image processing service in Rust, using FFmpeg to manipulate videos and audio codecs. Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects.".to_string(),
        vec![
            "Rust".to_string(),
            "FFmpeg".to_string(),
            "ESP32".to_string(),
            "React Native".to_string(),
            "Docker".to_string(),
            "Kubernetes".to_string(),
            "IoT".to_string(),
        ],
        vec![
            "Designed and implemented a video and image processing service in Rust, using FFmpeg to manipulate videos and audio codecs".to_string(),
            "Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects".to_string(),
            "Introduced CI/CD workflows using Docker and Kubernetes, streamlining deployments and improving efficiency".to_string(),
        ],
    );
    experience_repo.create(mtst_exp).await?;

    let ab_inbev_exp = Experience::new(
        "AB InBev".to_string(),
        "Agilist".to_string(),
        "2022-03".to_string(),
        Some("2023-11".to_string()),
        "Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements, increasing customer engagement by 35%. Implemented Kanban methodology, improving task visibility and team productivity by 150%.".to_string(),
        vec![
            "Kanban".to_string(),
            "Agile".to_string(),
            "Scrum".to_string(),
            "Team Management".to_string(),
            "Feature Activation".to_string(),
            "Customer Engagement".to_string(),
        ],
        vec![
            "Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements".to_string(),
            "Increased customer engagement by 35% through strategic feature releases".to_string(),
            "Implemented Kanban methodology, improving task visibility and team productivity by 150%".to_string(),
        ],
    );
    experience_repo.create(ab_inbev_exp).await?;

    let para_chegar_exp = Experience::new(
        "ParaChegar".to_string(),
        "Mobile Developer".to_string(),
        "2023-01".to_string(),
        Some("2023-09".to_string()),
        "Developed intuitive and user-friendly React Native applications. Integrated native modules for push notifications and location-based services, enhancing app functionality.".to_string(),
        vec![
            "React Native".to_string(),
            "Push Notifications".to_string(),
            "Location Services".to_string(),
            "Mobile Development".to_string(),
            "UI/UX".to_string(),
            "Native Modules".to_string(),
        ],
        vec![
            "Developed intuitive and user-friendly React Native applications".to_string(),
            "Integrated native modules for push notifications and location-based services".to_string(),
            "Enhanced app functionality with custom native extensions".to_string(),
        ],
    );
    experience_repo.create(para_chegar_exp).await?;

    let eldorado_exp = Experience::new(
        "Eldorado Research Institute".to_string(),
        "Developer list".to_string(),
        "2018-04".to_string(),
        Some("2022-02".to_string()),
        "Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times. Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS. Designed and implemented continuous integration pipelines, reducing deployment time by 50%.".to_string(),
        vec![
            ".NET".to_string(),
            "Docker".to_string(),
            "AWS".to_string(),
            "Agile".to_string(),
            "B2B".to_string(),
            "B2C".to_string(),
            "CI/CD".to_string(),
            "Backend".to_string(),
            "Frontend".to_string(),
        ],
        vec![
            "Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times".to_string(),
            "Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS".to_string(),
            "Designed and implemented continuous integration pipelines, reducing deployment time by 50%".to_string(),
        ],
    );
    experience_repo.create(eldorado_exp).await?;

    let sidia_exp = Experience::new(
        "SIDIA - Samsung Institute".to_string(),
        "QA Analyst".to_string(),
        "2015-11".to_string(),
        Some("2018-03".to_string()),
        "Conducted QA testing for Samsung products.".to_string(),
        vec![
            "QA Testing".to_string(),
            "Mobile Testing".to_string(),
            "Samsung Products".to_string(),
            "Quality Assurance".to_string(),
        ],
        vec![
            "Conducted comprehensive QA testing for Samsung products".to_string(),
            "Ensured product quality through rigorous testing methodologies".to_string(),
        ],
    );
    experience_repo.create(sidia_exp).await?;

    let indt_exp = Experience::new(
        "INDT (Nokia Institute)".to_string(),
        "Junior Developer".to_string(),
        "2015-05".to_string(),
        Some("2015-10".to_string()),
        "Automated hardware and software testing processes.".to_string(),
        vec![
            "Automated Testing".to_string(),
            "Hardware Testing".to_string(),
            "Software Testing".to_string(),
            "Test Automation".to_string(),
        ],
        vec![
            "Automated hardware and software testing processes".to_string(),
            "Improved testing efficiency through automation".to_string(),
        ],
    );
    experience_repo.create(indt_exp).await?;

    // Seed GitHub stats
    println!("Seeding GitHub stats data...");
    let github_stats = GithubStats {
        username: "kauefontes".to_string(),
        repo_count: 35,
        followers: 120,
        contributions: 980,
        top_languages: vec![
            TopLanguage {
                name: "Rust".to_string(),
                percentage: 35,
            },
            TopLanguage {
                name: "TypeScript".to_string(),
                percentage: 30,
            },
            TopLanguage {
                name: "JavaScript".to_string(),
                percentage: 20,
            },
            TopLanguage {
                name: "C#".to_string(),
                percentage: 10,
            },
            TopLanguage {
                name: "Other".to_string(),
                percentage: 5,
            },
        ],
        recent_activity: vec![
            RecentActivity {
                date: "2025-05-15".to_string(),
                message: "Implemented video processing service for the MTST Technology Center".to_string(),
                repo: "kauefontes/video-processor-rust".to_string(),
            },
            RecentActivity {
                date: "2025-05-10".to_string(),
                message: "Added React component library for Lenslock project".to_string(),
                repo: "kauefontes/lenslock-components".to_string(),
            },
            RecentActivity {
                date: "2025-05-05".to_string(),
                message: "Updated IoT dashboard for urban gardening project".to_string(),
                repo: "kauefontes/urban-garden-dashboard".to_string(),
            },
            RecentActivity {
                date: "2025-05-01".to_string(),
                message: "Fixed CI/CD pipeline for BairesDev project".to_string(),
                repo: "kauefontes/hensall-backend".to_string(),
            },
            RecentActivity {
                date: "2025-04-25".to_string(),
                message: "Added Rust documentation for backend service".to_string(),
                repo: "kauefontes/rust-backend-service".to_string(),
            },
        ],
    };
    github_stats_repo.create(github_stats).await?;

    println!("Seed data loaded successfully!");
    Ok(())
}
