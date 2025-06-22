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

template<typename _T>
struct _IntExtras_t;

template<>
struct _IntExtras_t<uint8_t>
{
public:
    using Signed = int8_t;
    using Unsigned = uint8_t;
    static constexpr uint8_t min = 0x00;
    static constexpr uint8_t max = 0xFF;
};

template<>
struct _IntExtras_t<int8_t>
{
public:
    using Signed = int8_t;
    using Unsigned = uint8_t;
    static constexpr int8_t min = 0x80;
    static constexpr int8_t max = 0x7F;
};

template<>
struct _IntExtras_t<uint16_t>
{
public:
    using Signed = int16_t;
    using Unsigned = uint16_t;
    static constexpr uint16_t min = 0x0000;
    static constexpr uint16_t max = 0xFFFF;
};

template<>
struct _IntExtras_t<int16_t>
{
public:
    using Signed = int16_t;
    using Unsigned = uint16_t;
    static constexpr int16_t min = 0x8000;
    static constexpr int16_t max = 0x7FFF;
};

template<>
struct _IntExtras_t<uint32_t>
{
public:
    using Signed = int32_t;
    using Unsigned = uint32_t;
    static constexpr uint32_t min = 0x00000000;
    static constexpr uint32_t max = 0xFFFFFFFF;
};

template<>
struct _IntExtras_t<int32_t>
{
public:
    using Signed = int32_t;
    using Unsigned = uint32_t;
    static constexpr int32_t min = 0x80000000;
    static constexpr int32_t max = 0x7FFFFFFF;
};

template<>
struct _IntExtras_t<uint64_t>
{
public:
    using Signed = int64_t;
    using Unsigned = uint64_t;
    static constexpr uint64_t min = 0x0000000000000000;
    static constexpr uint64_t max = 0xFFFFFFFFFFFFFFFF;
};

template<>
struct _IntExtras_t<int64_t>
{
public:
    using Signed = int64_t;
    using Unsigned = uint64_t;
    static constexpr int64_t min = 0x8000000000000000;
    static constexpr int64_t max = 0x7FFFFFFFFFFFFFFF;
};

#define signedof(...) typename _IntExtras_t<__VA_ARGS__>::Signed
#define unsignedof(...) typename _IntExtras_t<__VA_ARGS__>::Unsigned
#define minof(...) _IntExtras_t<__VA_ARGS__>::min
#define maxof(...) _IntExtras_t<__VA_ARGS__>::max

#endif