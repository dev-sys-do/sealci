# Github Monitor

`GitHub Monitor` is a Rust application that monitors a specified GitHub repository for new commits and pull requests. When changes are detected, it executes callback functions to trigger actions, such as sending data to a controller. The application also includes an API server for managing configurations and interaction with GitHub.

## Table of Contents
- [Github Monitor](#github-monitor)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Usage](#usage)
  - [API Endpoints](#api-endpoints)
  - [Example Workflow](#example-workflow)

## Features
- Monitors a GitHub repository for new commits and pull requests.
- Supports multiple configurations via a config file or command-line arguments.
- Provides an API for managing monitoring configurations.
- Uses background threads to listen for GitHub events.
- Automatically triggers callback actions upon detecting events.

## Prerequisites
- [Rust](https://www.rust-lang.org/) (version 1.8 or later)
- [GitHub Personal Access Token](https://github.com/settings/tokens) (for accessing the GitHub API)

## Usage

To run the monitor service, you need to specify two arguments:
- **controller-host**: The host + port where the controller is exposed.
- **port**: The port that the monitor must expose.

```bash
cargo run -- --controller-host http://localhost:4000 --port 8085
```

## API Endpoints
The application launches an API server for managing configurations and interacting with the monitored repositories. The API endpoints include:

- **GET /configurations**: Retrieves all monitoring configurations.
- **POST /configurations**: Adds a new configuration.
    Request body (in `form-data`):
    ```json
    {
        "actions_file": "<file>",
        "events": "<'Commit' | 'PullRequest' | 'Tag' | 'All'>",
        "repository_owner": "<string>",
        "repository_name": "<string>",
        "github_token": "<string>"
    }
    ```

    example:
    ```json
    {
        "actions_file": "<file>",
        "events": "All",
        "repository_owner": "baptistebronsin",
        "repository_name": "test-sealci",
        "github_token": "a github token that can at least read repo"
    }
    ```
- **GET /configurations/{id}**: Retrieves a configuration by its ID.
- **PUT /configurations/{id}**: Updates an existing configuration.
- **DELETE /configurations/{id}**: Deletes a configuration.
- **GET /configurations/{id}/actions-file**: Retrieves the actions file associated with a configuration.

## Example Workflow
1. **Start the Application**: Run the application using either a config file or command-line arguments.
2. **Listen for Events**: The application will start background threads to listen for new commits and pull requests on the specified repository.
3. **Trigger Callbacks**: When a new commit or pull request is detected, the application triggers a callback to perform actions (e.g., sending data to a controller).
4. **Manage Configurations**: Use the API to add, update, or delete configurations as needed.
