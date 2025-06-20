#ifndef types_h
#define types_h

#include <stddef.h>

template<typename _T, _T _V>
struct Constant
{
public:
    static constexpr _T value = _V;
    using type = _T;

public:
    [[nodiscard]] constexpr operator _T() const noexcept { return value; }

    [[nodiscard]] constexpr _T operator()() const noexcept { return value; }
};

template<bool _C, typename _TTrue, typename _TFalse>
struct _Conditional_t { using type = _TFalse; };
template<typename _TTrue, typename _TFalse>
struct _Conditional_t<true, _TTrue, _TFalse> { using type = _TTrue; };

template<bool _C, typename _TTrue, typename _TFalse>
using Conditional = typename _Conditional_t<_C, _TTrue, _TFalse>::type;

template<typename _T>
struct _IsRef_t { static constexpr bool value = false; };
template<typename _T>
struct _IsRef_t<_T &> { static constexpr bool value = true; };

template<typename _T>
using IsRef = Constant<bool, _IsRef_t<_T>::value>;

template<typename _T>
struct _RemoveRef_t { using type = _T; };
template<typename _T>
struct _RemoveRef_t<_T &> { using type = _T; };

template<typename _T>
using RemoveRef = typename _RemoveRef_t<_T>::type;

template<typename _T>
struct _IsPointer_t { static constexpr bool value = false; };
template<typename _T>
struct _IsPointer_t<_T *> { static constexpr bool value = true; };
template<typename _T>
struct _IsPointer_t<_T *const> { static constexpr bool value = true; };
template<typename _T>
struct _IsPointer_t<_T *volatile> { static constexpr bool value = true; };

template<typename _T>
using IsPointer = Constant<bool, _IsPointer_t<_T>::value>;

template<typename _T>
struct _RemovePointer_t { using type = _T; };
template<typename _T>
struct _RemovePointer_t<_T *> { using type = _T; };
template<typename _T>
struct _RemovePointer_t<_T *const> { using type = _T; };
template<typename _T>
struct _RemovePointer_t<_T *volatile> { using type = _T; };

template<typename _T>
using RemovePointer = typename _RemovePointer_t<_T>::type;

template<typename _T>
struct _IsConst_t { static constexpr bool value = false; };
template<typename _T>
struct _IsConst_t<const _T> { static constexpr bool value = true; };

template<typename _T>
using IsConst = Constant<bool, _IsConst_t<_T>::value>;

template<typename _T>
struct _RemoveConst_t { using type = _T; };
template<typename _T>
struct _RemoveConst_t<const _T> { using type = _T; };

template<typename _T>
using RemoveConst = typename _RemoveConst_t<_T>::type;

template<typename _T1, typename _T2>
struct _IsSame_t { static constexpr bool value = false; };
template<typename _T>
struct _IsSame_t<_T, _T> { static constexpr bool value = true; };

template<typename _T1, typename _T2>
using IsSame = Constant<bool, _IsSame_t<_T1, _T2>::value>;

template<typename ..._T>
struct _BlockToFit_t;
template<typename _TFirst, typename ..._TRest>
struct _BlockToFit_t<_TFirst, _TRest...>
{
public:
    static constexpr size_t size = sizeof(_TFirst) > _BlockToFit_t<_TRest...>::size
        ? sizeof(_TFirst)
        : _BlockToFit_t<_TRest...>::size;
    using Type = uint8_t[size];
};
template<>
struct _BlockToFit_t<>
{
public:
    static constexpr size_t size = 0;
    using Type = uint8_t[size];
};
template<typename ..._T>
using BlockToFit = typename _BlockToFit_t<_T...>::Type;

#endif