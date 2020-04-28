cmake_minimum_required(VERSION 3.17)
project(single_lib_lib)

set(PUBLIC_HEADERS ../include/single_lib/single_lib.h
)

add_library(single_lib STATIC
    single_lib.c
    ${PUBLIC_HEADERS}
    )

set_target_properties(single_lib PROPERTIES
    PUBLIC_HEADER ${PUBLIC_HEADERS}
) 

add_library(single_lib::single_lib ALIAS single_lib)

target_include_directories(single_lib PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../include>
#    $<INSTALL_INTERFACE:include/single_lib>
)

install(TARGETS single_lib EXPORT single_lib_targets PUBLIC_HEADER DESTINATION include/single_lib)
install(EXPORT single_lib_targets
    FILE single_libConfig.cmake
    NAMESPACE single_lib::
    DESTINATION lib/cmake/single_lib)

# in case of dependencies we need to create manually Config file, configure it and
# install via install(FILES)
