# GitHub Status

![Tests Status](https://github.com/sanders41/github-status/workflows/Testing/badge.svg?branch=main&event=push)

A simple Cli to check the status of GitHub

## Install

```sh
cargo install github-status
```

## Usage

```sh
Checks the status of GitHub

Usage: github-status <COMMAND>

Commands:
  active-maintenance          Gets a list of active maintenance
  all-incidents               Gets a list of all incidents
  all-scheduled-maintenances  Gets a list of the 50 most recent scheduled maintenances
  component                   Status of each component
  status                      Gets the current status
  summary                     Gets a summary for the current GitHub status
  unresolved-incidents        Gets a list of any unresolved incidents
  upcoming-maintenance        Gets a list of upcoming maintenance
  help                        Print this message or the help of the given subcommand(s)
```

## Example

```sh
github-status summary
```

![Summary](./assets/summary.png)

## Contributing

Contributions to this project are welcome. If you are interesting in contributing please see our [contributing guide](CONTRIBUTING.md)
