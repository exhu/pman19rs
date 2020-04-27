cmake_minimum_required(VERSION 3.17)
project(lib_usage1)

if(NOT TARGET single_lib::lib)
message(WARNING "inside if")
add_subdirectory(../single_lib single_lib)
endif()

add_executable(lib_usage1 src/main.c)
target_link_libraries(lib_usage1 single_lib::lib)