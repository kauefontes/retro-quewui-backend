# Retro Quewui Backend

Rust API backend for the retro-tech style "Quewui" portfolio.

## Technologies

- Rust
- Actix Web
- Serde (JSON serialization/deserialization)
- CORS for frontend integration

## Project Structure

```
src/
├── main.rs           # Application entry point
├── models/           # Data models
│   ├── contact.rs
│   ├── experience.rs
│   ├── github_stats.rs
│   ├── mod.rs
│   ├── post.rs
│   ├── profile.rs
│   ├── project.rs
│   └── skill.rs
└── routes/           # API routes
    ├── contact.rs
    ├── experiences.rs
    ├── github_stats.rs
    ├── mod.rs
    ├── posts.rs
    ├── profile.rs
    ├── projects.rs
    └── skills.rs
```

## API Routes

- `GET /projects` - List all projects
- `GET /projects/{id}` - Get a specific project
- `GET /experiences` - List all professional experiences
- `GET /experiences/{id}` - Get a specific experience
- `GET /skills` - List all skills
- `GET /posts` - List all blog posts
- `GET /posts/{id}` - Get a specific post
- `GET /github-stats` - Get GitHub statistics
- `GET /profile` - Get profile information (bio, social links, education, languages)
- `POST /contact` - Submit contact form data

## Requirements

- Rust 1.75 or higher
- Cargo (Rust package manager)

## Installation and Execution

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/retro-quewui-backend.git
   cd retro-quewui-backend
   ```

2. Compile and run in development mode:

   ```bash
   cargo run
   ```

3. Or compile in production mode:
   ```bash
   cargo build --release
   ./target/release/retro-quewui-backend
   ```

## Environment Variables

Configure the following variables in the `.env` file at the project root:

- `HOST` - Server IP address (default: 127.0.0.1)
- `PORT` - API port (default: 8080)
- `FRONTEND_URL` - Frontend URL for CORS configuration (default: http://localhost:5173)
- `RUST_LOG` - Log level (default: info)

## Development

For local development, the server will be available at:

```
http://localhost:8080
```

The backend currently uses mock data for development. In a future implementation, it can be connected to a database.
