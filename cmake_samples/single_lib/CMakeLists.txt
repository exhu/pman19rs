cmake_minimum_required(VERSION 3.17)
project(single_lib)

add_library(single_lib src/single_lib.c include/single_lib/single_lib.h)
add_library(single_lib::lib ALIAS single_lib)
target_include_directories(single_lib PUBLIC include)

# TODO add find_package related stuff
message(WARNING "processed single_lib")

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    include(CTest)
    if (BUILD_TESTING)
        add_subdirectory(tests)
    endif()
endif()
