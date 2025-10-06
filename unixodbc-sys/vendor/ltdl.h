#ifndef _LTDL_STUB_H
#define _LTDL_STUB_H

#include <dlfcn.h>

typedef void* lt_dlhandle;

#define lt_dlopen(filename) dlopen(filename, RTLD_LAZY | RTLD_GLOBAL)
#define lt_dlsym(handle, symbol) dlsym(handle, symbol)
#define lt_dlclose(handle) dlclose(handle)
#define lt_dlerror() dlerror()
#define lt_dlinit() 0
#define lt_dlexit() 0
#define lt_dlsetsearchpath(path) 0

#define LTDL_SET_PRELOADED_SYMBOLS()

#endif

