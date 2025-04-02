# Builder stage
FROM rust:1.85 AS builder

# Install OpenSSL development packages for building
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/third_rust_app /usr/local/bin/

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/third_rust_app"]
