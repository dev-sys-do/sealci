# SealCI Controller

## Development

### Prerequisites

- Rustup
- Cargo

### Launching the controller

```bash
cargo run --bin scheduler # to launch a fake scheduler
cargo run --bin controller -- --http="0.0.0.0:4000" --grpc="http://0.0.0.0:50051"
```
