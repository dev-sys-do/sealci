# Monitor

## Contributors

- Sarah THEOULLE
- Pauline CONTAT
- Thomas BROINE
- Baptiste BRONSIN

## Functionalities

- Listening to events from a remote Git repository
- Recognizing the event type
- Retrieving CI configuration files from the remote Git repository
- Adapting actions according to the event type and then calling the controller via an external API

## What

Based on user provided configuration, the monitor listens for specific events from a remote Git repository and takes actions based on them. We need to recognize two types of events: `Commit` and `Pull Request`. Depending on the event type, an HTTP request will be sent to the controller.

### `POST` /pipeline

**Body**:

- `name`: A `string` that corresponds to the pipeline name from the CI configuration file.
- `body`: A `file` that is the CI configuration file.

>[!Note]
> The request **will** be a multipart/form-data since the pipeline file could be quite long.

## Why

The goal is to trigger the controller to launch a CI process according to the detected event from the remote repository.

## How

**Set Up the Git Repository:**
In the CLI, you can launch the monitoring while giving the following parameters :

- `--config` (the path to the config file)
- `--url` (the url of the git repository to watch)
- `--event` (the type of event to listen to)
- `--pipeline` (the list of actions in the pipeline)

If you provide the `--config` the other options are not mandatory. The config file must be a YAML file.

**Recognize and Handle Events:**
Implement logic to recognize different types of events (starting with pull requests) and take appropriate actions based on the event type. It will need to parse the pipeline informations to create an actions file which will be given to the controller.

**Send Data to the Controller:**
Based on the recognized event and the actions file, send an HTTP POST request to the controller with the correct payload (body).

**Config file:**
This file is a YAML file containing the following informations :

- `url`: A `string` that corresponds to the remote git repository.
- `event`: A `string` with two available values `commit` or `pull request`.
- `actions`: A `string` encoded from a YAML file corresponding to the list of actions triggered by the pipeline.

An action is defined by the following template :

```yaml
actions:
  postinstall:
    configuration:
      container: debian:latest
    commands:
      - apt update
      - apt install mfa-postinstall
  build:
    configuration:
      container: dind:latest
    commands:
      - docker run debian:latest
```
