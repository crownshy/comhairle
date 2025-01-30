# ---- Build Stage ----
FROM rust:1.74 AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies (this helps speed up rebuilds)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs  # Fake main to cache dependencies
RUN cargo build api --release && rm -rf src

# Copy project files
COPY . .
RUN cargo build api --release

# ---- Production Stage ----
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install runtime dependencies (for OpenSSL if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/api /usr/local/bin/api

# Set environment variables
ENV PORT=3000
EXPOSE 3000

# Run the Axum application
CMD ["/usr/local/bin/api"]

