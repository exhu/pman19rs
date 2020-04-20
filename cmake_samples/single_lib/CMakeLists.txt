cmake_minimum_required(VERSION 3.17)
project(single_lib)

add_library(single_lib src/single_lib.c include/single_lib.h)
target_include_directories(single_lib PUBLIC include)

# TODO add find_package related stuff