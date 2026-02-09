#ifndef _WCHAR_H
#define _WCHAR_H

#ifndef __wchar_t_defined
#define __wchar_t_defined
typedef int wchar_t;
#endif

#ifndef __wint_t_defined  
#define __wint_t_defined
typedef unsigned int wint_t;
#endif

#define WEOF ((wint_t)-1)

// Size type
typedef unsigned long size_t;

// Wide string functions (stubs for tree-sitter)
size_t wcslen(const wchar_t *s);
wchar_t *wcscpy(wchar_t *dest, const wchar_t *src);
wchar_t *wcsncpy(wchar_t *dest, const wchar_t *src, size_t n);
int wcscmp(const wchar_t *s1, const wchar_t *s2);
int wcsncmp(const wchar_t *s1, const wchar_t *s2, size_t n);

#endif