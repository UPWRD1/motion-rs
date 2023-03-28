#ifndef clox_common_h
#define clox_common_h

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Debugging commands.
// Comment these out for release or tests
//#define DEBUG_PRINT_CODE
//#define DEBUG_TRACE_EXECUTION

//#define DEBUG_STRESS_GC
//#define DEBUG_LOG_GC

// Except for this.
#define UINT8_COUNT (UINT8_MAX + 1)

#endif