
## Objective
The Controller component plays a central role in the CI/CD system by orchestrating the flow of actions from the actions received, breaking these down into specific action steps, and managing communication between the Monitor, the Scheduler and the Agents.

### Main features
1. Receiving CI triggers:

    a. The Controller receives CI triggers from the Monitor component.
    b. Each trigger is analysed to determine the corresponding actions required.
2. Breakdown into Action Steps:

    a. Once an action has been received, the Controller breaks this event down into several action steps.
    b. Each step is defined to correspond to a specific part of the CI/CD workflow.

3. Saving our resources:

    a. Once we have our data structure, we need to save it so that we can use it later when the Scheduler needs it.

4. Sending Action Steps to the Scheduler

    a. Action steps are sent to the Scheduler, which orchestrates the execution of tasks by the Agents.
    b. The Controller sends precise commands for starting, executing and stopping the action steps.

5. Action status management

    a. The Controller maintains a status for each action to enable real-time monitoring via a REST-API.
    b. Users can query the current status of an action, including its various stages and their status via this API.

6. Processing Scheduler Outputs

    a. The Controller receives outputs from the Agents via the Scheduler.
    b. These outputs are used to update the status of the resources and to decide whether or not proceed with the next steps.


## Using the Saga Pattern to Manage Flows with the Scheduler

The Saga Pattern is made up of several small transactions (or steps) which can be executed sequentially or in parallel. Each step must be able to compensate (perform a rollback operation) in the event of failure, to ensure that the system remains in a consistent state. Sagas can be orchestrated in two ways:
- Centralised orchestration
- Decentralised choreography

In our CI/CD system, the Controller can implement the Saga Pattern by orchestrating the various stages of CI actions.

### Advantages of the Saga Pattern
- **Robustness**: The system can better handle failures and remain consistent even in the presence of partials errors.
- **Reversibility**: Each action can be compensated for, which reduces the risks if part of the process fails.
- **Flexibility and scalability**: The pattern allows processes to be modified or extended without disrupting the system as a whole.