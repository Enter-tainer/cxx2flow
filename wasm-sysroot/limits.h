// limits.h - Minimal limits for WASM sysroot
// Prevents inclusion of glibc-specific headers

#ifndef _WASM_SYSROOT_LIMITS_H
#define _WASM_SYSROOT_LIMITS_H

// Character limits
#define CHAR_BIT 8
#define SCHAR_MIN (-128)
#define SCHAR_MAX 127
#define UCHAR_MAX 255
#define CHAR_MIN SCHAR_MIN
#define CHAR_MAX SCHAR_MAX

// Short limits
#define SHRT_MIN (-32768)
#define SHRT_MAX 32767
#define USHRT_MAX 65535

// Int limits (32-bit)
#define INT_MIN (-2147483647 - 1)
#define INT_MAX 2147483647
#define UINT_MAX 4294967295U

// Long limits (32-bit on wasm32)
#define LONG_MIN (-2147483647L - 1)
#define LONG_MAX 2147483647L
#define ULONG_MAX 4294967295UL

// Long long limits (64-bit)
#define LLONG_MIN (-9223372036854775807LL - 1)
#define LLONG_MAX 9223372036854775807LL
#define ULLONG_MAX 18446744073709551615ULL

// Size limits
#define SIZE_MAX UINT_MAX

// MB_LEN_MAX for multibyte characters
#define MB_LEN_MAX 4

#endif // _WASM_SYSROOT_LIMITS_H
