FROM rust:1.80.1 AS builder
# Stage 1: Build the application

# Set working directory (inside the container)
WORKDIR /sealci

# Copy only the Cargo files first to leverage Docker layer caching
COPY monitor/Cargo.toml monitor/Cargo.lock ./
COPY monitor/src ./src

# Fetch dependencies (to leverage caching)
WORKDIR /sealci/monitor
RUN cargo fetch && apt-get update && apt-get install -y protobuf-compiler

# Build the app
RUN cargo build --release


# Stage 2: Creating a smaller image with the compiled binary!
FROM ubuntu:22.04

# Necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /sealci

# Copy compiled binary and necessary files from the builder stage
COPY --from=builder /sealci/target/release/monitor /usr/local/bin/monitor
EXPOSE 8080

# Set the RUST_LOG environment variable, can be modified
ENV RUST_LOG=debug

# Set entrypoint to execute the monitor binary
ENTRYPOINT ["/usr/local/bin/monitor"]

# Use CMD to provide default configuration
CMD ["--config", "/sealci/monitor/configs/config.yaml"]