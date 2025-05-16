# Contributing to Retro Quewui Backend

Thank you for considering contributing to Retro Quewui Backend! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

Please be respectful and inclusive when contributing to this project. We want to create a welcoming environment for everyone.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/retro-quewui-backend.git`
3. Install Rust and Cargo if you haven't already: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
4. Create a branch for your changes: `git checkout -b feature/your-feature-name`

## Development Workflow

1. Make sure to run `cargo run` to start the development server
2. Make your changes
3. Ensure your code follows the project's style guidelines
4. Write tests for your changes if applicable
5. Run `cargo clippy` to check for any linting issues
6. Run `cargo test` to run the test suite
7. Commit your changes using conventional commit messages (see below)
8. Push your changes to your fork
9. Open a pull request

## Commit Message Guidelines

We follow conventional commits for our commit messages. Each commit message should have a structured format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Changes that don't affect code functionality (formatting, etc.)
- `refactor`: Code changes that neither fix a bug nor add a feature
- `perf`: Performance improvements
- `test`: Adding or fixing tests
- `chore`: Changes to build process or auxiliary tools

Example:
```
feat(routes): add profile endpoint

Implement GET /profile endpoint to fetch user profile data
Close #123
```

## Pull Request Process

1. Update the README.md with details of changes if appropriate
2. Update documentation as needed
3. The PR should work on the main development branch
4. Your PR will be reviewed by maintainers who may request changes
5. Once approved, your PR will be merged

## Adding New Endpoints

When adding new endpoints to the API:
1. Create a model in the `models` directory
2. Create a route handler in the `routes` directory
3. Register the route in `routes/mod.rs`
4. Add the route configuration in `main.rs`
5. Update the README.md with the new endpoint information

## License

By contributing to Retro Quewui Backend, you agree that your contributions will be licensed under the project's MIT license.
