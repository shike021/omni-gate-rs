# Multi-stage build to reduce image size
FROM rust:1.92.0 as builder

WORKDIR /usr/src/app
COPY . .

# Install dependencies and build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/omni-gate-rs /app/omni-gate-rs

# Copy configuration file if it exists (optional)
COPY config.toml /app/config.toml

EXPOSE 3000
EXPOSE 5000

CMD ["./omni-gate-rs"]