# Backend Dockerfile - Multi-stage build for Rust application
FROM rust:1.91 as builder

WORKDIR /app

# Install build dependencies for RocksDB/libclang
RUN apt-get update && apt-get install -y \
    clang \
    llvm-dev \
    libclang-dev \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY db ./db

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/shorturl /app/shorturl
COPY --from=builder /app/db /app/db

# Expose the port the app runs on
EXPOSE 8080

# Run the binary
CMD ["/app/shorturl"]
