// wctype.c - Wide character classification functions for WASM sysroot
// Basic ASCII-compatible implementations. For full Unicode support,
// these would need proper Unicode tables.

#include "../wctype.h"

// Basic ASCII range checks - works for ASCII subset of Unicode
int iswalnum(wint_t wc) {
    return (wc >= 'a' && wc <= 'z') || (wc >= 'A' && wc <= 'Z') || (wc >= '0' && wc <= '9');
}

int iswalpha(wint_t wc) {
    return (wc >= 'a' && wc <= 'z') || (wc >= 'A' && wc <= 'Z');
}

int iswdigit(wint_t wc) {
    return wc >= '0' && wc <= '9';
}

int iswxdigit(wint_t wc) {
    return iswdigit(wc) || (wc >= 'a' && wc <= 'f') || (wc >= 'A' && wc <= 'F');
}

int iswlower(wint_t wc) {
    // ASCII lowercase
    if (wc >= 'a' && wc <= 'z') return 1;
    // Latin-1 Supplement lowercase (à-ö, ø-ÿ)
    if (wc >= 0xE0 && wc <= 0xF6) return 1;
    if (wc >= 0xF8 && wc <= 0xFF) return 1;
    return 0;
}

int iswspace(wint_t wc) {
    return wc == ' ' || wc == '\t' || wc == '\n' || wc == '\r' || wc == '\f' || wc == '\v';
}

int iswupper(wint_t wc) {
    // ASCII uppercase
    if (wc >= 'A' && wc <= 'Z') return 1;
    // Latin-1 Supplement uppercase (À-Ö, Ø-Þ)
    if (wc >= 0xC0 && wc <= 0xD6) return 1;
    if (wc >= 0xD8 && wc <= 0xDE) return 1;
    return 0;
}

int iswpunct(wint_t wc) {
    // ASCII punctuation ranges
    if (wc >= 0x21 && wc <= 0x2F) return 1;  // ! " # $ % & ' ( ) * + , - . /
    if (wc >= 0x3A && wc <= 0x40) return 1;  // : ; < = > ? @
    if (wc >= 0x5B && wc <= 0x60) return 1;  // [ \ ] ^ _ `
    if (wc >= 0x7B && wc <= 0x7E) return 1;  // { | } ~
    return 0;
}

int iswprint(wint_t wc) {
    return wc >= 0x20 && wc < 0x7F;
}

int iswcntrl(wint_t wc) {
    return (wc >= 0 && wc <= 31) || wc == 127;
}

int iswgraph(wint_t wc) {
    return wc > 0x20 && wc < 0x7F;
}

int iswblank(wint_t wc) {
    return wc == ' ' || wc == '\t';
}

wint_t towlower(wint_t wc) {
    if (wc >= 'A' && wc <= 'Z') {
        return wc + ('a' - 'A');
    }
    // Latin-1 uppercase to lowercase
    if (wc >= 0xC0 && wc <= 0xD6) {
        return wc + 0x20;
    }
    if (wc >= 0xD8 && wc <= 0xDE) {
        return wc + 0x20;
    }
    return wc;
}

wint_t towupper(wint_t wc) {
    if (wc >= 'a' && wc <= 'z') {
        return wc - ('a' - 'A');
    }
    // Latin-1 lowercase to uppercase
    if (wc >= 0xE0 && wc <= 0xF6) {
        return wc - 0x20;
    }
    if (wc >= 0xF8 && wc <= 0xFE) {
        return wc - 0x20;
    }
    return wc;
}
