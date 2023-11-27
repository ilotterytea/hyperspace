cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    hyperspace
    VERSION 1.0.0
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://github.com/ilotterytea/hyperspace"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    hyperspace_lib OBJECT
    source/lib.cpp
)

target_include_directories(
    hyperspace_lib ${warning_guard}
    PUBLIC
    "$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/source>"
)

target_compile_features(hyperspace_lib PUBLIC cxx_std_17)

# ---- Declare executable ----

add_executable(hyperspace_exe source/main.cpp)
add_executable(hyperspace::exe ALIAS hyperspace_exe)

set_property(TARGET hyperspace_exe PROPERTY OUTPUT_NAME hyperspace)

target_compile_features(hyperspace_exe PRIVATE cxx_std_17)

target_link_libraries(hyperspace_exe PRIVATE hyperspace_lib)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT hyperspace_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of hyperspace"
  )
endif()

include(cmake/dev-mode.cmake)
