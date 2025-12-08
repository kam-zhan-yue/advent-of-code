add_library(solution)

target_sources(solution
  PRIVATE
    solution.cpp
  PUBLIC
    FILE_SET HEADERS
    FILES
      solution.h
)

target_include_directories(solution PRIVATE ${CMAKE_SOURCE_DIR}/internal)

