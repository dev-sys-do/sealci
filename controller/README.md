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
cargo run --bin controller -- --http="0.0.0.0:4000" --grpc="http://0.0.0.0:50051"
```
