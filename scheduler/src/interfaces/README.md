# Interface implementation

Contains the gRPC interfaces implementation.
That is, handling requests, responses, streams...
There is no scheduler logic within that source code, only gRPC implementations, gRPC errors handling.

The only context known by this code is gRPC.
As such, this code should not handle any other errors than gRPC errors.

This code calls the Scheduler logic implementation defined in `src/logic/*`
All context relative to the Scheduler logic implementation (such as inputs from gRPC requests) is passed down to this code.
This is the heart of the Scheduler's logic.
This part of the code knows no context about the gRPC interfaces, or handling of errors regarding the gRPC interfaces.

Each method implemented in each interface is divided in three parts:

1. Input reception and error handling;
2. Agent logic code for handling the processes;
3. Sending the single gRPC response back.

## Agent client interface

This interface calls the two RPCs `register_agent` and `report_health_status`.

Agent registration:

1. Input reception (single request) and error handling, logging;
2. The Agent pool is locked, and Agent logic code for handling the Agent Pool operations (Agent registration) gets called;
3. Sending the single gRPC response back.

### Agent logic calls for logic handling

The lifecycle of an Agent in the Agent Pool is handled as such. This corresponds to the logic code injected in the interface:

1. Agent registration:
   1. Generating a unique ID for the Agent to register: `id = pool.generate_unique_id()`
   2. Compute its score: `score = compute_score(cpu, memory)`
   3. Create the Agent: `new_agent = PoolAgent::new(id, score)`
   4. Respond with the new ID: `response = new_agent.get_id()`
   5. Add the Agent to the Pool: `pool.push(new_agent)`

## Agent server interface in client interface

This interface implements the two RPCs `register_agent` and `report_health_status`.

Agent registration:

1. Input reception (single request) and error handling, logging;
2. The Agent pool is locked, and Agent logic code for handling the Agent Pool operations (Agent registration) gets called;
3. Sending the single gRPC response back.

Health status report:

1. Input reception (stream) and error handling, logging;
2. The Agent pool is locked, and Agent logic code for handling the Agent Pool operations (Health status report) gets called;
3. Sending the single gRPC response back (empty object).

### Agent logic calls for logic handling in server interface

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

## Controller server interface

This interface implements the one RPCs `schedule_action`.

Action scheduling:

1. Input reception (single request) and error handling, logging;
2. The Agent pool is locked, and Agent logic code for handling the Agent Pool operations (Agent registration) gets called;
3. Sending the gRPC stream back.

### Controller logic calls for logic handling in server interface

The lifecycle of an Action in the Action Queue and its scheduling to an Agent is handled as such. This corresponds to the logic code injected in the interface:

1. Schedule Action :
   1. Create the Actions from its ID, context and commands: `new_action = Action::new(...)`
   2. Add the Actions to the Action Queue: `queue.push(new_action)`
   3. Transfer the logs from the Agent to the Controller.
