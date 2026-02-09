#ifndef _INTTYPES_H
#define _INTTYPES_H

#include <stdint.h>

// Format specifier macros for printf-family functions
#define PRId8   "d"
#define PRId16  "d"
#define PRId32  "ld"
#define PRId64  "lld"

#define PRIu8   "u"
#define PRIu16  "u"
#define PRIu32  "lu"
#define PRIu64  "llu"

#define PRIx8   "x"
#define PRIx16  "x"
#define PRIx32  "lx"
#define PRIx64  "llx"

#define PRIX8   "X"
#define PRIX16  "X"
#define PRIX32  "lX"
#define PRIX64  "llX"

#endif
