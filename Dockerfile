# ---- Chef Stage ----
FROM rust:1.92-bookworm AS chef
WORKDIR /workspace

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef --locked

# ---- Planner Stage ----
FROM chef AS planner
WORKDIR /workspace

# Copy all Cargo files to generate recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- Builder Stage ----
FROM chef AS builder
WORKDIR /workspace

# Install required dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the recipe and build dependencies
COPY --from=planner /workspace/recipe.json recipe.json

# Build dependencies - this layer is cached unless dependencies change
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/workspace/target \
    cargo chef cook --release --recipe-path recipe.json --package comhairle_api && \
    # Copy the built dependencies out of the cache mount
    mkdir -p /tmp/target && \
    cp -r target/release/deps /tmp/target/ && \
    cp -r target/release/build /tmp/target/ 2>/dev/null || true

# Copy source code
COPY . .

# Restore built dependencies
RUN cp -r /tmp/target/* target/release/ 2>/dev/null || true

# Build the application
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --package comhairle_api

# ---- Production Stage ----
FROM debian:bookworm-slim AS production
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /workspace/target/release/comhairle_api .

# Expose the service port (adjust if needed)
EXPOSE 3000

# Set non-root user for security
RUN useradd -m rustuser
USER rustuser

# Run the compiled binary
CMD ["./comhairle_api"]
