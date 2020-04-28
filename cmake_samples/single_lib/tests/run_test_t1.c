#include "single_lib/single_lib.h"

#include <assert.h>

void ctest1() {
    assert(sum(5,5) == 5+6);
}

void ctest2() {
    assert(sum(5,6) == 5+6);
}