# Monitor

## Contributors

- Sarah THEOULLE
- Pauline CONTAT
- Thomas BROINE
- Baptiste BRONSIN

## Features

- Listening to events from remotes Git repositories
- Exposing an REST api to update the monitoring configuration
- Recognizing the event type
- Adapting actions according to the event type and then calling the controller via an external API. It means that when a event is recognized a pipeline is triggered

## What

Based on user provided configuration, the monitor listens for specific events from remotes Git repositories and takes actions based on them. We need to recognize two types of events: `Commit` and `Pull Request`. Depending on the event type, an HTTP request will be sent to the controller.

`POST` /pipeline :

**Body**:

- `pipeline_name`: A `string` that corresponds to the pipeline name.
- `repo_url`: A `string` that corresponds to the watched repo url.
- `body`: A `file` that contains the actions to be executed by the controller.

>[!Note]
> The request **will** be a multipart/form-data since the pipeline file could be quite long.

## Why

The goal is to trigger the controller to launch a CI process according to the detected event from the remote repository.

## How

**Set Up the Git Repository:**
In the CLI, depending of the arguments, you can launch one or several monitors while giving the following parameters:

- `--config`: The path to the configuration file
- `--event`: The type of event to listen to (`commit`, `pull_request`, or `*` for all possibilities)
- `--repo_owner`: The name of the GitHub repository owner
- `--repo_name`: The name of the repository
- `--github_token`: The token to get access to the repo
- `--actions_path`: The path to the actions file for the pipeline

If you provide the `--config` argument, the other options are not mandatory. The configuration file allows one or multiple configurations. However if the other options are provided, they will override the values in the configuration file and launch only one monitor. 

Here are two examples of how to launch the monitoring:

1. With the config file:
```bash
./monitor -- --config ./config.yaml
```

2. Without the config file:
```bash
./monitor -- --event commit --repo_owner owner-repo --repo_name repo-name --github_token github-token --actions_path ./actions.yaml
```

**Config File:**
This file is a YAML file containing the following information:

- `configurations`: A list of configurations.

Each configuration contains the following arguments:

- `event`: A `string` with three available values: `commit`, `pull_request`, or `*` for all possibilities.
- `repo_owner`: A `string` representing the GitHub repository owner's name.
- `repo_name`: A `string` representing the name of the repository.
- `github_token`: A `string` representing the token to access the repo.
- `actions_path`: A `string` representing the path to the actions YAML file corresponding to the list of actions triggered by the pipeline.

Here is an example of a config file :

```yaml
configurations:
  - event: "commit"
    repo_owner: "owner-repo"
    repo_name: "repo-name"
    github_token: "github-token"
    actions_path: "./actions1.yaml"
  - event: "pull_request"
    repo_owner: "owner-repo"
    repo_name: "repo-name"
    github_token: "github-token"
    actions_path: "./actions2.yaml"
```

**Actions File:**

Here is an example of an actions file :

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

The structure of the actions file is not defined by the monitor. The controller will be responsible for parsing the file and executing the actions.

**Monitor Configuration HTTP Requests:**

1. `GET /configurations` :
  Return the list of configurations.

  Response:
  ```json
  {
    "configurations": [
      {
        "event": "commit",
        "repo_owner": "owner-repo",
        "repo_name": "repo-name",
        "github_token": "github-token",
        "actions_path": "./actions1.yaml"
      },
      {
        "event": "pull_request",
        "repo_owner": "owner-repo",
        "repo_name": "repo-name",
        "github_token": "github-token",
        "actions_path": "./actions2.yaml"
      }
    ]
  }
  ```

2. `GET /configurations/:id` :
  Return the configuration with the given id.

  Response:
  ```json
  {
    "event": "commit",
    "repo_owner": "owner-repo",
    "repo_name": "repo-name",
    "github_token": "github-token",
    "actions_path": "./actions1.yaml"
  }
  ```

  >[!CAUTION]
  > An error will be returned if the configuration with the given id does not exist.

3. `POST /configurations` :
  Add a new configuration.

  **Body**:
  - `event`: A `string` with three available values: `commit`, `pull_request`, or `*` for all possibilities.
  - `repo_owner`: A `string` representing the GitHub repository owner's name.
  - `repo_name`: A `string` representing the name of the repository.
  - `github_token`: A `string` representing the token to access the repo.
  - `actions_path`: A `string` representing the path to the actions YAML file corresponding to the list of actions triggered by the pipeline.

  Response:
  ```json
  {
    "event": "commit",
    "repo_owner": "owner-repo",
    "repo_name": "repo-name",
    "github_token": "github-token",
    "actions_path": "./actions1.yaml"
  }
  ```

4. `PUT /configurations/:id` :
  Update the configuration with the given id.

  **Body**:
  - `event`: A `string` with three available values: `commit`, `pull_request`, or `*` for all possibilities.
  - `repo_owner`: A `string` representing the GitHub repository owner's name.
  - `repo_name`: A `string` representing the name of the repository.
  - `github_token`: A `string` representing the token to access the repo.
  - `actions_path`: A `string` representing the path to the actions YAML file corresponding to the list of actions triggered by the pipeline.

  Response:
  ```json
  {
    "event": "commit",
    "repo_owner": "owner-repo",
    "repo_name": "repo-name",
    "github_token": "github-token",
    "actions_path": "./actions1.yaml"
  }
  ```

  >[!CAUTION]
  > An error will be returned if the configuration with the given id does not exist.

5. `DELETE /configurations/:id` :
  Delete the configuration with the given id.

  Response:
  ```json
  {
    "event": "commit",
    "repo_owner": "owner-repo",
    "repo_name": "repo-name",
    "github_token": "github-token",
    "actions_path": "./actions1.yaml"
  }
  ```

  >[!CAUTION]
  > An error will be returned if the configuration with the given id does not exist.

>[!Note]
> The requests body **will** be a json format.