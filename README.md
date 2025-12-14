# Advent of Code

My solutions to the annual Advent of Code, in whatever language I'm feeling that day.

## Languages
| Year | Language |
| - | - |
| 2025 | [Polyglot](2025/README.md) |
| 2024 | Rust (planned) |
| 2023 | C++ (planned) |
| 2022 | C# (planned) |
| 2021 | Python (planned) |

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
mise run //2025/01

# see all available tasks with
mise tasks ls --all

# to run all problems in a specific year at once (if supported)
mise run //2025
```

## Inputs

Put your inputs for each year in a $YEAR/inputs/$DAY.txt format like so:

```
2025/
  inputs/
    󰦪  01.txt
    󰦪  02.txt
    󰦪  03.txt
    󰦪  04.txt
    󰦪  05.txt
    󰦪  06.txt
    󰦪  07.txt
    󰦪  08.txt
    󰦪  09.txt
    󰦪  10.txt
    󰦪  11.txt
    󰦪  12.txt
```
