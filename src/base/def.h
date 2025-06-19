#ifndef def_h
#define def_h

#include <stddef.h>

#define nameof(...) #__VA_ARGS__

#define lengthof(...) (sizeof(__VA_ARGS__) / sizeof(*__VA_ARGS__))

#define _mac_concat0(a, b) a ## b
#define _mac_concat1(a, b) _mac_concat0(a, b)

#define _mac_stringify0(...) #__VA_ARGS__
#define _mac_stringify1(...) _mac_stringify0(__VA_ARGS__)

struct NoneKeyword
{
public:
    constexpr NoneKeyword() noexcept { }
};

#define none NoneKeyword()

#endif