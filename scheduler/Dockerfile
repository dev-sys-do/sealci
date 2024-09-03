FROM rust:1.80.1 AS builder
# Stage 1: Build the application

# Set working directory (inside the container)
WORKDIR /sealci

# Copy only the Cargo files first to leverage Docker layer caching
COPY scheduler/ ./scheduler
COPY api/ ./api

# Fetch dependencies (to leverage caching)
WORKDIR /sealci/scheduler
RUN cargo fetch && apt-get update && apt-get install -y protobuf-compiler

# Build the app
RUN cargo build --release


# Stage 2: Creating a smaller image with the compiled binary!
FROM ubuntu:22.04

# Necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary and necessary files from the builder stage
COPY --from=builder /sealci/scheduler/target/release/server /usr/local/bin/server
COPY --from=builder /sealci/api/proto /usr/local/api/proto

EXPOSE 5005

# Set the RUST_LOG environment variable
ENV RUST_LOG=debug

# Binary entry point
ENTRYPOINT ["server"]
