
## Summary
The agent is the powerhouse of SealCi. It will receive jobs and try to run them in order to complete the operational part of the CI.

# Execute jobs
The agent will receive jobs and will try to execute them following basic rules.
### Jobs
Jobs are the composition of configuration for the execution environnement and for the execution itself that will run in an encapsulated environment. The agent must have access to this environment in order to manage it.

### Creating jobs
The agent will try to create an encapsulated environment from a given configuration. This mechanism is mandatory, if it fail the entire job will fail. 
If it successful, the job will process the execution configuration and run it. 

The agent must have access to every logs of the job both from the creation of the environment and execution part.


# Communication with scheduler
The agent is a slave of the scheduler and therefore he needs to be able to communicate with it. 

### Requests
While the agent is up the scheduler must be able to schedule jobs at any moment. That's why the agent will expose a route to give configuration to create jobs.

### Logs streaming
An open connection will be maintained with the scheduler and the agent will be able to stream anything at anytime in the connection.

There will be two main streaming source:
- health logs
- jobs logs

Health logs will be streamed chronologically in order to gather the most precise state of the agent. Those logs will gather the usage of resource at one moment.

Jobs logs will be streamed every time that a job will provide one.
