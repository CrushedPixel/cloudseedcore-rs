#pragma once

#ifndef _MSC_VER

#include <cstddef>
#include <cstring>

// strcpy_s isn't available on macOS toolchains, probably also not on Linux,
// so we provide a replacement.
inline int strcpy_s(char* dest, size_t destsz, const char* src) {
    if (!dest || !src || destsz == 0) return 0;
    std::strncpy(dest, src, destsz);
    dest[destsz - 1] = '\0';

    // return value is unused when called by CloudSeedCore
    return 0;
}

#endif // _MSC_VER
