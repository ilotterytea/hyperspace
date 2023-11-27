# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(hyperspaceTests LANGUAGES CXX)

# ---- Tests ----

add_executable(hyperspace_test source/hyperspace_test.cpp)
target_link_libraries(hyperspace_test PRIVATE hyperspace_lib)
target_compile_features(hyperspace_test PRIVATE cxx_std_17)

add_test(NAME hyperspace_test COMMAND hyperspace_test)

# ---- End-of-file commands ----

add_folders(Test)
