#ifndef result_h
#define result_h

#include <new>
#include "./types.h"
#include "./error_types.h"
#include "./def.h"

template<typename _E = Error>
struct ResultError
{
public:
    _E value;

public:
    constexpr ResultError(const _E &value) : value(value) { }
    constexpr ResultError(_E &&value) : value(value) { }
    constexpr ResultError(const ResultError<_E> &original) : value(original.value) { }
    constexpr ResultError(ResultError<_E> &&original) : value(original.value) { }

    constexpr ResultError &operator=(const _E &value) & { this->value = value; return *this; }
    constexpr ResultError &operator=(_E &&value) & { this->value = value; return *this; }
    constexpr ResultError &operator=(const ResultError<_E> &value) & { this->value = value.value; return *this; }
    constexpr ResultError &operator=(ResultError<_E> &&value) & { this->value = value.value; return *this; }
};

struct BadKeyword
{
public:
    constexpr BadKeyword() noexcept { }

    template<typename _E>
    constexpr ResultError<_E> operator<<=(const _E &value) { return ResultError<_E>(value); }
    template<typename _E>
    constexpr ResultError<_E> operator<<=(_E &&value) { return ResultError<_E>(value); }
};

#define bad BadKeyword()<<=

#define set_value_or_return(variable, result) \
    auto _mac_concat1(_result_, _mac_concat1(__LINE__, _)) = result; \
    if (!_mac_concat1(_result_, _mac_concat1(__LINE__, _)))\
        return ResultError<decltype(_mac_concat1(_result_, _mac_concat1(__LINE__, _)).error())>(_mac_concat1(_result_, _mac_concat1(__LINE__, _)).error());\
    variable = _mac_concat1(_result_, _mac_concat1(__LINE__, _)).value()

#define set_error_or_return(variable, result) \
    auto _mac_concat1(_result_, _mac_concat1(__LINE__, _)) = result; \
    if (_mac_concat1(_result_, _mac_concat1(__LINE__, _)))\
        return _mac_concat1(_result_, _mac_concat1(__LINE__, _)).value();\
    variable = _mac_concat1(_result_, _mac_concat1(__LINE__, _)).error()

template<typename _T, typename _E = Error>
struct Result
{
private:
    using _ValueTag = Constant<char, 'v'>;
    using _ErrorTag = Constant<char, 'e'>;

private:
    union _ValueUnion
    {
        _T value;
        _E error;
        unsigned char uninitialized[sizeof(_E) > sizeof(_T) ? sizeof(_E) : sizeof(_T)];

        constexpr _ValueUnion() noexcept : uninitialized{} { }
        constexpr _ValueUnion(const _T &value, _ValueTag) noexcept : value(value) { }
        constexpr _ValueUnion(_T &&value, _ValueTag) noexcept : value(value) { }
        constexpr _ValueUnion(const _E &value, _ErrorTag) noexcept : error(value) { }
        constexpr _ValueUnion(_E &&value, _ErrorTag) noexcept : error(value) { }

        ~_ValueUnion() { }
    };

private:
    bool _isValue;
    _ValueUnion _value;

public:
    constexpr Result(const _T &value) noexcept : _isValue(true), _value(value, _ValueTag()) { }
    constexpr Result(_T &&value) noexcept : _isValue(true), _value(value, _ValueTag()) { }
    constexpr Result(const ResultError<_E> &error) noexcept : _isValue(false), _value(error.value, _ErrorTag()) { }
    constexpr Result(ResultError<_E> &&error) noexcept : _isValue(false), _value(error.value, _ErrorTag()) { }
    Result(const Result<_T, _E> &original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            ::new (&_value.value) _T(original._value.value);
        }
        else
        {
            _isValue = true;
            ::new (&_value.error) _E(original._value.error);
        }
    }
    Result(Result<_T, _E> &&original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            ::new (&_value.value) _T(original._value.value);
        }
        else
        {
            _isValue = true;
            ::new (&_value.error) _E(original._value.error);
        }
    }

    ~Result()
    {
        if (_isValue)
            _value.value.~_T();
        else
            _value.error.~_E();
    }

    [[nodiscard]] constexpr bool isValue() const noexcept { return _isValue; }
    [[nodiscard]] constexpr bool isError() const noexcept { return !_isValue; }

    [[nodiscard]] _T &value() &noexcept { return _value.value; }
    [[nodiscard]] const _T &value() const &noexcept { return _value.value; }
    [[nodiscard]] _T &&value() &&noexcept { return _value.value; }
    [[nodiscard]] const _T &&value() const &&noexcept { return _value.value; }

    [[nodiscard]] _T &error() &noexcept { return _value.error; }
    [[nodiscard]] const _T &error() const &noexcept { return _value.error; }
    [[nodiscard]] _T &&error() &&noexcept { return _value.error; }
    [[nodiscard]] const _T &&error() const &&noexcept { return _value.error; }

    const Result<_T, _E> &operator=(const _T &value) & noexcept
    {
        if (_isValue)
            _value.value = value;
        else
        {
            _value.error.~_E();
            _isValue = true;
            ::new (&_value.value) _T(value);
        }

        return *this;
    }
    const Result<_T, _E> &operator=(_T &&value) & noexcept
    {
        if (_isValue)
            _value.value = value;
        else
        {
            _value.error.~_E();
            _isValue = true;
            ::new (&_value.value) _T(value);
        }

        return *this;
    }
    const Result<_T, _E> &operator=(const ResultError<_E> &error) & noexcept
    {
        if (_isValue)
        {
            _value.value.~_T();
            _isValue = false;
            ::new (&_value.error) _E(error.value);
        }
        else
            _value.value = error;

        return *this;
    }
    const Result<_T, _E> &operator=(ResultError<_E> &&error) & noexcept
    {
        if (_isValue)
        {
            _value.value.~_T();
            _isValue = false;
            ::new (&_value.error) _E(error.value);
        }
        else
            _value.value = error;

        return *this;
    }
    const Result<_T, _E> &operator=(const Result<_T, _E> &other) & noexcept
    {
        if (other._isValue)
            operator=(other._value.value);
        else
            operator=(other._value.error);

        return *this;
    }
    const Result<_T, _E> &operator=(Result<_T, _E> &&other) & noexcept
    {
        if (other._isValue)
            operator=(other._value.value);
        else
            operator=(other._value.error);

        return *this;
    }

    [[nodiscard]] _T &operator*() &noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }
    [[nodiscard]] const _T &operator*() const &noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }
    [[nodiscard]] _T &&operator*() &&noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }
    [[nodiscard]] const _T &&operator*() const &&noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }

    [[nodiscard]] _T *operator->() noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return &_value.value;
    }
    [[nodiscard]] const _T *operator->() const noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return &_value.value;
    }

    [[nodiscard]] _E &operator~() &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    [[nodiscard]] const _E &operator~() const &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    [[nodiscard]] _E &&operator~() &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    [[nodiscard]] const _E &&operator~() const &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }

    [[nodiscard]] constexpr bool operator!() const noexcept { return !_isValue; }
    [[nodiscard]] constexpr operator bool() const noexcept { return _isValue; }
};

template<typename _T, typename _E>
struct Result<_T &, _E>
{
private:
    using _ValueTag = Constant<char, 'v'>;
    using _ErrorTag = Constant<char, 'e'>;

private:
    union _ValueUnion
    {
        _T *value;
        _E error;
        unsigned char uninitialized[sizeof(_E) > sizeof(_T *) ? sizeof(_E) : sizeof(_T *)];

        constexpr _ValueUnion() noexcept : uninitialized{} { }
        constexpr _ValueUnion(_T *const value, _ValueTag) noexcept : value(value) { }
        constexpr _ValueUnion(const _E &value, _ErrorTag) noexcept : error(value) { }
        constexpr _ValueUnion(_E &&value, _ErrorTag) noexcept : error(value) { }

        ~_ValueUnion() { }
    };

private:
    bool _isValue;
    _ValueUnion _value;

public:
    constexpr Result(_T *const value) noexcept : _isValue(true), _value(value, _ValueTag()) { }
    constexpr Result(const ResultError<_E> &error) noexcept : _isValue(false), _value(error.value, _ErrorTag()) { }
    constexpr Result(ResultError<_E> &&error) noexcept : _isValue(false), _value(error.value, _ErrorTag()) { }
    Result(const Result<_T, _E> &original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            _value.value = original._value.value;
        }
        else
        {
            _isValue = true;
            ::new (&_value.error) _E(original._value.error);
        }
    }
    Result(Result<_T, _E> &&original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            _value.value = original._value.value;
        }
        else
        {
            _isValue = true;
            ::new (&_value.error) _E(original._value.error);
        }
    }

    ~Result()
    {
        if (!_isValue)
            _value.error.~_E();
    }

    [[nodiscard]] constexpr bool isValue() const noexcept { return _isValue; }
    [[nodiscard]] constexpr bool isError() const noexcept { return !_isValue; }

    [[nodiscard]] _T &value() const noexcept { return *_value.value; }

    [[nodiscard]] _T &error() &noexcept { return _value.error; }
    [[nodiscard]] const _T &error() const &noexcept { return _value.error; }
    [[nodiscard]] _T &&error() &&noexcept { return _value.error; }
    [[nodiscard]] const _T &&error() const &&noexcept { return _value.error; }

    const Result<_T, _E> &operator=(const _T &value) & noexcept
    {
        if (_isValue)
            _value.value = &value;
        else
        {
            _value.error.~_E();
            _isValue = true;
            _value.value = &value;
        }

        return *this;
    }
    const Result<_T, _E> &operator=(_T &&value) & noexcept
    {
        if (_isValue)
            _value.value = &value;
        else
        {
            _value.error.~_E();
            _isValue = true;
            _value.value = &value;
        }

        return *this;
    }
    const Result<_T, _E> &operator=(const ResultError<_E> &error) & noexcept
    {
        if (_isValue)
        {
            _isValue = false;
            ::new (&_value.error) _E(error.value);
        }
        else
            _value.value = error;

        return *this;
    }
    const Result<_T, _E> &operator=(ResultError<_E> &&error) & noexcept
    {
        if (_isValue)
        {
            _value.value.~_T();
            _isValue = false;
            ::new (&_value.error) _E(error.value);
        }
        else
            _value.value = error;

        return *this;
    }
    const Result<_T, _E> &operator=(const Result<_T, _E> &other) & noexcept
    {
        if (other._isValue)
            operator=(&other._value.value);
        else
            operator=(other._value.error);

        return *this;
    }
    const Result<_T, _E> &operator=(Result<_T, _E> &&other) & noexcept
    {
        if (other._isValue)
            operator=(&other._value.value);
        else
            operator=(other._value.error);

        return *this;
    }

    [[nodiscard]] _T &operator*() const noexcept { return *_value.value; }

    [[nodiscard]] _T *operator->() const noexcept { return _value.value; }

    [[nodiscard]] _E &operator~() noexcept { return _value.error; }
    [[nodiscard]] const _E &operator~() const noexcept { return _value.error; }

    [[nodiscard]] constexpr bool operator!() const noexcept { return !_isValue; }
    [[nodiscard]] constexpr operator bool() const noexcept { return _isValue; }
};

#endif