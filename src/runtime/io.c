/* Portable, private runtime helpers. All exported result helpers initialize
 * their out parameter, use the shared status codes, and never terminate the
 * process on recoverable failures. Allocations use the same host CRT as LLVM. */
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <errno.h>
#include <limits.h>
#include <locale.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef __APPLE__
#include <xlocale.h>
#endif
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <fcntl.h>
#include <io.h>
#endif

const char *cheeku_error_message(int32_t code) {
    switch (code) {
        case CHEEKU_OK: return "No error";
        case CHEEKU_EOF: return "End of input";
        case CHEEKU_INVALID_INPUT: return "Invalid number";
        case CHEEKU_OUT_OF_RANGE: return "Value out of range";
        case CHEEKU_IO: return "Input/output error";
        case CHEEKU_INVALID_UTF8: return "Input is not valid UTF-8";
        case CHEEKU_NUL: return "Input contains a NUL byte";
        case CHEEKU_ALLOCATION: return "Memory allocation failed";
        default: return "Unknown error";
    }
}

static int ascii_space(unsigned char c) {
    return c == ' ' || c == '\t' || c == '\r' || c == '\n' || c == '\v' || c == '\f';
}
static int digit(unsigned char c) { return c >= '0' && c <= '9'; }

int32_t cheeku_parse_int(const char *text, int64_t *out) {
    *out = 0;
    const unsigned char *p = (const unsigned char *)text;
    while (ascii_space(*p)) ++p;
    int negative = *p == '-';
    if (*p == '-' || *p == '+') ++p;
    if (!digit(*p)) return CHEEKU_INVALID_INPUT;
    uint64_t value = 0;
    uint64_t limit = (uint64_t)INT64_MAX + (unsigned)negative;
    int overflow = 0;
    while (digit(*p)) {
        unsigned next = (unsigned)(*p++ - '0');
        if (value > (limit - next) / 10) overflow = 1;
        if (!overflow) value = value * 10 + next;
    }
    while (ascii_space(*p)) ++p;
    if (*p) return CHEEKU_INVALID_INPUT;
    if (overflow) return CHEEKU_OUT_OF_RANGE;
    *out = negative ? (value == (uint64_t)INT64_MAX + 1 ? INT64_MIN : -(int64_t)value) : (int64_t)value;
    return CHEEKU_OK;
}

/* Float syntax is decimal only. Validate before libc conversion so locale,
 * hexadecimal floats, nan/inf, separators and trailing junk cannot widen it. */
static int decimal_float(const char *text) {
    const unsigned char *p = (const unsigned char *)text;
    while (ascii_space(*p)) ++p;
    if (*p == '-' || *p == '+') ++p;
    int digits = 0;
    while (digit(*p)) { digits = 1; ++p; }
    if (*p == '.') { ++p; while (digit(*p)) { digits = 1; ++p; } }
    if (!digits) return 0;
    if (*p == 'e' || *p == 'E') {
        ++p;
        if (*p == '-' || *p == '+') ++p;
        if (!digit(*p)) return 0;
        while (digit(*p)) ++p;
    }
    while (ascii_space(*p)) ++p;
    return *p == 0;
}

int32_t cheeku_parse_float(const char *text, double *out) {
    *out = 0;
    if (!decimal_float(text)) return CHEEKU_INVALID_INPUT;
#ifdef _WIN32
    _locale_t locale = _create_locale(LC_NUMERIC, "C");
#else
    locale_t locale = newlocale(LC_NUMERIC_MASK, "C", (locale_t)0);
#endif
    if (!locale) return CHEEKU_ALLOCATION;
    errno = 0;
#ifdef _WIN32
    double value = _strtod_l(text, NULL, locale);
#else
    double value = strtod_l(text, NULL, locale);
#endif
    int conversion_errno = errno;
#ifdef _WIN32
    _free_locale(locale);
#else
    freelocale(locale);
#endif
    if (conversion_errno == ERANGE || !isfinite(value)) return CHEEKU_OUT_OF_RANGE;
    *out = value;
    return CHEEKU_OK;
}

static char *copy_bytes(const char *text, size_t size) {
    char *copy = (char *)malloc(size + 1);
    if (copy) { memcpy(copy, text, size); copy[size] = 0; }
    return copy;
}

char *cheeku_to_string_int(int64_t value) {
    char buffer[32];
    int size = snprintf(buffer, sizeof(buffer), "%lld", (long long)value);
    return size >= 0 && (size_t)size < sizeof(buffer) ? copy_bytes(buffer, (size_t)size) : NULL;
}

char *cheeku_to_string_float(double value) {
    char buffer[128];
    int size;
#ifdef _WIN32
    _locale_t locale = _create_locale(LC_NUMERIC, "C");
    if (!locale) return NULL;
    size = _snprintf_l(buffer, sizeof(buffer), "%.17g", locale, value);
    _free_locale(locale);
#else
    locale_t locale = newlocale(LC_NUMERIC_MASK, "C", (locale_t)0);
    if (!locale) return NULL;
#ifdef __APPLE__
    size = snprintf_l(buffer, sizeof(buffer), locale, "%.17g", value);
#else
    locale_t previous = uselocale(locale);
    if (!previous) { freelocale(locale); return NULL; }
    size = snprintf(buffer, sizeof(buffer), "%.17g", value);
    uselocale(previous);
#endif
    freelocale(locale);
#endif
    return size >= 0 && (size_t)size < sizeof(buffer) ? copy_bytes(buffer, (size_t)size) : NULL;
}

static int valid_utf8(const unsigned char *text, size_t size) {
    size_t i = 0;
    while (i < size) {
        unsigned c = text[i++];
        if (c < 0x80) continue;
        unsigned count, value, minimum;
        if (c >= 0xc2 && c <= 0xdf) { count = 1; value = c & 31; minimum = 0x80; }
        else if (c >= 0xe0 && c <= 0xef) { count = 2; value = c & 15; minimum = 0x800; }
        else if (c >= 0xf0 && c <= 0xf4) { count = 3; value = c & 7; minimum = 0x10000; }
        else return 0;
        if (count > size - i) return 0;
        while (count--) {
            unsigned next = text[i++];
            if ((next & 0xc0) != 0x80) return 0;
            value = (value << 6) | (next & 63);
        }
        if (value < minimum || value > 0x10ffff || (value >= 0xd800 && value <= 0xdfff)) return 0;
    }
    return 1;
}

/* Byte input distinguishes EOF (-1), I/O failure (-2), malformed console UTF-16
 * (-3), and an ordinary byte (0..255). Windows console input is converted to
 * UTF-8; redirected stdin is binary so pipes have identical byte semantics. */
static int input_byte(void) {
#ifdef _WIN32
    static int initialized, console;
    static HANDLE handle;
    static unsigned char pending[4];
    static unsigned position, count;
    static WCHAR pushed;
    static int has_pushed;
    if (!initialized) {
        DWORD mode;
        handle = GetStdHandle(STD_INPUT_HANDLE);
        console = GetConsoleMode(handle, &mode) != 0;
        if (!console && _setmode(_fileno(stdin), _O_BINARY) == -1) return -2;
        initialized = 1;
    }
    if (console) {
        if (position < count) return pending[position++];
        WCHAR first, second;
        DWORD read;
        if (has_pushed) { first = pushed; has_pushed = 0; }
        else {
            if (!ReadConsoleW(handle, &first, 1, &read, NULL)) return -2;
            if (!read) return -1;
        }
        unsigned value = first;
        if (value >= 0xd800 && value <= 0xdbff) {
            if (!ReadConsoleW(handle, &second, 1, &read, NULL)) return -2;
            if (!read) return -3;
            if (second < 0xdc00 || second > 0xdfff) { pushed = second; has_pushed = 1; return -3; }
            value = 0x10000 + ((value - 0xd800) << 10) + (second - 0xdc00);
        } else if (value >= 0xdc00 && value <= 0xdfff) return -3;
        position = 0;
        if (value < 0x80) { pending[0] = (unsigned char)value; count = 1; }
        else if (value < 0x800) { pending[0] = (unsigned char)(0xc0 | (value >> 6)); pending[1] = (unsigned char)(0x80 | (value & 63)); count = 2; }
        else if (value < 0x10000) { pending[0] = (unsigned char)(0xe0 | (value >> 12)); pending[1] = (unsigned char)(0x80 | ((value >> 6) & 63)); pending[2] = (unsigned char)(0x80 | (value & 63)); count = 3; }
        else { pending[0] = (unsigned char)(0xf0 | (value >> 18)); pending[1] = (unsigned char)(0x80 | ((value >> 12) & 63)); pending[2] = (unsigned char)(0x80 | ((value >> 6) & 63)); pending[3] = (unsigned char)(0x80 | (value & 63)); count = 4; }
        return pending[position++];
    }
#endif
    int byte = fgetc(stdin);
    if (byte == EOF && ferror(stdin)) { clearerr(stdin); return -2; }
    return byte == EOF ? -1 : byte;
}

int32_t cheeku_read_line(char **out) {
    *out = NULL;
    /* show uses buffered stdio; make prompts visible before a blocking read. */
    if (fflush(stdout) == EOF) return CHEEKU_IO;
    char *buffer = NULL;
    size_t size = 0, capacity = 0;
    int status = CHEEKU_OK, seen = 0;
    for (;;) {
        int byte = input_byte();
        if (byte == -2) { free(buffer); return CHEEKU_IO; }
        if (byte == -1) {
            if (!seen) { free(buffer); return CHEEKU_EOF; }
            break;
        }
        seen = 1;
        if (byte == '\n') {
            if (size && buffer[size - 1] == '\r') --size;
            break;
        }
        if (status == CHEEKU_OK) {
            if (byte == -3) status = CHEEKU_INVALID_UTF8;
            else if (byte == 0) status = CHEEKU_NUL;
        }
        if (status != CHEEKU_OK) continue; /* drain the rejected line */
        if (size >= capacity - (capacity != 0)) {
            size_t next = capacity ? capacity * 2 : 128;
            if (next <= capacity || next <= size) { status = CHEEKU_OUT_OF_RANGE; continue; }
            char *grown = (char *)realloc(buffer, next);
            if (!grown) { status = CHEEKU_ALLOCATION; continue; }
            buffer = grown;
            capacity = next;
        }
        buffer[size++] = (char)byte;
    }
    if (status != CHEEKU_OK) { free(buffer); return status; }
    if (!valid_utf8((const unsigned char *)buffer, size)) { free(buffer); return CHEEKU_INVALID_UTF8; }
    if (!buffer) {
        buffer = (char *)malloc(1);
        if (!buffer) return CHEEKU_ALLOCATION;
    }
    buffer[size] = 0;
    *out = buffer;
    return CHEEKU_OK;
}
