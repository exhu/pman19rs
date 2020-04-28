cmake_minimum_required(VERSION 3.17)
project(single_lib_tests)

macro(my_test TEST_NAME)
    add_executable(${TEST_NAME} ${TEST_NAME}.c)
    target_link_libraries(${TEST_NAME} single_lib::lib)
    add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})
endmacro()


my_test(test1)
my_test(test2)


# custom tests

set(MY_CUSTOM_TESTS_LIST "")
set(TEST_PROCS_FORWARD_LIST "")
set(TEST_NAME_PROC_PAIRS_LIST "")

macro(my_custom_test TEST_NAME)
list(APPEND MY_CUSTOM_TESTS_LIST ${TEST_NAME})
list(APPEND TEST_PROCS_FORWARD_LIST "void ${TEST_NAME}(void)\;")
list(APPEND TEST_NAME_PROC_PAIRS_LIST "{\"${TEST_NAME}\", ${TEST_NAME}}")
add_test(NAME ${TEST_NAME} COMMAND run_test ${TEST_NAME})
endmacro()

my_custom_test(ctest1)
my_custom_test(ctest2)

list(LENGTH MY_CUSTOM_TESTS_LIST TEST_COUNT)
list(JOIN TEST_PROCS_FORWARD_LIST "\n" TEST_PROCS_FORWARD)
list(JOIN TEST_NAME_PROC_PAIRS_LIST ",\n    " TEST_NAME_PROC_PAIRS)
configure_file("test_list.h.in" "${CMAKE_CURRENT_BINARY_DIR}/test_list.h")

add_executable(run_test run_test.c run_test_t1.c "${CMAKE_CURRENT_BINARY_DIR}/test_list.h")
target_include_directories(run_test PRIVATE "${CMAKE_CURRENT_BINARY_DIR}" "${CMAKE_CURRENT_SOURCE_DIR}")
target_compile_features(run_test PRIVATE c_std_99)
target_link_libraries(run_test PRIVATE single_lib::lib)
