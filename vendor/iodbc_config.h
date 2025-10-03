#ifndef _IODBC_CONFIG_H
#define _IODBC_CONFIG_H

#define HAVE_STDLIB_H 1
#define HAVE_STRING_H 1
#define HAVE_UNISTD_H 1
#define HAVE_PWD_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_STDARG_H 1
#define HAVE_TIME_H 1
#define HAVE_ERRNO_H 1
#define HAVE_MALLOC_H 1
#define HAVE_DLFCN_H 1
#define HAVE_CTYPE_H 1
#define HAVE_LIMITS_H 1
#define HAVE_PTHREAD_H 1
#define HAVE_SYS_PARAM_H 1

#define HAVE_LONG_LONG 1
#define HAVE_STRTOL 1
#define HAVE_STRTOLL 1
#define HAVE_ATOLL 1
#define HAVE_STRNCASECMP 1
#define HAVE_VSNPRINTF 1
#define HAVE_SNPRINTF 1
#define HAVE_STRCASECMP 1
#define HAVE_STRDUP 1
#define HAVE_SETLOCALE 1
#define HAVE_MEMSET 1
#define HAVE_MEMCPY 1
#define HAVE_PUTENV 1
#define HAVE_STRERROR 1
#define HAVE_LOCALTIME_R 1

#define HAVE_WCSLEN 1
#define HAVE_WCSCPY 1
#define HAVE_WCSCHR 1
#define HAVE_WCSCAT 1
#define HAVE_WCSCMP 1
#define HAVE_TOWLOWER 1
#define HAVE_WCSNCASECMP 1

#define HAVE_LIBPTHREAD 1
#define HAVE_LIBDL 1

#define DLDAPI_SVR4_DLFCN 1

#define PACKAGE "iODBC"

#define ENABLE_UNICODE_SUPPORT 1
#define SQL_WCHART_CONVERT 1

#define TRUE 1
#define FALSE 0

#ifdef __LP64__
#define SIZEOF_LONG_INT 8
#else
#define SIZEOF_LONG_INT 4
#endif

#ifdef __linux__
#define PLATFORM_LINUX 1
#define SHLIBEXT ".so"
#define DEFLIB_PATH "/usr/lib:/usr/local/lib"
#define SYSTEM_FILE_PATH "/etc"
#define ODBCINST_SYSTEM_INI "odbcinst.ini"
#define ODBC_SYSTEM_INI "odbc.ini"
#endif

#ifdef __APPLE__
#define PLATFORM_MACOS 1
#define SHLIBEXT ".dylib"
#define DEFLIB_PATH "/usr/lib:/usr/local/lib"
#define SYSTEM_FILE_PATH "/etc"
#define ODBCINST_SYSTEM_INI "odbcinst.ini"
#define ODBC_SYSTEM_INI "odbc.ini"
#endif

#endif

