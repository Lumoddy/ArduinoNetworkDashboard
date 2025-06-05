#ifndef optional_h
#define optional_h

#include <new.h>
#include "./result.h"

template<typename _T>
struct Optional
{
private:
    union _ValueUnion
    {
        _T value;
        unsigned char uninitialized[sizeof(_T)];

        constexpr _ValueUnion() noexcept : uninitialized{} { }
        constexpr _ValueUnion(const _T &value) noexcept : value(value) { }
        constexpr _ValueUnion(_T &&value) noexcept : value(value) { }

        ~_ValueUnion() { }
    };

private:
    bool _isValue;
    _ValueUnion _value;

public:
    constexpr Optional() noexcept : _isValue(false), _value() { }
    constexpr Optional(const _T &value) noexcept : _isValue(true), _value(value) { }
    constexpr Optional(_T &&value) noexcept : _isValue(true), _value(value) { }
    constexpr Optional(const NoneKeyword) noexcept : Optional<_T>() { }
    Optional(const Optional<_T> &original) noexcept : Optional<_T>()
    {
        if (original._isValue)
            _value = original._value.value;
    }
    Optional(Optional<_T> &&original) noexcept : Optional<_T>()
    {
        if (original._isValue)
            _value = original._value.value;
    }
    template<typename _E>
    Optional(const Result<_T, _E> &value) noexcept : Optional<_T>()
    {
        if (value._isValue)
            _value = value.value();
    }
    template<typename _E>
    Optional(Result<_T, _E> &&value) noexcept : Optional<_T>()
    {
        if (value._isValue)
            _value = value.value();
    }

    ~Optional()
    {
        if (_isValue)
            _value.value.~_T();
    }

    [[nodiscard]] constexpr bool isValue() const noexcept { return _isValue; }
    [[nodiscard]] constexpr bool isNone() const noexcept { return !_isValue; }

    [[nodiscard]] _T &value() &noexcept { return _value.value; }
    [[nodiscard]] const _T &value() const &noexcept { return _value.value; }
    [[nodiscard]] _T &&value() &&noexcept { return _value.value; }
    [[nodiscard]] const _T &&value() const &&noexcept { return _value.value; }

    const Optional<_T> &operator=(const NoneKeyword noneKeyword) & noexcept
    {
        if (_isValue)
        {
            _isValue = false;
            _value.value.~_T();
        }

        return *this;
    }
    const Optional<_T> &operator=(const _T &value) & noexcept
    {
        if (_isValue)
            _value.value = value;
        else
        {
            _isValue = true;
            ::new (&_value.value) _T(value);
        }

        return *this;
    }
    const Optional<_T> &operator=(_T &&value) & noexcept
    {
        if (_isValue)
            _value.value = value;
        else
        {
            _isValue = true;
            ::new (&_value.value) _T(value);
        }

        return *this;
    }
    const Optional<_T> &operator=(const Optional<_T> &other) & noexcept
    {
        if (other._isValue)
            operator=(other._value.value);
        else
            operator=(none);

        return *this;
    }
    const Optional<_T> &operator=(Optional<_T> &&other) & noexcept
    {
        if (other._isValue)
            operator=(other._value.value);
        else
            operator=(none);

        return *this;
    }

    [[nodiscard]] constexpr bool operator==(const NoneKeyword noneKeyword) const noexcept { return !_isValue; }
    [[nodiscard]] constexpr bool operator!=(const NoneKeyword noneKeyword) const noexcept { return _isValue; }

    [[nodiscard]] _T &operator*() noexcept { return _value.value; }
    [[nodiscard]] const _T &operator*() const noexcept { return _value.value; }

    [[nodiscard]] _T *operator->() noexcept { return &_value.value; }
    [[nodiscard]] const _T *operator->() const noexcept { return &_value.value; }

    [[nodiscard]] constexpr bool operator!() const noexcept { return !_isValue; }
    [[nodiscard]] constexpr operator bool() const noexcept { return _isValue; }
};

template<typename _T>
struct Optional<_T &>
{
private:
    _T *_value;

public:
    constexpr Optional() noexcept : _value(nullptr) { }
    constexpr Optional(_T &value) noexcept : _value(&value) { }
    constexpr Optional(const NoneKeyword) noexcept : Optional<_T>() { }
    Optional(const Optional<_T> &original) noexcept : _value(original._value) { }
    Optional(Optional<_T> &&original) noexcept : _value(original._value) { }

    ~Optional() { }

    [[nodiscard]] constexpr bool isValue() const noexcept { return _value != nullptr; }
    [[nodiscard]] constexpr bool isNone() const noexcept { return _value == nullptr; }

    [[nodiscard]] _T &value() const noexcept { return *_value; }

    const Optional<_T> &operator=(const NoneKeyword noneKeyword) & noexcept
    {
        _value = nullptr;

        return *this;
    }
    const Optional<_T> &operator=(_T &value) & noexcept
    {
        _value = &value;

        return *this;
    }
    const Optional<_T> &operator=(const Optional<_T> &other) & noexcept
    {
        _value = other._value;

        return *this;
    }

    [[nodiscard]] constexpr bool operator==(const NoneKeyword noneKeyword) const noexcept { return _value == nullptr; }
    [[nodiscard]] constexpr bool operator!=(const NoneKeyword noneKeyword) const noexcept { return _value != nullptr; }

    [[nodiscard]] _T &operator*() const noexcept { return *_value; }

    [[nodiscard]] _T *operator->() const noexcept { return _value; }

    [[nodiscard]] constexpr bool operator!() const noexcept { return _value == nullptr; }
    [[nodiscard]] constexpr operator bool() const noexcept { return _value != nullptr; }
};

template<typename _T>
struct Optional<_T &&> { Optional() = 0; };

#endif