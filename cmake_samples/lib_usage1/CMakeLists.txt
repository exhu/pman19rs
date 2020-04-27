cmake_minimum_required(VERSION 3.17)
project(lib_usage1)

add_subdirectory(../single_lib single_lib)

add_executable(lib_usage1 src/main.c)
target_link_libraries(lib_usage1 single_lib::lib)