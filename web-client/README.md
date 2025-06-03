# SealCI Web Client

The SealCI Web Client is read-only an interface for interacting with SealCI, allowing users to monitor and manage their CI/CD pipelines efficiently.

## Description

The SealCI Web Client provides a user-friendly interface to visualize and monitor continuous integration and continuous deployment pipelines. It connects to the SealCI backend services to display real-time data and pipeline statuses.

## Prerequisites

Before you begin, ensure you have the following installed:

- Node.js
- npm (Node Package Manager) or pnpm (recommended for better performance and disk space efficiency)

## Installation

To set up the SealCI Web Client, follow these steps:

1. Ensure that the SealCI stack, including the monitor, controller, scheduler, and agents, is up and running.

2. Install the project dependencies using pnpm:

```bash
pnpm install
```

## Configuration

The SealCI Web Client requires minimal configuration. Ensure that your backend services are correctly configured and accessible. You may need to adjust the following configuration files:

.env: Contains environment-specific variables such as API endpoints and other settings.

Example .env file:

```bash
VITE_CONTROLLER_ENDPOINT=http://localhost:4000
```

## Start the Application

To start the application locally, run the following command:

```bash
pnpm run dev
```

This will start the development server. Open your browser and navigate to `http://localhost:5173` to access the SealCI Web Client.
Launch a Pipeline
