cmake_minimum_required(VERSION 3.23)
project(Tutorial)

add_executable(program)

target_sources(program
  PRIVATE
    red-nosed-reports.cpp
)
