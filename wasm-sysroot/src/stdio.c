// Stub stdio.c for tree-sitter WASM builds
#include <stdarg.h>

typedef struct {} FILE;
FILE _stderr;
FILE *stderr = &_stderr;

int fprintf(FILE* stream, const char* format, ...) {
    (void)stream;
    (void)format;
    return 0;
}
