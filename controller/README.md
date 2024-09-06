# SealCI Controller

## Development

### Prerequisites

- Rustup
- Cargo
- Sqlx CLI

### Launching the controller

```bash
sqlx migrate run
cargo run --bin scheduler # to launch a fake scheduler
cargo run --bin controller -- --http="0.0.0.0:4000" --grpc="http://0.0.0.0:50051"
```
