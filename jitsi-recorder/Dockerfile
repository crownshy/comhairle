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

# nice-gst-meet's Candidate::addr() reads sockaddr_in's sin_addr/sin_port
# fields - which the kernel stores in network (big-endian) byte order -
# directly into Rust's SocketAddrV4::new() without converting from network
# to host byte order first. On any little-endian host (effectively all of
# them) this silently reverses the octets of our own local candidate before
# it's embedded in the Jingle session-accept IQ sent to JVB, e.g. our real
# pod IP 192.168.75.33 gets advertised as 33.75.168.192. JVB then sends RTP
# to a bogus address and the recorder receives zero media, with no error
# anywhere - this is why recorded calls joined/stayed connected but nothing
# was ever recorded.
RUN perl -0777 -pi -e 's/self\.inner\.addr\.s\.ip4\.sin_addr\.s_addr\.into\(\),\n(\s*)self\.inner\.addr\.s\.ip4\.sin_port,/std::net::Ipv4Addr::from(u32::from_be(self.inner.addr.s.ip4.sin_addr.s_addr)),\n$1u16::from_be(self.inner.addr.s.ip4.sin_port),/' nice-gst-meet/src/candidate.rs

# The CLI's main() creates the GLib MainLoop but doesn't start running it
# (task::spawn_blocking(move || main_loop.run())) until the very end, after
# conference.join() (which spawns the ICE/Jingle setup), the colibri sends,
# and set_pipeline_state - all awaited sequentially first. libnice's
# connectivity-check retransmission timers are GLib timeout sources on that
# same MainContext, so until the loop actually starts iterating, nothing
# timer-driven ever fires even though synchronous state-change callbacks
# (emitted directly in the ICE agent's C call stack) still appear immediately.
# Observed: ICE state reaches "connecting" right away but never progresses
# to connected/ready, and zero STUN/RTP packets are ever sent. Start the
# loop running immediately after creating it instead of at the end.
RUN perl -0777 -pi -e 's/(let main_loop = glib::MainLoop::new\(None, false\);\n)/$1\n  let main_loop_handle = {\n    let main_loop = main_loop.clone();\n    task::spawn_blocking(move || main_loop.run())\n  };\n/' gst-meet/src/main.rs
RUN perl -0777 -pi -e 's/task::spawn_blocking\(move \|\| main_loop\.run\(\)\)\.await\?;/main_loop_handle.await?;/' gst-meet/src/main.rs

# Bisection diagnostic: confirm whether main() ever actually reaches the
# set_pipeline_state(Playing) call site at all, vs hanging somewhere earlier
# (e.g. a lock held across the ~9-12s Colibri connection-attempt-and-fail
# cycle, since both the explicit CLI-level send_colibri_message call and the
# library's own automatic Colibri setup lock the same jingle_session mutex).
RUN perl -0777 -pi -e 's/(  conference\n    \.set_pipeline_state\(gstreamer::State::Playing\))/  warn!("about to call set_pipeline_state(Playing)");\n$1/' gst-meet/src/main.rs

# set_pipeline_state() requests the state change via call_async and returns
# immediately - it never confirms the pipeline actually reaches PLAYING.
# nicesrc/nicesink are live sources, and the dynamically-built RTP receive
# chain (rtpbin, depayloaders added on pad-added) can get stuck mid-transition
# (e.g. waiting on preroll that a live source never truly provides) without
# raising any error - set_state() returning ASYNC is not an error, just "in
# progress," and nothing here ever waits for or logs the actual outcome.
# Observed: ICE/STUN fully succeeds, pipeline construction completes, but
# zero DTLS handshake packets are ever sent - consistent with the pipeline
# never truly reaching PLAYING. Block briefly on the real state after
# requesting it, both as a diagnostic and because polling state is itself
# often what unblocks a stuck async transition.
RUN perl -0777 -pi -e 's/(self\.pipeline\(\)\.await\?\.call_async\(move \|p\| \{\n\s*if let Err\(e\) = p\.set_state\(state\) \{\n\s*error!\("pipeline set_state: \{:\?\}", e\);\n\s*\}\n)(\s*)\}\);/$1$2  let (result, current, pending) = p.state(Some(gstreamer::ClockTime::from_seconds(10)));\n$2  warn!("pipeline state after set_state({state:?}): result={result:?} current={current:?} pending={pending:?}");\n$2});/' lib-gst-meet/src/conference.rs

# lib-gst-meet's ICE component-state-changed signal is defined in the nice
# bindings but never wired up anywhere, so there is zero log visibility into
# whether ICE connectivity checks ever progress past gathering (state 0 =
# disconnected, 1 = gathering, 2 = connecting, 3 = connected, 4 = ready,
# 5 = failed, per libnice's NiceComponentState). Diagnostic-only addition -
# logs state transitions, doesn't change any behavior.
RUN perl -0777 -pi -e 's/(ice_agent\.connect_candidate_gathering_done\(move \|_agent, candidates\| \{\n\s*debug!\("ICE candidate-gathering-done \{:\?\}", candidates\);\n\s*\}\);\n)/$1\n    ice_agent.connect_component_state_changed(move |_agent, stream_id, component_id, state| {\n      warn!("ICE component state changed: stream={} component={} state={}", stream_id, component_id, state);\n    });\n/' lib-gst-meet/src/jingle.rs

# lib-gst-meet *also* opens the Colibri channel automatically right after
# every session-accept (lib-gst-meet/src/conference.rs, in the XMPP read
# loop's IQ-result handler) with the same unconditional `?`, independent of
# the CLI-level call above - this is the one that actually kills every
# session a short way in. Make it non-fatal: log and skip the
# colibri-dependent setup (video type message, periodic stats) for this IQ
# instead of propagating the error out of the read loop.
#
# CRITICAL: the unconditional `connected_tx.send(())` that unblocks
# `JitsiConference::join()`'s `rx.await?` sits *after* this whole colibri
# setup block in the original code - an early `return Ok(())` on failure
# (as a naive fix would do) skips it entirely, so join() - and therefore
# all of main(), including set_pipeline_state - hangs forever. Fire that
# same signal here before returning so join() still unblocks. Confirmed via
# diagnostic: ICE/STUN succeeded fully, yet main() never even reached its
# set_pipeline_state call site - this early-return skipping connected_tx was
# the actual reason zero media/DTLS ever happened, not a GStreamer/DTLS bug.
RUN perl -0777 -pi -e 's/let colibri_channel =\s*\n(\s*)ColibriChannel::new\(&colibri_url, self\.tls_insecure\)\.await\?;/let colibri_channel = match ColibriChannel::new(&colibri_url, self.tls_insecure).await {\n$1  Ok(c) => c,\n$1  Err(e) => {\n$1    warn!("Failed to connect Colibri WebSocket, continuing without it: {e:?}");\n$1    if let Some(connected_tx) = self.inner.lock().await.connected_tx.take() {\n$1      connected_tx.send(()).ok();\n$1    }\n$1    return Ok(());\n$1  },\n$1};/' lib-gst-meet/src/conference.rs

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
