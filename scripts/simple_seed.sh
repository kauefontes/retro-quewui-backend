#!/bin/bash
# Simple script to seed database

# Base URL
BASE_URL="http://localhost:8080"

# Get credentials from environment variables or use defaults
USER_EMAIL=${USER_EMAIL:-"admin"}
USER_PASSWORD=${USER_PASSWORD:-"admin"}

# Login to get JWT token
echo "Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "'${USER_EMAIL}'",
    "password": "'${USER_PASSWORD}'"
  }')

# Extract token
TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*' | sed 's/"token":"//')

if [ -z "$TOKEN" ]; then
  echo "Error: Failed to authenticate. Response: $LOGIN_RESPONSE"
  exit 1
fi

echo "Successfully authenticated."

# Function to POST with error checking
post_data() {
  local endpoint=$1
  local data=$2
  local response

  echo "Sending to $endpoint..."
  response=$(curl -s -X POST "${BASE_URL}${endpoint}" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "$data")

  echo "Response: $response"
  echo ""
}

# Function to PUT with error checking
put_data() {
  local endpoint=$1
  local data=$2
  local response

  echo "Sending to $endpoint..."
  response=$(curl -s -X PUT "${BASE_URL}${endpoint}" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "$data")

  echo "Response: $response"
  echo ""
}

echo "=================== Starting Simple Seed ==================="

# Update Profile
PROFILE_DATA='{
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
      "degree": "Bachelor'\''s in Systems Analysis and Development",
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

put_data "/profile" "$PROFILE_DATA"

# Create Frontend Skills
FRONTEND_SKILLS='{
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

post_data "/skills" "$FRONTEND_SKILLS"

# Create Backend Skills
BACKEND_SKILLS='{
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

post_data "/skills" "$BACKEND_SKILLS"

# Create DevOps & Tools Skills
DEVOPS_SKILLS='{
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

post_data "/skills" "$DEVOPS_SKILLS"

# Create Mobile & IoT Skills
MOBILE_SKILLS='{
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

post_data "/skills" "$MOBILE_SKILLS"

# Create Agile & Management Skills
AGILE_SKILLS='{
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

post_data "/skills" "$AGILE_SKILLS"

# Create BairesDev Experience
BAIRESDEV_EXP='{
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

post_data "/experiences" "$BAIRESDEV_EXP"

# Create MTST Experience
MTST_EXP='{
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

post_data "/experiences" "$MTST_EXP"

# Create AB InBev Experience
ABINBEV_EXP='{
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

post_data "/experiences" "$ABINBEV_EXP"

# Create ParaChegar Experience
PARACHEGAR_EXP='{
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

post_data "/experiences" "$PARACHEGAR_EXP"

# Create Eldorado Experience
ELDORADO_EXP='{
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

post_data "/experiences" "$ELDORADO_EXP"

# Create SIDIA Experience
SIDIA_EXP='{
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

post_data "/experiences" "$SIDIA_EXP"

# Create INDT Experience
INDT_EXP='{
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

post_data "/experiences" "$INDT_EXP"

# Create GitHub Stats
GITHUB_STATS='{
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

post_data "/github-stats" "$GITHUB_STATS"

echo "=================== Seed completed ==================="
echo "Verifying data..."

# Verify data was uploaded
echo "Checking profile..."
curl -s "${BASE_URL}/profile"
echo ""

echo "Checking skills..."
curl -s "${BASE_URL}/skills"
echo ""

echo "Checking experiences..."
curl -s "${BASE_URL}/experiences"
echo ""

echo "Checking GitHub stats..."
curl -s "${BASE_URL}/github-stats"
echo ""
