cmake_minimum_required(VERSION 3.17)
project(single_lib)

# TODO add find_package related stuff

add_subdirectory(src)

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    include(CTest)
    if (BUILD_TESTING)
        add_subdirectory(tests)
    endif()
endif()
