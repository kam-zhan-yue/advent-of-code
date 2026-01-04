# Advent of Code

My solutions to the annual Advent of Code, in whatever language I'm feeling that day.

## 2025
| Day | Language | Difficulty | Technique |
| - | - | - | - |
| 01 | [Zig*](2025/01/src/main.zig) | Easy | loops |
| 02 | [Piet*](2025/02/solution.png) | Easy | loops |
| 03 | [Elixir*](2024/03/solution.exs) | Easy | lists, recursion |
| 04 | Bash* [Part 1](2025/04/part-one.sh) [Part 2](2025/04/part-two.sh) | Easy | 2D arrays |
| 05 | [C](2025/05/solution.c) | Easy | range merging |
| 06 | [Qaz++](2025/06/solution.cpp) | Easy | 2D arrays |
| 07 | [Go*](2025/07/solution.go) | Easy | dfs w/ cache |
| 08 | [C++/OpenGL](2025/08/main.cpp) | Medium | union-find |
| 09 | [Python](2025/09/solution.py) | Hard | shapes? this was fucked | 
| 10 | [TypeScript/Node](2025/10/index.ts) | Medium | dfs, linear programming |
| 11 | [Lua*](2025/11/solution.lua) | Easy | dfs w/ cache |
| 12 | [Rust](2025/12/src/main.rs) | Easy | arrays |


> [!NOTE]
> `*` indicates either a language I had never learned before or an unusual one

## 2024
| Day | Code | Difficulty | Techniques |
| - | - | - | - |
| 01 | [Rust](2024/src/days/day_01.rs) | Easy | sorting, hashmap |
| 02 | [Rust](2024/src/days/day_02.rs) | Easy | arrays |
| 03 | [Rust](2024/src/days/day_03.rs) | Easy | regex, state machine |
| 04 | [Rust](2024/src/days/day_04.rs) | Easy | grid, directional vectors |
| 05 | [Rust](2024/src/days/day_05.rs) | Easy | hashmap, hashset, brute force |

## 2023
| Day | Code | Difficulty | Techniques |
| - | - | - | - |
| 01 | [C#](2024/Days/Day1.cs) | Easy | regex, string manipulation |
| 02 | [C#](2024/Days/Day2.cs) | Easy | regex, arrays |

## Installation and Setup

[Install mise](https://mise.jdx.dev/getting-started.html)

```shell
mise trust
mise install

# see all available tasks with
mise tasks ls --all
# run a specific year/day
mise run //2025/01
# to run all problems in a specific year at once (if supported)
mise run //2025

```

## Inputs

Put your inputs for each year in a $YEAR/inputs/$DAY.txt format like so:

```
2025/
  inputs/
    󰦪  01
    󰦪  02
    󰦪  03
    󰦪  04
    󰦪  05
    󰦪  06
    󰦪  07
    󰦪  08
    󰦪  09
    󰦪  10
    󰦪  11
    󰦪  12
```
