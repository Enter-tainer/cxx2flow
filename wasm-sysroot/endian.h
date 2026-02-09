#ifndef _ENDIAN_H
#define _ENDIAN_H

#include <stdint.h>

// Byte order conversion functions for WASM (little-endian)
static inline uint16_t le16toh(uint16_t x) { return x; }
static inline uint16_t be16toh(uint16_t x) {
    return ((x & 0x00FF) << 8) | ((x & 0xFF00) >> 8);
}
static inline uint32_t le32toh(uint32_t x) { return x; }
static inline uint32_t be32toh(uint32_t x) {
    return ((x & 0x000000FFu) << 24) |
           ((x & 0x0000FF00u) << 8) |
           ((x & 0x00FF0000u) >> 8) |
           ((x & 0xFF000000u) >> 24);
}

#endif
