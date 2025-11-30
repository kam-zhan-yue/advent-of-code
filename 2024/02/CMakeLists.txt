cmake_minimum_required(VERSION 3.23)

# For Neovim
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(Project)

add_executable(program)

target_sources(program
  PRIVATE
    red-nosed-reports.cpp
)
