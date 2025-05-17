FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/retro-quewui-backend /app/retro-quewui-backend

# Copy migrations directory
COPY --from=builder /app/migrations /app/migrations

# Create a directory for the database
RUN mkdir -p /app/data

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=8080
ENV DATABASE_URL=sqlite:./data/data.db
ENV RUST_LOG=info

# Expose the port
EXPOSE 8080

# Run the application
CMD ["/app/retro-quewui-backend"]
