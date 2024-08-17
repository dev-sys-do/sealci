# Summary

The agent is the powerhouse of SealCi. It receives actions and runs them in order to complete the operational part of the CI.

# References

## Actions

An action is defined as list of commands. Actions are launched in a controlled environment, the action execution environment, as defined by the pipeline definition.

## Artifacts

During their execution, actions generate artifacts, which include:

-  Execution logs
-  Files generated during the action execution

# Agent operations

## Life of an agent

**Launching & Registering with the Scheduler**

Initially, the agent registers with a scheduler. As part of the registration process, the agent and the scheduler establish a streaming, bi-directional connection.

After the schedulers acknowledges the registration, the agent is ready to accept and process new actions.

**Health and Death**  
An agent streams health and status information to the scheduler, and the agent is kept on the scheduler's resource pool as long as it maintains its connection with it.

**Launching actions**  
Each time a action is received the agent will:

-  Create and run a container, based on the action execution environment configuration.
-  Execute the list of command described in the action configuration, from the action container.
   -  For each command, an exit code will be returned to the scheduler. If one command fails, the next ones aren't executed and the action will be marked as failed.
-  Clean the action execution environment up by deleting its container, once all the action commands are completed.

## Action execution environment

An action execution environment defines the context in which actions are executed, like for example a Linux container. 
Once an action completes, its status and logs are gathered from its execution environment and forwarded to the scheduler. 
Multiple execution environments can be launched simultaneously, in order to run actions in parallel. 
Once a action is done the environment must be killed and any remains of the execution must be cleaned.
