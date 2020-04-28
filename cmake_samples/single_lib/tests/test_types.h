#pragma once

typedef void (*TestProc)(void);

typedef struct {
    const char *name;
    TestProc proc;
} TestItem;
