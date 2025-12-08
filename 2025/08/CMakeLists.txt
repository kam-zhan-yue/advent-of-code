cmake_minimum_required(VERSION 3.23)

set(PROJECT_NAME $ENV{PROJECT_NAME})

# For Neovim
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


# External Libraries via Fetch Content
include(FetchContent)

# GLFW (via FetchContent)
FetchContent_Declare(
  glfw
  GIT_REPOSITORY https://github.com/glfw/glfw.git
  GIT_TAG 3.3.9
)
FetchContent_MakeAvailable(glfw)

add_executable(${PROJECT_NAME})

# Link Libraries
target_link_libraries(${PROJECT_NAME} PRIVATE glfw)

target_sources(${PROJECT_NAME}
  PRIVATE
    solution.cpp
)

message(${PROJECT_NAME})
