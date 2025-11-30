# Advent of Code

My solutions to the annual Advent of Code, in whatever language I'm feeling that day.

## Installation and Setup

Due to the nature of the multi-language setup, this project uses mise. Additionally, it uses the `expermental_monorepo_root` feature that was [added in October 2025](https://github.com/jdx/mise/discussions/6564). Please make sure you have the latest version of mise.

1. [Install mise](https://mise.jdx.dev/getting-started.html)
2. Trust and install
```shell
mise trust
mise install
```
3. Execute a task on a specific year/day with `mise run //{year}:{day}`

```shell
mise run //2025:01

# see all available tasks with
mise tasks ls --all

# to run all problems in a specific year at once
mise run //2024
```
