# Step 1: Build Stage
FROM rust:1.72 AS builder

# Set working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Step 2: Run Stage
FROM debian:bookworm-slim

# Install dependencies required to run the binary
RUN apt-get update && apt-get install -y \
    openssl libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*  # Clean up the apt cache

# Set working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/bless-network /app/bless-network

# Copy the configuration file (if needed)
COPY config.json /app/config.json

# Expose any ports the application uses (e.g., 30025)
EXPOSE 30025

# Command to run the application with the configuration file
CMD ["./bless-network", "--init", "/app/config.json"]
