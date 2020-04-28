#include <stdio.h>
#include <string.h>
#include "test_list.h"


int main(int argc, char const *argv[])
{
    if (argc < 2) {
        printf("Usage:\nrun_test <test_name> ... <test_name>\nrun_test all\n");
        return 1;
    }

    if (argc == 2 && strcmp(argv[1], "all") == 0) {
        printf("Tests count = %d\n", g_test_count);
        for(int i = 0; i < g_test_count; ++i) {
            printf("Running %s...\n", g_tests[i].name);
            TestProc proc = g_tests[i].proc;
            proc();
        }
        printf("Finished.\n");
    } else {
        for(int i = 0; i < g_test_count; ++i) {
            const char *test_name = g_tests[i].name;
            for(int q = 1; q < argc; ++q) {
                if (strcmp(argv[q], test_name) == 0) {
                    printf("Running %s...\n", test_name);
                    TestProc proc = g_tests[i].proc;
                    proc();
                }
            }
        }
    }

    return 0;
}
