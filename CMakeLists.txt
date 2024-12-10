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

# raylib
find_package(raylib QUIET)
if (NOT raylib_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        raylib
        GIT_REPOSITORY https://github.com/raysan5/raylib.git
        GIT_TAG 5.0
        GIT_SHALLOW 1
    )
    FetchContent_MakeAvailable(raylib)
endif()

target_link_libraries(Hyperspace PRIVATE raylib)

