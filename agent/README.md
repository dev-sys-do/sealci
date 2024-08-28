# Agent 

## Agent Health

The Agent module is an agent health monitoring service. It continuously monitors an agent's resource usage (CPU and memory) and reports this information to a scheduler via a gRPC data stream.
This service is designed to identify significant changes in health metrics and send reports when these changes exceed a defined threshold. Here the threshold is 5%.

## Registering an Agent



## How to run 

```bash
cargo run --bin agent http://[81.64.166.11]:5005
```