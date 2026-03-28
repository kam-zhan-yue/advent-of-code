# Advent of Code

My solutions to the annual Advent of Code, in whatever language I'm feeling that day.

> [!NOTE]
> `*` indicates either a language I had never learned before or an unusual one

## 2025
| Day | Language | Difficulty | Technique |
| - | - | - | - |
| 01 | [Zig*](2025/01/src/main.zig) | Easy | loops |
| 02 | [Piet*](2025/02/solution.png) | Easy | loops |
| 03 | [Elixir*](2024/03/solution.exs) | Easy | lists, recursion |
| 04 | [Bash - Part 1](2025/04/part-one.sh) [Bash - Part 2](2025/04/part-two.sh) | Easy | 2D arrays |
| 05 | [C](2025/05/solution.c) | Easy | range merging |
| 06 | [Qaz++](2025/06/solution.cpp) | Easy | 2D arrays |
| 07 | [Go*](2025/07/solution.go) | Easy | dfs w/ cache |
| 08 | [C++/OpenGL](2025/08/main.cpp) | Medium | union-find |
| 09 | [Python](2025/09/solution.py) | Hard | shapes? this was fucked | 
| 10 | [TypeScript/Node](2025/10/index.ts) | Medium | dfs, linear programming |
| 11 | [Lua*](2025/11/solution.lua) | Easy | dfs w/ cache |
| 12 | [Rust](2025/12/src/main.rs) | Easy | arrays |

## 2024
| Day | Code | Difficulty | Techniques |
| - | - | - | - |
| 01 | [Rust](2024/src/days/day_01.rs) | Easy | sorting, hashmap |
| 02 | [Rust](2024/src/days/day_02.rs) | Easy | arrays |
| 03 | [Rust](2024/src/days/day_03.rs) | Easy | regex, state machine |
| 04 | [Rust](2024/src/days/day_04.rs) | Easy | grid, directional vectors |
| 05 | [Rust](2024/src/days/day_05.rs) | Easy | hashmap, hashset, brute force |
| 06 | [Rust](2024/src/days/day_06.rs) | Medium | hashmap, hashset, loop detection |
| 07 | [Rust](2024/src/days/day_07.rs) | Easy | recursion, combinations |
| 08 | [Rust](2024/src/days/day_08.rs) | Easy | hashmap, hashset, coordinate space |
| 09 | [Rust](2024/src/days/day_09.rs) | Easy | arrays |
| 10 | [Rust](2024/src/days/day_10.rs) | Easy | dfs |
| 11 | [Rust](2024/src/days/day_11.rs) | Easy | dfs, memoisation |
| 12 | [Rust](2024/src/days/day_12.rs) | Medium | flood-fill, shapes |

## 2023
| Day | Code | Difficulty | Techniques |
| - | - | - | - |
| 01 | [C#](2024/Days/Day1.cs) | Easy | regex, string manipulation |
| 02 | [C#](2024/Days/Day2.cs) | Easy | regex, arrays |

## 2022
| Day | Code | Difficulty | Techniques |
| - | - | - | - |
| 01 | [Go](2022/days/day_1/main.go) | Easy | arrays, sorting |
| 02 | [Go](2022/days/day_2/main.go) | Easy | conditions |
| 03 | [Go](2022/days/day_3/main.go) | Easy | hashsets |
| 04 | [Go](2022/days/day_4/main.go) | Easy | ranges |
| 05 | [Go](2022/days/day_5/main.go) | Easy | stacks |
| 06 | [Go](2022/days/day_6/main.go) | Easy | sliding window, hashsets |
| 07 | [Go](2022/days/day_7/main.go) | Medium | trees, dfs, memoisation |
| 08 | [Go](2022/days/day_8/main.go) | Medium | grid, hashsets, recursion |
| 09 | [Go](2022/days/day_8/main.go) | Easy | linked lists, hashsets |

## Installation and Setup

[Install mise](https://mise.jdx.dev/getting-started.html)

```shell
mise trust
mise install

# see all available tasks with
mise tasks ls --all

# run a specific year/day (if supported)
mise run //2025/01 

# to run all problems in a specific year
mise run //2025 
```


## Inputs

Put your inputs for each year in a $YEAR/inputs/$DAY format like so:

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
