# Multi-stage Dockerfile for jisti-recorder
# Stage 1: Build gst-meet
FROM rustlang/rust:nightly-bookworm AS gst-meet-builder

# Install GStreamer development libraries and dependencies
RUN apt-get update && apt-get install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    gstreamer1.0-tools \
    libnice-dev \
    libsrtp2-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Clone and build gst-meet
WORKDIR /build
RUN git clone https://github.com/avstack/gst-meet.git
WORKDIR /build/gst-meet

# gst-meet's CLI unconditionally sends a ReceiverVideoConstraints Colibri
# message right after joining and bails the whole process (.await?) if that
# fails - irrelevant to an audio-only recv-pipeline, but fatal if the
# server's Colibri WebSocket is ever unavailable/broken (observed: JVB's
# bundled Jetty 500s every colibri-ws request with a classpath mismatch,
# NoSuchMethodError on AuthenticationState). Patch it to log and continue
# instead of dying, so recording doesn't hard-depend on that channel.
RUN sed -i '/send_colibri_message(ColibriMessage::ReceiverVideoConstraints/,/\.await?;/{s/\.await?;/.await.ok();/}' gst-meet/src/main.rs

# lib-gst-meet *also* opens the Colibri channel automatically right after
# every session-accept (lib-gst-meet/src/conference.rs, in the XMPP read
# loop's IQ-result handler) with the same unconditional `?`, independent of
# the CLI-level call above - this is the one that actually kills every
# session a short way in. Make it non-fatal: log and skip the
# colibri-dependent setup (video type message, periodic stats) for this IQ
# instead of propagating the error out of the read loop.
RUN perl -0777 -pi -e 's/let colibri_channel =\s*\n(\s*)ColibriChannel::new\(&colibri_url, self\.tls_insecure\)\.await\?;/let colibri_channel = match ColibriChannel::new(&colibri_url, self.tls_insecure).await {\n$1  Ok(c) => c,\n$1  Err(e) => {\n$1    warn!("Failed to connect Colibri WebSocket, continuing without it: {e:?}");\n$1    return Ok(());\n$1  },\n$1};/' lib-gst-meet/src/conference.rs

RUN cargo build --release --bin gst-meet

# Stage 2: Build jisti-recorder
FROM rustlang/rust:nightly-bookworm AS recorder-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml ./
COPY src ./src
COPY templates ./templates

# Build the application
RUN cargo build --release

# Stage 3: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    gstreamer1.0-tools \
    gstreamer1.0-nice \
    libnice10 \
    libsrtp2-1 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 recorder && \
    mkdir -p /tmp/jitsi-recordings && \
    chown -R recorder:recorder /tmp/jitsi-recordings

# Copy binaries from builders
COPY --from=gst-meet-builder /build/gst-meet/target/release/gst-meet /usr/local/bin/gst-meet
COPY --from=recorder-builder /app/target/release/jisti-recorder /usr/local/bin/jisti-recorder

# Set permissions
RUN chmod +x /usr/local/bin/gst-meet /usr/local/bin/jisti-recorder

# Switch to non-root user
USER recorder

# Set working directory
WORKDIR /home/recorder

# Expose port
EXPOSE 3001

# Environment variables (can be overridden)
ENV RUST_LOG=info \
    PORT=3001 \
    JITSI_WEBSOCKET_URL=wss://jitsi.comhairle.scot/xmpp-websocket \
    GST_MEET_PATH=/usr/local/bin/gst-meet \
    RECORDINGS_DIR=/tmp/jitsi-recordings

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/recordings || exit 1

# Run the application
CMD ["/usr/local/bin/jisti-recorder"]
