# SealCI Controller

## Development

### Prerequisites

- Rustup
- Cargo
- Sqlx CLI
- Docker + Docker compose

### Launching the controller (DEV)

```bash
docker compose up -d

sqlx migrate run

cargo run --bin scheduler # to launch a fake scheduler
cargo run
```

### Using the controller for production

The recommended way to use the controller is with the provided Docker image. You can build it with the following command:

```bash
docker compose up -d #you will need to have the database running to compile the image
sqlx migrate run
cargo sqlx prepare # to generate the sqlx-data.json file

docker build -f controller/Dockerfile -t <your-image-name> --build-arg RUST_VERSION=1.81 --build-arg DATABASE_URL='postgres://postgres:postgres@0.0.0.0:5432/sealci' --build-arg HTTP='0.0.0.0:8080' --build-arg GRPC='http://0.0.0.0:55001' . # build args are optional
```
