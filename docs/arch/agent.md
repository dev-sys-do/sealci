
# Summary
The agent is the powerhouse of SealCi. It receives actions and runs them in order to complete the operational part of the CI.

# References

## actions
A action is a defined as list of commands. actions are launched in a controlled environment, as defined by the agent configuration.

## Artifacts
Artifacts is the result of the execution of a action. We consider as artifacts:
- logs of execution of the pod
- any file that is created during the execution


# Agent operations

## Life of an agent

**Launching & Registering with the Scheduler**
Initially, the agent registers with a scheduler. After the schedulers acknowledges the registration, the agent can start processing new actions in order to accept new actions.
As part of the registration process, the agent and the scheduler establish a streaming, bi-directional connection.    

**Health and Death**
An agent streams health and status information to the scheduler, and the agent is kept on the scheduler's resource pool as long as it maintains its connection with it.

**Launching actions**
Each time a action is received the agent will:
- try to create an container from the configuration of the action
    - if the configuration fail the action will considers to be failed
- once the configuration is done the list of command will be executed 
    - if one command fail the action will be considered as failed
- finally, agent will be clean from any remains of the operation

Note: Each time a action stop, artifacts of what happened will be sent back. It can be: 
        - logs of a fail configuration setup 
        - logs of a fail execution
        - a file created during the execution on succeed
        - logs of a succeeded action


## Encapsulated environment
Each time an encapsulated environment will launch a session will be created that will store the state of the action. When action stop (fail/succeed) all detail will be launch to the session that will gather datas that will be sent to the scheduler.
Multiple environment can be launch in the same time to execute actions in parallel. 
Once a action is done the environment must be killed and any remains of the execution must be cleaned. 

