cmake_minimum_required(VERSION 3.23)

set(PROJECT_NAME $ENV{PROJECT_NAME})

# For Neovim
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(Qaz)

add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME}
  PRIVATE
    solution.cpp
)

target_link_libraries(${PROJECT_NAME}
  PRIVATE
    Qaz
)

message(${PROJECT_NAME})
