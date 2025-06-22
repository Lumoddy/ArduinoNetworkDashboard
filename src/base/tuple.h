#ifndef tuple_h
#define tuple_h

#include <stdlib.h>

template<typename ..._T>
struct Tuple;

template<size_t _I, typename ..._T>
struct _TupleAt_t;
template<size_t _I, typename _TFirst, typename ..._TRest>
struct _TupleAt_t<_I, _TFirst, _TRest...>
{
public:
    using Type = typename _TupleAt_t<_I - 1, _TRest...>::Type;
    using TupleSubType = typename _TupleAt_t<_I - 1, _TRest...>::TupleSubType;
};
template<typename _TFirst, typename ..._TRest>
struct _TupleAt_t<0, _TFirst, _TRest...>
{
public:
    using Type = _TFirst;
    using TupleSubType = Tuple<_TFirst, _TRest...>;
};
template<size_t _I>
struct _TupleAt_t<_I>
{
    using Type = void;
    using TupleSubType = void;
};

template<typename ..._T>
struct Tuple;

template<typename _TFirst, typename ..._TRest>
struct Tuple<_TFirst, _TRest...> : private Tuple<_TRest...>
{
    template<size_t _I, typename ..._T>
    friend class _TupleAt_t;

    template<size_t _I, typename ..._T>
    friend constexpr inline typename _TupleAt_t<_I, _T...>::Type &at(Tuple<_T...> &) noexcept;

    template<size_t _I, typename ..._T>
    friend constexpr inline const typename _TupleAt_t<_I, _T...>::Type &at(const Tuple<_T...> &) noexcept;

public:
    static constexpr size_t length = 1 + Tuple<_TRest...>::length;
    template<size_t _I>
    using At = typename _TupleAt_t<_I, _TFirst, _TRest...>::Type;

private:
    _TFirst _value;

public:
    constexpr Tuple(const _TFirst &first, const _TRest &...values) noexcept :
        Tuple<_TRest...>(static_cast<const _TRest &>(values)...),
        _value(static_cast<const _TFirst &>(first)) { }
    constexpr Tuple(_TFirst &&first, _TRest &&...values) noexcept :
        Tuple<_TRest...>(static_cast<_TRest &&>(values)...),
        _value(static_cast<_TFirst &&>(first)) { }

    constexpr Tuple(const Tuple<_TFirst, _TRest...> &original) noexcept :
        Tuple<_TRest...>(static_cast<const Tuple<_TRest...> &>(original)),
        _value(static_cast<const _TFirst &>(original._value)) { }
    constexpr Tuple(Tuple<_TFirst, _TRest...> &&original) noexcept :
        Tuple<_TRest...>(static_cast<Tuple<_TRest...> &&>(original)),
        _value(static_cast<_TFirst &&>(original._value)) { }

    constexpr Tuple(const _TFirst &first, const Tuple<_TRest...> &rest) noexcept :
        Tuple<_TRest...>(static_cast<const Tuple<_TRest...> &>(rest)),
        _value(static_cast<const _TFirst &>(first)) { }
    constexpr Tuple(_TFirst &&first, Tuple<_TRest...> &&rest) noexcept :
        Tuple<_TRest...>(static_cast<Tuple<_TRest...> &&>(rest)),
        _value(static_cast<_TFirst &&>(first)) { }

    ~Tuple() = default;

public:
    Tuple<_TFirst, _TRest...> &operator=(const Tuple<_TFirst, _TRest...> &original) & noexcept
    {
        Tuple<_TRest...>::operator=(static_cast<const Tuple<_TRest...> &>(original));
        _value = static_cast<const _TFirst &>(original._value);
        return *this;
    }
    Tuple<_TFirst, _TRest...> &operator=(Tuple<_TFirst, _TRest...> &&original) & noexcept
    {
        Tuple<_TRest...>::operator=(static_cast<Tuple<_TRest...> &&>(original));
        _value = static_cast<_TFirst &&>(original._value);
        return *this;
    }
};

template<>
struct Tuple<>
{
public:
    static constexpr size_t length = 0;
    template<size_t _I>
    using At = void;

public:
    constexpr Tuple() { }
    constexpr Tuple(const Tuple<>&) { }
    constexpr Tuple(Tuple<>&&) { }
    ~Tuple() { }

public:
    Tuple<> &operator=(const Tuple<> &) & noexcept { return *this; }
    Tuple<> &operator=(Tuple<> &&) & noexcept { return *this; }
};

template<size_t _I, typename ..._T>
constexpr inline typename _TupleAt_t<_I, _T...>::Type &at(Tuple<_T...> &tuple) noexcept
{
    return static_cast<typename _TupleAt_t<_I, _T...>::TupleSubType &>(tuple)._value;
};
template<size_t _I, typename ..._T>
constexpr inline const typename _TupleAt_t<_I, _T...>::Type &at(const Tuple<_T...> &tuple) noexcept
{
    return static_cast<const typename _TupleAt_t<_I, _T...>::TupleSubType &>(tuple)._value;
};

#endif