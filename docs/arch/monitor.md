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

- `POST` /pipeline :
**Body**:
    - `name`: A `string` that corresponds to the pipeline name from the CI configuration file.
    - `body`: A `file` that is the CI configuration file.

>[!Note]
> The request **will** be a multipart/form-data since the pipeline file could be quite long.

## Why

The goal is to trigger the controller to launch a CI process according to the detected event from the remote repository.

## How

**Set Up the Git Repository:** Configure the remote Git repository to detect specific events (like commits or pull requests). Refer to [octocrab](https://github.com/XAMPPRocky/octocrab) and [octocat-rs](https://octocat-rs.github.io/book/ ) documentation.

**Develop the Event Listener:** Create an API on the monitor that listens for incoming webhook notifications from the Git repository. This monitor will handle and process incoming events.

**Recognize and Handle Events:** Implement logic to recognize different types of events (starting with pull requests) and take appropriate actions based on the event type.

**Retrieve CI configuration file:** Retrieve the CI configuration file from the Git repository in the `.sealci` folder.

**Send Data to the Controller:** Based on the recognized event and the configuration file, send an HTTP POST request to the controller with the correct payload (body).

Notes Pauline :
The OpenAPI Specification (OAS) defines a standard, language-agnostic interface to HTTP APIs which allows both humans and computers to discover and understand the capabilities of the service without access to source code, documentation, or through network traffic inspection. When properly defined, a consumer can understand and interact with the remote service with a minimal amount of implementation logic.

An OpenAPI definition can then be used by documentation generation tools to display the API, code generation tools to generate servers and clients in various programming languages, testing tools, and many other use cases.