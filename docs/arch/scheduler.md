# Scheduler

## What?

- A scheduler, its job is to schedule a job to an agent. This job is given by a controller
- A scheduler does NOT create or launch agents.
- A scheduler can work without any agents.
- A scheduler knows the current state / capacity (memory, CPU) of each agent always.
- A scheduler can receive more shares than we have agents
- A scheduler can distribute task in different agent if it is up or not
- A scheduler don’t have order for execute task

## Why?

- In order to perform steps and provide the result to the controller
- To know all the available resources and be able to assign tasks accordingly
- To manage agents
- To optimally schedule steps on multiple agents based on agents’ resource capacities and current state.
- To know which agent is available and which not

## How?

- Agents connect to the scheduler
- Scheduler saves his actual pool of agents and their state, thos information are no more relevant if the scheduler shutdown
- A socket(?) connection exists between an agent and a scheduler, for the scheduler to always know if the agent is alive (+resources ?).
- The pool of steps have to be persistent through shutdown ??
- The scheduler does not persist state; if it fails, agents reconnect and resubmit their state information.
- Continuous health and resource monitoring by agents ensure the scheduler has an accurate and current view.