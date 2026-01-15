# Build stage
FROM rust:1.86-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for caching
COPY contact-handler/Cargo.toml contact-handler/Cargo.lock ./contact-handler/

# Create dummy source to cache dependencies
RUN mkdir -p contact-handler/src && echo "fn main() {}" > contact-handler/src/main.rs
RUN cd contact-handler && cargo build --release && rm -rf src

# Copy actual source and rebuild
COPY contact-handler/src ./contact-handler/src
RUN touch contact-handler/src/main.rs && cd contact-handler && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/contact-handler/target/release/contact-handler /app/contact-handler

# Copy static website files
COPY index.html ./
COPY css ./css
COPY js ./js
COPY images ./images

# Copy config files (will be mounted as volumes in production)
COPY contact-handler/accounts.txt ./accounts.txt

# Create empty contacts.csv
RUN touch contacts.csv

EXPOSE 9000

ENV PORT=9000

CMD ["./contact-handler"]
