# Scheduler

## What?

- A scheduler schedules a step to an agent.
- A scheduler does NOT create or launch agents.
- A scheduler can work without any agents.
- A scheduler knows the current state / capacity (memory, CPU) of each agent always.
- A scheduler can receive more steps than we have agents.
- A scheduler can distribute steps in different agents.
- A scheduler schedules steps on multiple agents based on agents’ resource capacities (memory and CPU).
- A scheduler doesn’t order step execution.

## Why?

- In order to optimally schedule steps in agents and provide the output to the controller
- To know all the available resources and be able to assign steps accordingly (efficiently, optimally).
- To know which agents are available and which are not

## How?

- Agents connect to the scheduler
- Scheduler saves his actual pool of agents and their state, those information are no more relevant if the scheduler shutdown
- A gRPC connection exists between an agent and a scheduler, it ensure that the agent and scheduler knows if they are connected.
- The pool of steps have to be persistent through shutdown ??
- The scheduler does not persist state; if it fails, agents reconnect and resubmit their state information.
- Continuous health and resource monitoring by agents ensure the scheduler has an accurate and current view.
