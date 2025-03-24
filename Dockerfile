# ---- Base Stage ----
FROM rust:1.82-bookworm AS base
WORKDIR /workspace

# Install required dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# ---- Dependencies Caching Stage ----
FROM base AS deps
WORKDIR /workspace/

# Copy only Cargo files for efficient caching
COPY api/Cargo.toml api/Cargo.lock ./api/
COPY Cargo.toml Cargo.lock ./

# Create a fake source file to allow dependency resolution
RUN mkdir -p api/src && echo "fn main() {}" > api/src/main.rs && echo "" > api/src/lib.rs

# Fetch dependencies and build only dependencies layer
RUN cargo build --bin comhairle_api --release && rm -rf target/release/deps

# ---- Build Stage ----
FROM base AS build
WORKDIR /workspace/api

# Copy the entire monorepo but ignore unnecessary files via .dockerignore
COPY . /workspace

# Ensure dependencies from previous stage are used
COPY --from=deps /workspace/target /workspace/target
COPY --from=deps /workspace/Cargo.lock /workspace/Cargo.lock
COPY --from=deps /workspace/api/Cargo.lock /workspace/api/Cargo.lock

# Compile the comhairle_api crate
RUN cargo build --release --package comhairle_api

# ---- Production Stage ----
FROM debian:bookworm-slim AS production
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage
COPY --from=build /workspace/target/release/comhairle_api .

# Expose the service port (adjust if needed)
EXPOSE 3000

# Set non-root user for security
RUN useradd -m rustuser
USER rustuser

# Run the compiled binary
CMD ["./comhairle_api"]
