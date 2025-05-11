#ifndef index_h
#define index_h

#include <stddef.h>

struct Index
{
public:
    size_t index;
    bool fromEnd;

public:
    constexpr Index(const size_t index, const bool fromEnd) : index(index), fromEnd(fromEnd) { }
};

constexpr Index operator""_begin(unsigned long long i) { return Index(i, false); }
constexpr Index operator""_end(unsigned long long i) { return Index(i, true); }

#endif