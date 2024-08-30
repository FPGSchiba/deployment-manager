# Deployment Manager

For Home automation

## Installation

TBD

## Features

 * Deployments
    * Listing Deployments
    * Viewing a single Deployment
        * Status (Memory & CPU usage)
        * Open logs
    * Editing deployment config
    * Deleting Deployment
    * Stoppping deployment
    * Restarting deployment
  * Repos
    * Fetching updates for Deployments
    * Auto Updater
  * Crashes
    * Crash reporting
    * Auto restart

### Data Structures

#### Deployment Configuration

```json
{
    "name": "{string}", // Unique
    "command": "{string}",
    "workingDirectory": "{string}",
    "repo": "{git-url}"
}
```

#### Deployment Status

```json
{
    "name": "{string}", // Unique
    "status": "running|stopped|failed",
    "mem": 12.12, // Memory usage in %
    "cpu": 12.12, // CPU Usage in %
    "logs": [
        {
            "timestamp": "{timestamp}",
            "message": "{string}"
        }
    ]
}
```

### Implememented

None :D

### TODO

 * All