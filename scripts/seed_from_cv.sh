#!/bin/bash
# Script to seed the database with CV data using the API

# Base URL
BASE_URL="http://localhost:8080"
TOKEN=""

echo "=================== Seeding DB from CV ==================="

# Login to get JWT token
echo "Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "kauefontes@outlook.com",
    "password": "MT$MT$T4lutaEPRAV4LER"
  }')

# Extract token
TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*' | sed 's/"token":"//')

if [ -z "$TOKEN" ]; then
  echo "Error: Failed to authenticate. Response: $LOGIN_RESPONSE"
  exit 1
fi

echo "Successfully authenticated."

# Create/Update Profile
echo "Creating profile..."
curl -s -X PUT "${BASE_URL}/profile" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "bio": [
      "Accomplished software engineer with over 9 years of experience specializing in frontend and backend development.",
      "Expert in React, Next.js, Typescript, and Rust, with a proven track record of leading cross-functional teams and driving agile practices.",
      "Demonstrated success in delivering high-quality software solutions, improving performance metrics, and streamlining workflows with modern cloud and containerization technologies.",
      "Ready to leverage my expertise and leadership skills to solve complex technical challenges."
    ],
    "social_links": [
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
    ],
    "education": [
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
    ],
    "languages": [
      {
        "name": "Portuguese",
        "level": "Native"
      },
      {
        "name": "English",
        "level": "Fluent"
      }
    ]
  }'

# Create Skills
echo "Creating frontend skills..."
curl -s -X POST "${BASE_URL}/skills" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "category": "Frontend",
    "items": [
      "React Native",
      "React",
      "Next.js",
      "Typescript",
      "Javascript",
      "HTML",
      "CSS",
      "Redux",
      "Redux Toolkit",
      "MobX",
      "Hooks",
      "Design Patterns"
    ]
  }'

echo "Creating backend skills..."
curl -s -X POST "${BASE_URL}/skills" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "category": "Backend",
    "items": [
      "Rust",
      ".NET",
      "NodeJS",
      "NestJS",
      "Prisma",
      "TypeORM",
      "RESTful API",
      "SOAP",
      "Docker",
      "Kubernetes",
      "AWS",
      "Google Cloud",
      "Azure"
    ]
  }'

echo "Creating DevOps & Tools skills..."
curl -s -X POST "${BASE_URL}/skills" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "category": "DevOps & Tools",
    "items": [
      "Firebase",
      "GitHub Actions",
      "Jenkins",
      "Storybook",
      "Pipelines",
      "Continuous Integration (CI)",
      "Continuous Deployment (CD)"
    ]
  }'

echo "Creating Mobile & IoT skills..."
curl -s -X POST "${BASE_URL}/skills" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "category": "Mobile & IoT",
    "items": [
      "Android",
      "iOS",
      "Kotlin",
      "Java",
      "BLE",
      "Push Notifications",
      "ESP32",
      "FFmpeg"
    ]
  }'

echo "Creating Agile & Management skills..."
curl -s -X POST "${BASE_URL}/skills" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "category": "Agile & Management",
    "items": [
      "Kanban",
      "Scrum",
      "Agile Coaching",
      "Team Building",
      "Risk Management",
      "Capacity Management",
      "Team Metrics"
    ]
  }'

# Create Experiences
echo "Creating experiences..."

echo "Creating BairesDev experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "BairesDev",
    "position": "Senior Software Engineer",
    "start_date": "2023-12",
    "end_date": null,
    "description": "Working for the Hensall Coorp client, developing both frontend and backend solutions. For the Lenslock client, developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%.",
    "technologies": [
      "React",
      "Redux",
      "Styled Components",
      "NestJS",
      "TypeORM",
      "Prisma",
      "Jest",
      "GitHub Actions",
      "Docker",
      "Storybook",
      "Typescript"
    ],
    "highlights": [
      "Built and maintained CI/CD pipelines using GitHub Actions, automating testing, linting, and deployments",
      "Leveraged containerization for local development environments, including databases and auxiliary services",
      "Developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%"
    ]
  }'

echo "Creating MTST experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "MTST Technology Center",
    "position": "Coordinator Developer",
    "start_date": "2021-08",
    "end_date": null,
    "description": "Designed and implemented a video and image processing service in Rust, using FFmpeg to manipulate videos and audio codecs. Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects.",
    "technologies": [
      "Rust",
      "FFmpeg",
      "ESP32",
      "React Native",
      "Docker",
      "Kubernetes",
      "IoT"
    ],
    "highlights": [
      "Designed and implemented a video and image processing service in Rust, using FFmpeg to manipulate videos and audio codecs",
      "Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects",
      "Introduced CI/CD workflows using Docker and Kubernetes, streamlining deployments and improving efficiency"
    ]
  }'

echo "Creating AB InBev experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "AB InBev",
    "position": "Agilist",
    "start_date": "2022-03",
    "end_date": "2023-11",
    "description": "Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements, increasing customer engagement by 35%. Implemented Kanban methodology, improving task visibility and team productivity by 150%.",
    "technologies": [
      "Kanban",
      "Agile",
      "Scrum",
      "Team Management",
      "Feature Activation",
      "Customer Engagement"
    ],
    "highlights": [
      "Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements",
      "Increased customer engagement by 35% through strategic feature releases",
      "Implemented Kanban methodology, improving task visibility and team productivity by 150%"
    ]
  }'

echo "Creating ParaChegar experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "ParaChegar",
    "position": "Mobile Developer",
    "start_date": "2023-01",
    "end_date": "2023-09",
    "description": "Developed intuitive and user-friendly React Native applications. Integrated native modules for push notifications and location-based services, enhancing app functionality.",
    "technologies": [
      "React Native",
      "Push Notifications",
      "Location Services",
      "Mobile Development",
      "UI/UX",
      "Native Modules"
    ],
    "highlights": [
      "Developed intuitive and user-friendly React Native applications",
      "Integrated native modules for push notifications and location-based services",
      "Enhanced app functionality with custom native extensions"
    ]
  }'

echo "Creating Eldorado Research Institute experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "Eldorado Research Institute",
    "position": "Developer list",
    "start_date": "2018-04",
    "end_date": "2022-02",
    "description": "Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times. Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS. Designed and implemented continuous integration pipelines, reducing deployment time by 50%.",
    "technologies": [
      ".NET",
      "Docker",
      "AWS",
      "Agile",
      "B2B",
      "B2C",
      "CI/CD",
      "Backend",
      "Frontend"
    ],
    "highlights": [
      "Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times",
      "Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS",
      "Designed and implemented continuous integration pipelines, reducing deployment time by 50%"
    ]
  }'

echo "Creating SIDIA experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "SIDIA - Samsung Institute",
    "position": "QA Analyst",
    "start_date": "2015-11",
    "end_date": "2018-03",
    "description": "Conducted QA testing for Samsung products.",
    "technologies": [
      "QA Testing",
      "Mobile Testing",
      "Samsung Products",
      "Quality Assurance"
    ],
    "highlights": [
      "Conducted comprehensive QA testing for Samsung products",
      "Ensured product quality through rigorous testing methodologies"
    ]
  }'

echo "Creating INDT experience..."
curl -s -X POST "${BASE_URL}/experiences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "company": "INDT (Nokia Institute)",
    "position": "Junior Developer",
    "start_date": "2015-05",
    "end_date": "2015-10",
    "description": "Automated hardware and software testing processes.",
    "technologies": [
      "Automated Testing",
      "Hardware Testing",
      "Software Testing",
      "Test Automation"
    ],
    "highlights": [
      "Automated hardware and software testing processes",
      "Improved testing efficiency through automation"
    ]
  }'

# Create GitHub Stats
echo "Creating GitHub stats..."
curl -s -X POST "${BASE_URL}/github-stats" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "username": "kauefontes",
    "repo_count": 35,
    "followers": 120,
    "contributions": 980,
    "top_languages": [
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
    ],
    "recent_activity": [
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
    ]
  }'

echo "=================== Seed completed ==================="
echo "The database has been successfully populated with your CV data."
