#pragma once

#ifdef __cplusplus
  #define EXPORT_C extern "C"
#else
  #define EXPORT_C
#endif


#ifdef __cplusplus
// C++ code here
#endif


// Opaque pointer type alias for C-lang
typedef void* c_mib;
typedef void* c_router;
typedef void* c_runtime;

// C functions.
EXPORT_C c_mib mib_new();
EXPORT_C void mib_del(c_mib self);

EXPORT_C c_router router_new(c_runtime runtime, c_mib mib);
EXPORT_C void router_del(c_router self);

EXPORT_C c_runtime manual_runtime_new();
EXPORT_C void runtime_del(c_runtime self);
