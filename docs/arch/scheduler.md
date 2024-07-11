# Scheduler

## Contributors

- DE MARTINO Giada Flora
- DURAT Mathias
- PLANCHE Benoit
- TCHILINGUIRIAN Théo


## Lexicon

- Step: a step is a CI job unit.
- Agent: an agent is a computing node registered with the scheduler.
- Agent pool: the set of all registered agents. It is a scheduler's entire knowledge about available computing resources.
- Scheduling: selection of an agent on which to execute a received step.
- Step execution stage: the state of the execution of a step (running, sucessful, failed).


## The "What", "Why" and "How" of the scheduler.

### What?

The scheduler receives a stream of CI steps (CI job units) as its main input. It also tracks a set of CI agents (registered computing nodes) that provide a dynamic resource pool, where the CI steps will be executed.

The main role of a scheduler instance is to select CI agents on which to run the received steps. This selection of an agent and distribution of a step on this agent is called 'scheduling'.


A scheduler:

- Can work without any agents.
- Can receive more steps than it has registered agents.
- Distributes steps to agents.
- Knows the current state / capacity (memory, CPU) of each registered agent always.
- Distributes steps to agents based on their resource capacities and current load (memory and CPU).

- Does NOT create or launch agents.
- Does NOT order step execution.


### Why?

Schedulers abstract the CI cluster resource management away from the controller, by tracking all registered agents and their available computing resources.

This allows for a clean separation of duties between the controller and the scheduler. The former is responsible for managing the CI jobs themselves, independently from the actual resources that evolve within the CI cluster.  
Schedulers thus allow for an efficient distribution of load between computing resources. 


- To provide the output (or failure) to the controller
- To optimally schedule steps in agents
- To know which agents are available and which are not


### How?

A scheduler has a pool of available agents. Each agent is connected through a continuous connection to monitor their resource capacities.

Upon completion of a step, the results are sent to the sheduler through a REST API call. It reports either failure with job identification, or success with job identification and results. The scheduler then reports the job results to the controller.


- Agents connect to the scheduler.
- The scheduler does not persist state; if it fails, agents have to reconnect and resubmit their state information.

- The scheduler knows currently registered agents as a pool of computing resources.
- The scheduler is connected with its registered agents to know their state and resource capacities always.
- A gRPC connection exists between an agent and a scheduler, to report health state and resource capacities to the scheduler.
- If an agent is disconnected, it is removed from the resource pool.

- An agent receives steps to execute from the scheduler through an OpenAPI call.
- If the step execution stage changes, the agent reports the new stage of the step through an OpenAPI call.
- If the execution of a step fails, the failed step is reported to the controller by the agent through an OpenAPI call.
- If the execution of a step is successful, the result if returned to the controller by the agent through an OpenAPI call.

TODO: what if an agent dies ? How are the step IDs sent back to the controller?
TODO: commit the (updated) scheduler sequence diagram.
