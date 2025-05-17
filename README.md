# Retro Quewui Backend

A production-ready Rust API backend for the retro-tech style "Quewui" portfolio.

## Features

- **RESTful API**: Clean and consistent API endpoints
- **Database Integration**: SQLite with SQLx for type-safe queries
- **Authentication**: JWT-based authentication system
- **Error Handling**: Centralized error handling with custom error types
- **Validation**: Request validation with helpful error messages
- **Logging**: Structured logging with configurable levels
- **Health Checks**: Endpoint for monitoring application health
- **Docker Support**: Containerization for easy deployment
- **Testing**: Unit and integration tests
- **Documentation**: Comprehensive API documentation

## Project Structure

```
src/
├── auth/           # Authentication system
├── config/         # Configuration management
├── error/          # Error handling
├── models/         # Data models and repositories
├── routes/         # API routes
├── tests/          # Tests
├── validation/     # Input validation
└── main.rs         # Application entry point
```

## API Routes

- `GET /health` - Health check endpoint
- `POST /auth/login` - Authenticate and get JWT token
- `GET /admin/dashboard` - Protected admin dashboard (requires authentication)
- `GET /projects` - List all projects
- `GET /projects/{id}` - Get a specific project
- `GET /experiences` - List all professional experiences
- `GET /experiences/{id}` - Get a specific experience
- `GET /skills` - List all skills
- `GET /posts` - List all blog posts
- `GET /posts/{id}` - Get a specific post
- `GET /github-stats` - Get GitHub statistics
- `GET /profile` - Get profile information
- `POST /contact` - Submit contact form data

## Requirements

- Rust 1.75 or higher
- Cargo (Rust package manager)
- SQLite (for development)

## Installation and Setup

### Local Development

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/retro-quewui-backend.git
   cd retro-quewui-backend
   ```

2. Copy the example environment file:

   ```bash
   cp .env.example .env
   ```

3. Edit the `.env` file to configure your environment.

4. Compile and run in development mode:

   ```bash
   cargo run
   ```

### Docker Deployment

1. Build and run using Docker Compose:

   ```bash
   docker-compose up -d
   ```

2. The API will be available at `http://localhost:8080`.

## Testing

Run the test suite:

```bash
cargo test
```

## Environment Variables

Configure the following variables in the `.env` file:

- `HOST` - Server IP address (default: 127.0.0.1)
- `PORT` - API port (default: 8080)
- `DATABASE_URL` - SQLite database URL (default: sqlite:./data.db)
- `FRONTEND_URL` - Frontend URL for CORS configuration (default: http://localhost:5173)
- `RUST_LOG` - Log level (default: info)
- `JWT_SECRET` - Secret key for JWT token generation

## Production Deployment Checklist

Before deploying to production, ensure:

1. Set a strong `JWT_SECRET` in the environment
2. Configure proper CORS settings with your production frontend URL
3. Set up monitoring and logging
4. Configure a reverse proxy (like Nginx) for SSL termination
5. Set up database backups
6. Configure rate limiting if needed

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
