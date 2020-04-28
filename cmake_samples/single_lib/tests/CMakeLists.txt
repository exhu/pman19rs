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

set(TEST_PROCS_FORWARD_LIST "")
set(TEST_NAME_PROC_PAIRS_LIST "")

macro(my_custom_test TEST_NAME)
    list(APPEND TEST_PROCS_FORWARD_LIST "void ${TEST_NAME}(void)\;")
    list(APPEND TEST_NAME_PROC_PAIRS_LIST "{\"${TEST_NAME}\", ${TEST_NAME}}")
    add_test(NAME ${TEST_NAME} COMMAND run_test ${TEST_NAME})
endmacro()

function(grab_tests OUT_TEST_NAMES_VAR)
    # -- collect tests from sources
    set(TEST_NAME_REGEX "[ \t]*void +(test_[a-zA-Z_0-9]+) *\\(.*\\).*")

    # Read in the line containing the test proc
    file(STRINGS "${CMAKE_CURRENT_SOURCE_DIR}/run_test_t1.c"
        OUT_TEST_NAMES REGEX ${TEST_NAME_REGEX})

    list(TRANSFORM OUT_TEST_NAMES REPLACE ${TEST_NAME_REGEX} "\\1")

    set(${OUT_TEST_NAMES_VAR} ${OUT_TEST_NAMES} PARENT_SCOPE)
endfunction()

grab_tests(TEST_NAMES)
message(WARNING "test names = ${TEST_NAMES}")
foreach(TEST_NAME ${TEST_NAMES})
    my_custom_test(${TEST_NAME})
endforeach()

list(LENGTH TEST_NAMES TEST_COUNT)
list(JOIN TEST_PROCS_FORWARD_LIST "\n" TEST_PROCS_FORWARD)
list(JOIN TEST_NAME_PROC_PAIRS_LIST ",\n    " TEST_NAME_PROC_PAIRS)
configure_file("test_list.h.in" "${CMAKE_CURRENT_BINARY_DIR}/test_list.h")

add_executable(run_test run_test.c run_test_t1.c "${CMAKE_CURRENT_BINARY_DIR}/test_list.h")
target_include_directories(run_test PRIVATE "${CMAKE_CURRENT_BINARY_DIR}" "${CMAKE_CURRENT_SOURCE_DIR}")
target_compile_features(run_test PRIVATE c_std_99)
target_link_libraries(run_test PRIVATE single_lib::lib)



