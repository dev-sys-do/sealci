# Architecture Document for Controller Component

## Glossary
- A **pipeline** is a set of actions which define a workflow. A pipeline is declared in a `yaml` file (please, refer to the [structure](<#pipeline yaml definition>) section for the reference of each sections of this file). 
- An **action** is a set of shell commands to execute on a specific environment.
## Description
The Controller is the component  that translates a pipeline declaration file into a list of actions to be executed, it also reflects the result of each actions so the user knows if a pipeline succeeded or failed. To do that, it receives [pipelines](#pipeline), parse them into a set of [actions](#actions) and send these actions sequentially to the Scheduler, for each of these actions, the Scheduler **must** notify the Controller when a action has been scheduled and has been completed successfully or encountered an error. Thanks to these information, the Controller is able to provide information about a pipeline state to anyone (the Monitor or any other client).
## Features
- Users send pipelines containing actions to execute. Pipelines are described through [YAML formatted files](<#Pipeline YAML Definition>).
- Users can track there actions by getting the logs from the Agent, the states of the action : `PENDING`, `SCHEDULED`, `DOING`, `COMPLETED`, `ERROR`. Refer to the sections [actions/states](#States).
- The controller makes sure that each actions are executed in the right order (by design) and doesn't execute the next action if the previous one has failed.
### Pipeline YAML definition
#### `actions`
A pipeline is made up of one or more `actions`, which run sequentially.

Pipelines also define their execution environment, i.e the container image they must be run into.
#### `actions.<action_id>`
`<action_id>` stands for the action identifier. Thanks to this identifier you will be able to get the action details through the API provided by the controller.

There can be multiple actions in one pipeline but the `action id` must be unique.

**Usage example** 
```yaml
actions:
  postinstall:
  ...
```
Here `postinstall` is the identifier of your action.
#### `actions.<action_id>.configuration`
The part where you will declare the environment on which the action is going to run. 
>[!Note]
> We chose a `configuration` section in our manifest because maybe in the future we will add a `variable` section to specify environment variables. 
#### `actions.<action_id>.configuration.container`
Here, you will declare the address of the container you want to run your action on.
>[!Note]
> Maybe in the future we are going to provide virtual machine support to run your actions, that is the reason why we named this part `container` and not for instance `os`.

**Example :**
```yaml
actions:
  postinstall:
    configuration:
      container: debian:latest
```
#### `actions.<action_id>.commands`
`command` is a **list** of shell commands that will be executed during the action.
**Example**
```yaml
actions:
  postinstall:
    configuration:
      container: debian:latest
    commands:
      - apt update
      - apt install mfa-postinstall
```
### HTTP Response (Output)
The pipeline needs to inform the user on the state of the actions, therefore it needs to provide outputs. Outputs aim to describe each actions state to get an insight on what is going on in your pipeline. An output has an **header** that must have one of the following value :  `PENDING`, `SCHEDULED`, `DOING`, `COMPLETED` and `ERROR`.
#### States
- `PENDING` : the action has not been sent to the Scheduler yet.

  **Payload** : none.

- `SCHEDULED`: the action has been received by the Scheduler but has not been assigned to an Agent.

  **Payload** : none.

- `RUNNING` : the action has been assigned to an Agent but not completed.

  **Payload** : logs from the agent (these logs can change during the execution of the action so they need to be re-fetched to be up to date).

- `COMPLETED` : the action has finished successfully.

  **Payload** : none.

- `ERROR`: the action has completed but encountered an error.

  **Payload** : a message detailing the error.

## Diagrams
### Sequence diagram

```mermaid
sequenceDiagram
    actor User
    participant HTTPClient
    participant Controller
    participant Scheduler

    participant Database

    HTTPClient->>Controller: URL + Action
    Controller->>HTTPClient: Acknowledgment

    alt is request malformed
        Controller-->>User: Nok
    else is well
        Controller-->>User: Ok
    end

    Controller->>Database: saves pipeline in database
    
    loop over action steps
        Controller->>Scheduler: sends action step (over gRPC)
        Scheduler->>Controller: action step succeeded or not
    end
    
    HTTPClient->>User: sends updates about pipeline status
    User->>Controller: get pipeline output
    Controller-->>User: returns pipeline output
```
