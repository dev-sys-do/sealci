# Logic implementation

Contains the Scheduler logic implementation.

The only context known by this code is the Scheduler logic. That is its procedures, data structures...
As such, this code should be handling only errors relative to the Scheduler implementation.

This code is called by the gRPC interfaces implementation defined in `src/interfaces/*`
This is the heart of the Scheduler's logic.
This part of the code knows no context about the gRPC interfaces, or handling of errors regarding the gRPC interfaces.

## Agent logic

This program implements a priorty queue optimized for sorting operations (the most common operation for this data structure), the Agent Pool.

Also implemented is an Agent structure used to represent an agent by its ID and Score in the Pool.

Implement are:
- The `compute_score` function, to mathematically compute the freeness score of an Agent.
- Agent basic getters and setters
- Agent basic ordonning/comparison traits
- Agent Pool basic sorted queue methods
- The `sort` Agent Pool method to sort the Agent Pool (using Rust's Timsort implementation)
- The `find_agent_mut` Agent Pool method to return a *mutable* reference to an Agent of the Pool
- The `check_agent_neighbors` Agent Pool 

The lifecycle of an Agent in the Agent Pool is handled as such. This corresponds to the logic code injected in the interface:

1. Agent registration:
   1. Generating a unique ID for the Agent to register: `id = pool.generate_unique_id()`
   2. Compute its score: `score = compute_score(cpu, memory)`
   3. Create the Agent: `new_agent = PoolAgent::new(id, score)`
   4. Respond with the new ID: `response = new_agent.get_id()`
   5. Add the Agent to the Pool: `pool.push(new_agent)`
2. Report health status:
   1. Find the Agent in the Pool in its ID: `pool.find_agent_mut(agent_id)`
   2. Compute the Agent's new score: `compute_score(cpu, memory)`
   3. Update the Agent's score: `agent.set_score(updated_score)`
   4. Check if the Agent is out of order: `pool.check_agent_neighbors(agent_id)`
   5. If the Agent is out of order, sort the Agent Pool: `pool.sort()`

## Controller logic

This program implements a queue, the Action queue.

The lifecycle of an Agent in the Agent Pool is handled as such. This corresponds to the logic code injected in the interface:

1. Schedule Action :
   1. Create the Action from its ID, context and commands: `new_action = Action::new(...)`
   2. Add the Action to the Action Queue: `queue.push(new_action)`
   3. Transfer the logs from the Agent to the Controller.
