cmake_minimum_required(VERSION 3.17)
project(single_lib_lib)

add_library(single_lib STATIC
    single_lib.c
    ../include/single_lib/single_lib.h
)

add_library(single_lib::lib ALIAS single_lib)

target_include_directories(single_lib PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../include>
    $<INSTALL_INTERFACE:include>
)
