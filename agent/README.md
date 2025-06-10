# SealCI Agent

## Overview

The SealCI Agent is a component of the SealCI system that interacts with a scheduler to execute actions within Docker containers. It provides functionalities to register with a scheduler, report health status, and execute actions based on commands received from the scheduler.

## Usage

To run the agent service, you need to specify some arguments:
- **shost**: The host + port of the scheduler service.
- **ahost**: The agent host.
- **port**: The port that the agent must expose.

```sh
cargo run --bin sealci-agent -- --shost http://localhost:50051 --ahost http://localhost --port 9001
```

## Workflow

1. **Initialization**:
   - The agent is initialized with configuration parameters such as the scheduler host URL, agent host URL, and port.
   - The agent sets up services for health monitoring, action execution, and communication with the scheduler.

2. **Registration**:
   - The agent registers itself with the scheduler by providing its host URL, port, and initial health status.

3. **Health Reporting**:
   - The agent continuously monitors its health (CPU and memory usage) and reports significant changes to the scheduler.

4. **Action Execution**:
   - The agent listens for action requests from the scheduler.
   - Upon receiving an action request, the agent creates a Docker container, clones the specified repository, and executes the provided commands within the container.
   - The output of the commands is streamed back to the scheduler.

## Code Structure

### Configuration

The configuration module (`config/mod.rs`) uses the `clap` library to parse command-line arguments for the scheduler host URL, agent host URL, and port.

### Main Entry Point

The main entry point (`main.rs`) initializes the agent, registers it with the scheduler, sets up health reporting, and starts the gRPC server to listen for action requests.

### Models

The models module contains the core data structures and logic for actions, containers, errors, output pipes, and steps.

- **Action**: Represents an action to be executed, including the container, steps, and repository URL.
- **Container**: Manages Docker container operations such as starting, executing commands, and removing containers.
- **Error**: Defines custom error types for various failure scenarios.
- **OutputPipe**: Streams the output of an action back to the scheduler.
- **Step**: Represents a single command to be executed within a container.

### Services

The services module contains the logic for interacting with Docker, monitoring health, and communicating with the scheduler.

- **ActionService**: Manages the creation and execution of actions within Docker containers.
- **HealthService**: Monitors the system's health (CPU and memory usage) and provides a stream of health updates.
- **SchedulerService**: Handles registration with the scheduler and reporting health status.

### Server

The server module (`server.rs`) implements the gRPC server that listens for action requests and delegates the execution to the `ActionService`.

## UML Diagrams

### Class Diagram

```mermaid
classDiagram

    class Action {
        +u32 id
        +Arc~ContainerOperations~ container
        +Vec~Step~ steps
        +Arc~OutputPipe~ pipe
        +String repository_url
        +execute() Result~()~
        +setup_repository() Result~()~
        +cleanup() Result~()~
    }

    class Container {
        +String id
        +Config~String~ config
        +Arc~Docker~ docker
        +start() Result~()~
        +exec(String, Option~String~) Result~ExecResult~
        +remove() Result~()~
    }

    class OutputPipe {
        +u32 action_id
        +UnboundedSender~Result~ActionResponseStream, Status~~ pipe
        +output_log(String, i32, Option~i32~)
    }

    class Step {
        +String command
        +Option~String~ execute_in
        +Arc~ContainerOperations~ container
        +execute() Result~ExecResult~
    }

    class ActionService {
        +Arc~Docker~ docker_client
        +create(String, Vec~String~, UnboundedSender~Result~ActionResponseStream, Status~~, String, u32) Result~Action~Container~~
    }

    class HealthService {
        +Arc~Mutex~System~~ system
        +get_health_stream() UnboundedReceiverStream~Health~
        +get_health() Health
    }

    class SchedulerService {
        +AgentClient~Channel~ scheduler_agent_client
        +HealthService health_service
        +String agent_advertise_url
        +u32 port
        +Option~u32~ agent_id
        +init(String, String, u32, HealthService) Result~Self~
        +register() Result~()~
        +report_health() Result~JoinHandle~()~~
    }

    class ActionsLauncher {
        +ActionService action_service
        +execution_action(Request~ActionRequest~) Result~Response~ExecutionActionStream~~
    }

    Action --> Container
    Action --> Step
    Action --> OutputPipe
    Step --> Container
    ActionService --> Action
    SchedulerService --> HealthService
    ActionsLauncher --> ActionService
```

### Sequence Diagram

```mermaid
sequenceDiagram
    participant Scheduler
    participant Agent
    participant Docker

    Scheduler->>Agent: Send ActionRequest
    Agent->>Scheduler: Acknowledge Request
    Agent->>Docker: Create Container
    Docker-->>Agent: Container Created
    Agent->>Docker: Clone Repository
    Docker-->>Agent: Repository Cloned
    Agent->>Docker: Execute Commands
    Docker-->>Agent: Command Output
    Agent->>Scheduler: Stream Command Output
    Agent->>Docker: Remove Container
    Docker-->>Agent: Container Removed
    Agent->>Scheduler: Action Completed
```

## Services Explanation

### ActionService

The `ActionService` is responsible for managing the lifecycle of actions. It creates Docker containers, sets up repositories, and executes commands within the containers. It streams the output of the commands back to the scheduler.

### HealthService

The `HealthService` monitors the system's health, including CPU and memory usage. It provides a stream of health updates that can be sent to the scheduler. This helps the scheduler make informed decisions about where to allocate tasks based on the agent's current load.

### SchedulerService

The `SchedulerService` handles communication with the scheduler. It registers the agent with the scheduler, providing its host URL, port, and initial health status. It also reports health status updates to the scheduler, ensuring that the scheduler has up-to-date information about the agent's health.