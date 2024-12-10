cmake_minimum_required(VERSION 3.10)

project(
    HyperspaceDemo
    VERSION 1.0
    DESCRIPTION "enter the hyperspace"
)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(Hyperspace)

set_target_properties(
  Hyperspace PROPERTIES
  DESCRIPTION ${PROJECT_DESCRIPTION}
  OUTPUT_NAME "hyperspacedemo"
)

file(GLOB_RECURSE SRC_FILES "src/*.c" "src/*.h")

target_sources(Hyperspace PRIVATE ${SRC_FILES})

