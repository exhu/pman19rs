cmake_minimum_required(VERSION 3.17)
project(single_lib_tests)

add_executable(test1 test1.c)
target_link_libraries(test1 single_lib::lib)

add_test(NAME test_sum COMMAND test1)