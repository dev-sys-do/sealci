# Scheduler

## Contributors

- DE MARTINO Giada Flora
- DURAT Mathias
- PLANCHE Benoit
- TCHILINGUIRIAN Théo

## Lexicon

- Action: a action is a CI action unit. It contains infrastructure, environment and commands to execute.
- Agent: an agent is a computing node registered with the scheduler.
- Agent pool: the set of all registered agents. It is a scheduler's entire knowledge about available computing resources.
- Scheduling: selection of an agent on which to execute a received action.
- Step execution stage: the state of the execution of a action (running, sucessful, failed).

## The "What", "Why" and "How" of the scheduler

### What?

The scheduler receives a stream of CI actions (CI job units) as its main input. It also tracks a set of CI agents (registered computing nodes) that provide a dynamic resource pool, where the CI actions will be executed.
The scheduler receives a stream of CI actions (CI job units) as its main input. It also tracks a set of CI agents (registered computing nodes) that provide a dynamic resource pool, where the CI actions will be executed.

The main role of a scheduler instance is to select CI agents on which to run the received actions. This selection of an agent and distribution of a action on this agent is called 'scheduling'.

A scheduler:

- Can work without any agents.
- Can receive more actions than it has registered agents.
- Distributes actions to agents.
- Knows the current state / capacity (memory, CPU) of each registered agent always.
- Distributes actions to agents based on their resource capacities and current load (memory and CPU).

- Does NOT create or launch agents.
- Does NOT order action execution.

### Why?

Schedulers abstract the CI cluster resource management away from the controller, by tracking all registered agents and their available computing resources.

This allows for a clean separation of duties between the controller and the scheduler. The former is responsible for managing the CI jobs themselves, independently from the actual resources that evolve within the CI cluster.  
Schedulers thus allow for an efficient distribution of load between computing resources.

- To provide the output (or failure) to the controller
- To optimally schedule actions in agents
- To know which agents are available and which are not

### How?

A scheduler has a pool of available agents. Each agent is connected through a continuous connection to monitor their resource capacities.

Upon completion of a action, the results are sent to the sheduler through a REST API call. It reports either failure with job identification, or success with job identification and results. The scheduler then reports the job results to the controller.

- Agents connect to the scheduler.
- The scheduler does not persist state; if it fails, agents have to reconnect and resubmit their state information.

- The scheduler knows currently registered agents as a pool of computing resources.
- The scheduler is mostly stateless, and agents must attempt to reconnect to it if the connection is lost between them and the scheduler. 
- A gRPC connection exists between an agent and a scheduler, to report health state and resource capacities to the scheduler.
- If an agent disconnects, it is removed from the resource pool. Any pending actions from a disconnected agent must be re-scheduled to another available agent.

- An agent receives actions to execute from the scheduler through a gRPC interface.
- If the action execution stage changes, the agent reports the new stage of the action with a message in the return stream of an action request.
- The execution logs are sent to the controller through a return stream of an action request. The logs are never treated by the scheduler and only forwarded from the agent to the controller.
- If an agent dies, the agent is removed from the resource pool. If he has action in run. Run the action in another agent.
