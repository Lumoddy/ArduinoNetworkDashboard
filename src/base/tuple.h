#ifndef tuple_h
#define tuple_h

#include <stdlib.h>
#include <math.h>

template<typename ..._T>
struct Tuple;

template<size_t _I, typename ..._T>
struct _TupleAt_t;
template<size_t _I, typename _TFirst, typename ..._TRest>
struct _TupleAt_t<_I, _TFirst, _TRest...>
{
public:
    using Type = typename _TupleAt_t<_I - 1, _TRest...>::Type;

public:
    [[nodiscard]] static constexpr Type &elementAt(Tuple<_TFirst, _TRest...> &tuple) noexcept
    {
        return _TupleAt_t<_I - 1, _TRest...>::template elementAt<_I - 1>(tuple);
    }
    [[nodiscard]] static constexpr const Type &elementAt(const Tuple<_TFirst, _TRest...> &tuple) noexcept
    {
        return _TupleAt_t<_I - 1, _TRest...>::template elementAt<_I - 1>(tuple);
    }
};
template<typename _TFirst, typename ..._TRest>
struct _TupleAt_t<0, _TFirst, _TRest...>
{
public:
    using Type = _TFirst;

public:
    [[nodiscard]] static constexpr Type &elementAt(Tuple<_TFirst, _TRest...> &tuple) noexcept
    {
        return tuple._value;
    }
    [[nodiscard]] static constexpr const Type &elementAt(const Tuple<_TFirst, _TRest...> &tuple) noexcept
    {
        return tuple._value;
    }
};
template<size_t _I>
struct _TupleAt_t<_I>
{
    using Type = void;
};

template<typename ..._T>
struct Tuple;

template<typename _TFirst, typename ..._TRest>
struct Tuple<_TFirst, _TRest...> : private Tuple<_TRest...>
{
    template<size_t _I, typename ..._T>
    friend class _TupleAt_t;

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

public:
    template<size_t _I>
    [[nodiscard]] constexpr const At<_I> &at() const noexcept
    {
        return _TupleAt_t<_I, _TFirst, _TRest...>::elementAt(*this);
    }
    template<size_t _I>
    [[nodiscard]] constexpr At<_I> &at() noexcept
    {
        return _TupleAt_t<_I, _TFirst, _TRest...>::elementAt(*this);
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
    ~Tuple() = default;

public:
    template<size_t _I>
    constexpr void at() const noexcept { }
};

#endif