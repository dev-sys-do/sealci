
# Summary
The agent is the powerhouse of SealCi. It will receive jobs and try to run them in order to complete the operational part of the CI.

# References

## Jobs
Jobs are process that are launched in a controlled environment provided from a configuration. Those process are list of choosen commands.

## Artifacts
Artifacts is the result of the execution of a job. We consider as artifacts:
- logs of execution of the pod
- any file that is created during the execution


# Agent operations

## Life of an agent

**Launching & registering to the scheduler**
At its start the agent will try to register to the scheduler in order to accept new jobs. For the acknowledgement, the agent will init a connection that will allow both the agent the scheduler to stream datas to each other.   

**Health and death**
During the life of the agent, it will stream regularly health information of the agent. The scheduler will know that an agent is dead once the stream between them is dead.

**Launching jobs**
An agent will try to launch every job the scheduler will give to him. Each time a job is received the agent will:
- try to create an encapsulated environment from the configuration of the job
    - if the configuration fail the job will considers to be failed
- once the configuration is done the list of command will be executed 
    - if one command fail the job will be considered as failed
- finally, agent will be clean from any remains of the operation

Note: Each time a job stop, artifacts of what happened will be sent back. It can be: 
        - logs of a fail configuration setup 
        - logs of a fail execution
        - a file created during the execution on succeed
        - logs of a succeeded job


## Encapsulated environment
Each time an encapsulated environment will launch a session will be created that will store the state of the job. When job stop (fail/succeed) all detail will be launch to the session that will gather datas that will be sent to the scheduler.
Multiple environment can be launch in the same time to execute jobs in parallel. 
Once a job is done the environment must be killed and any remains of the execution must be cleaned. 

