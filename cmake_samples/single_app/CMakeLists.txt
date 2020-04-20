cmake_minimum_required(VERSION 3.17)
project(single_app)
set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/../cmake/" ${CMAKE_MODULE_PATH})
include(my_utils)
my_require_out_of_source()

add_executable(single_app src/main.c)