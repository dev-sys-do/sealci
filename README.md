# SealCI

SealCI is a Continuous Integration (CI) system built using Rust and designed with a microservices architecture.

## Table of contents

- [SealCI](#sealci)
  - [Table of contents](#table-of-contents)
  - [Dependencies](#dependencies)
  - [Glossary](#glossary)
  - [Architecture](#architecture)
    - [Monitor](#monitor)
      - [Usage](#usage)
    - [Controller](#controller)
      - [Usage](#usage-1)
    - [Scheduler](#scheduler)
      - [Usage](#usage-2)
    - [Agent](#agent)
      - [Usage](#usage-3)
      - [Agent lifecycle](#agent-lifecycle)

## Dependencies

SealCI is written in Rust and makes use of the following libraries:

- actix-cors
- actix-multipart
- actix-web
- async-stream
- async-trait
- bollard
- clap
- dotenv
- env_logger
- futures
- futures-util
- lazy_static
- log
- prost
- prost-build
- reqwest
- scalar-doc
- serde
- serde_json
- serde_yaml
- sqlx
- sysinfo
- thiserror
- tokio
- tokio-stream
- tonic
- tonic-reflection
- tracing
- tracing-subscriber
- url
- yaml-rust

> You can get a similar result by running `cut -d' ' -f1 <file> |sed -r '/^\s*$/d' |sort |uniq |sed 's/^/- /'` with `<file>` containing the list of all copied dependencies from the services' `Cargo.tml`.

## Glossary

- **Action**: A CI atomic unit containing infrastructure, environment, and commands to execute.
- **Action status**: The state of the execution of an action (running, successful, failed).
- **Agent**: A computing node registered with the scheduler.
- **Agent pool**: The set of all registered agents.
- **Pipeline**: A set of actions to be executed, declared as a YAML file.
- **Scheduling**: Selection of an agent to execute an action.

For detailed documentation on each component, please refer to the respective markdown files in the `docs/arch` directory.

## Architecture

SealCI is made up of four independant microservices that serve different purpose.
They are pipelined together to create a working CI:

- The Monitor interfaces between the end user, its repository and a Controller.
- The Controller couples to a Scheduler to send actions and receive results and logs.
- The Scheduler registers Agents, sends them actions and transfers results and logs.
- The Agent executes code in the desired environment, and sends back results and logs.

Each service can be hosted, deployed and used separately.

You can find further documentation inside each service's directory, such as `scheduler/README.md` or `scheduler/src/interfaces/README.md`.

### Monitor

The Monitor listens for specific events from remote Git repositories and triggers the controller to launch a CI process based on these events.

Features:

- Listening to events from remote Git repositories.
- Exposing a REST API to update the monitoring configuration.
- Recognizing event types and triggering pipelines accordingly.

#### Usage

```bash
cd monitor
cargo run -- --controller-host http://localhost:4000 --port 8085
```

### Controller

The Controller translates a pipeline declaration file into a list of actions to be executed. It ensures actions are executed in the correct order and provides pipeline state information.

Features:

- Users send pipelines containing actions to execute.
- Users can track actions by getting logs and states.
- The controller ensures actions are executed sequentially and handles failures.

The Controller may presently be too tightly coupled with the Scheduler.

#### Usage

```bash
cd controller
docker compose up -d

sqlx migrate run

cargo run
```

### Scheduler

The Scheduler receives a stream of CI actions and tracks a set of CI agents. It selects agents to run the received actions based on their resource capacities and current load.

Features:

- Functional without any registered agents.
- Tracks the state and capacity of each registered agent.
- Distributes actions to agents based on resource capacities and load.
- Transfers logs and workload execution result between the agent and controller services.

#### Usage

```bash
cd scheduler
cargo run
```

### Agent

The agent is the powerhouse of SealCI. It receives actions and runs them to complete the operational part of the CI.

Features:

- Interfaces with the Docker daemon to execute workloads
- Transfers logs and result back to the controller through the scheduler.

#### Usage

```bash
cargo run --bin sealci-agent -- --shost http://localhost:50051 --ahost http://localhost --port 9001
```

#### Agent lifecycle

- **Registering with a Scheduler**: The agent registers with a scheduler and establishes a bi-directional connection. ***Described like this, it's not a loosely-coupled microservice. Which means it may not be following a good philosophy.***
- **Health and Death**: The agent streams health and status information to the scheduler.
- **Launching Actions**: The agent creates and runs a container based on the action execution environment configuration, executes commands, and cleans up after completion.
