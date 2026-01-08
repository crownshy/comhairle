# ---- Base Stage ----
FROM rust:1.83-bookworm AS base
WORKDIR /workspace

# Install required dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# ---- Dependencies Caching Stage ----
FROM base AS deps
WORKDIR /workspace/

# Copy only Cargo files for efficient caching
COPY api/Cargo.toml api/Cargo.lock ./api/
COPY comhairle_macros/Cargo.toml ./comhairle_macros/Cargo.toml
COPY adaptors/heyform-rust-sdk/Cargo.toml ./adaptors/heyform-rust-sdk/Cargo.toml
COPY adaptors/ragflow/Cargo.toml ./adaptors/ragflow/Cargo.toml
COPY Cargo.toml Cargo.lock ./

# Create dummy source files
RUN mkdir -p api/src && echo "fn main() {}" > api/src/main.rs && echo "" > api/src/lib.rs && \
    mkdir -p comhairle_macros/src && echo "" > comhairle_macros/src/lib.rs && \
    mkdir -p adaptors/heyform-rust-sdk/src && echo "" > adaptors/heyform-rust-sdk/src/lib.rs && \
	mkdir -p target/release

# Build dependencies only - keep the target directory intact
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/workspace/target \
    cargo build --bin comhairle_api --release

# ---- Build Stage ----
FROM base AS build
WORKDIR /workspace

# Copy source code
COPY . /workspace

# Copy cached dependencies and build
COPY --from=deps /workspace/target /workspace/target

# Build with cache mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --package comhairle_api

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
