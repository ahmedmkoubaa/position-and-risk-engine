# ==========================================
# Multi-Stage Dockerfile for Rust Risk Engine
# ==========================================

# ------------------------------------------
# Stage 1: Build & Compile Binary
# ------------------------------------------
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code and tests
COPY src ./src
COPY tests ./tests
COPY static ./static

# Compile optimized release binary
RUN cargo build --release

# ------------------------------------------
# Stage 2: Minimal & Secure Runtime Image
# ------------------------------------------
FROM debian:bookworm-slim AS runtime

# Install basic SSL/TLS certificates and clean apt cache
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-privileged user for container security
RUN groupadd -r appgroup && useradd -r -g appgroup -d /app appuser

WORKDIR /app

# Copy compiled binary and static web assets from builder
COPY --from=builder /app/target/release/position-and-risk-engine /app/position-and-risk-engine
COPY --from=builder /app/static /app/static

# Set permissions
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Configure runtime environment
ENV PORT=3000
ENV RUST_LOG="position_and_risk_engine=info,tower_http=info"
EXPOSE 3000

# Run the binary
ENTRYPOINT ["/app/position-and-risk-engine"]
