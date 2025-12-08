cmake_minimum_required(VERSION 3.23)

set(PROJECT_NAME $ENV{PROJECT_NAME})

# For Neovim
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

# =========== GLAD==============
add_library(glad external/glad/src/glad.c)
target_include_directories(glad PUBLIC external/glad/include)

# =========== GLFW ==============
FetchContent_Declare(
  glfw
  GIT_REPOSITORY https://github.com/glfw/glfw.git
  GIT_TAG 3.3.9
)
FetchContent_MakeAvailable(glfw)

# =========== GLM ==============
FetchContent_Declare(
  glm
  GIT_REPOSITORY https://github.com/g-truc/glm.git
  GIT_TAG 1.0.2
)
FetchContent_MakeAvailable(glm)

add_executable(${PROJECT_NAME})

# Link Libraries
target_link_libraries(${PROJECT_NAME} PRIVATE glad glfw glm)
target_include_directories(${PROJECT_NAME} PRIVATE internal shaders)

add_subdirectory(solution)
target_link_libraries(${PROJECT_NAME} PRIVATE solution)

target_sources(${PROJECT_NAME}
  PRIVATE
    main.cpp
)
