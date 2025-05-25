# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install required dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false -m -d /var/lib/ask ask

# Copy the binary from builder stage
COPY --from=builder /app/target/release/ask /usr/local/bin/ask

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/ask

# Switch to non-root user
USER ask
WORKDIR /var/lib/ask

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ask --version || exit 1

ENTRYPOINT ["ask"]
CMD ["--help"] 