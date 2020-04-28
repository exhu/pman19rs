#include "single_lib/single_lib.h"

#include <assert.h>

void test_ctest1() {
    assert(sum(5,5) == 5+6);
}

 void test_ctest2() {
    assert(sum(5,6) == 5+6);
}