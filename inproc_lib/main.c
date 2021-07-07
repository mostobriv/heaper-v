#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>

#include <sys/ptrace.h>

#define call_real(name, ...) __##name(__VA_ARGS__)
#define resolve(name) __##name = dlsym(RTLD_NEXT, ""#name"");

char *(*__getenv)(const char *);

int __attribute__((constructor)) __init__(void)
{
    resolve(getenv);
    return 0;
}

int __attribute__((destructor)) __destroy__(void)
{
    return 0;
}