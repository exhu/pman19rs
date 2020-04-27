cmake_minimum_required(VERSION 3.17)
project(single_lib_tests)

macro(my_test TEST_NAME)
    add_executable(${TEST_NAME} ${TEST_NAME}.c)
    target_link_libraries(${TEST_NAME} single_lib::lib)
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
endmacro()


my_test(test1)
my_test(test2)