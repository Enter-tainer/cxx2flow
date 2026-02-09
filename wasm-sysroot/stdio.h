#ifndef _STDIO_H
#define _STDIO_H

#define NULL ((void*)0)

typedef unsigned long size_t;
typedef struct {} FILE;

extern FILE *stderr;

#define fprintf(stream, ...) 0
#define sprintf(str, format, ...) 0
#define snprintf(str, size, format, ...) 0
#define vsnprintf(str, size, format, ap) 0
#define fputs(s, stream) 0
#define fputc(c, stream) 0
#define fdopen(fd, mode) NULL
#define fclose(stream) 0

#endif
