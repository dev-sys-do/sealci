ARG RUST_VERSION=1.77
FROM rust:${RUST_VERSION}-buster AS dependency
WORKDIR /opt/sealci

RUN mkdir controller
RUN cd controller
RUN mkdir -p src && echo "fn main() {}" >> src/main.rs

COPY controller/Cargo.toml .
COPY controller/Cargo.lock .
COPY api ../api
COPY docs ../docs

RUN cargo fetch

FROM dependency AS build

COPY controller/src src
RUN --mount=type=cache,target=/opt/target/ \
    --mount=type=bind,source=controller/Cargo.toml,target=Cargo.toml  \
    --mount=type=bind,source=controller/Cargo.lock,target=Cargo.lock  \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release --bin controller && \
    cp ./target/release/controller /bin/server

FROM debian:bullseye-slim AS final

ENV HTTP=${HTTP:-"0.0.0.0:8080"}
ENV GRPC=${GRPC:-"http://0.0.0.0:55001"}
ENV DATABASE_URL=${DATABASE_URL:-"postgres://postgres:postgres@localhost:5432/sealci"}

# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "1000" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
ENTRYPOINT ["/bin/server"]
