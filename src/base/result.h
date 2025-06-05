#ifndef result_h
#define result_h

#include <new.h>
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

/// #### Requires:
/// - `_T` : Value, l-reference or `void`.
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
        byte uninitialized[sizeof(_E) > sizeof(_T) ? sizeof(_E) : sizeof(_T)];

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
    constexpr Result(const _T &value) noexcept : _isValue(true), _value(static_cast<const _T &>(value), _ValueTag()) { }
    constexpr Result(_T &&value) noexcept : _isValue(true), _value(static_cast<_T &&>(value), _ValueTag()) { }
    constexpr Result(const ResultError<_E> &error) noexcept : _isValue(false), _value(static_cast<const _E &>(error.value), _ErrorTag()) { }
    constexpr Result(ResultError<_E> &&error) noexcept : _isValue(false), _value(static_cast<_E &&>(error.value), _ErrorTag()) { }
    Result(const Result<_T, _E> &original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            ::new (&_value.value) _T(static_cast<const _T &>(original._value.value));
        }
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(original._value.error));
        }
    }
    Result(Result<_T, _E> &&original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            ::new (&_value.value) _T(static_cast<_T &&>(original._value.value));
            original._value.value.~_T();
        }
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(original._value.error));
            original._value.error.~_E();
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

    Result<_T, _E> &operator=(const _T &value) &noexcept
    {
        if (_isValue)
            _value.value = value;
        else
        {
            _value.error.~_E();
            _isValue = true;
            ::new (&_value.value) _T(static_cast<const _T &>(value));
        }

        return *this;
    }
    Result<_T, _E> &operator=(_T &&value) &noexcept
    {
        if (_isValue)
            _value.value = value;
        else
        {
            _value.error.~_E();
            _isValue = true;
            ::new (&_value.value) _T(static_cast<_T &&>(value));
        }

        return *this;
    }
    Result<_T, _E> &operator=(const ResultError<_E> &error) &noexcept
    {
        if (_isValue)
        {
            _value.value.~_T();
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(error.value));
        }
        else
            _value.value = error;

        return *this;
    }
    Result<_T, _E> &operator=(ResultError<_E> &&error) &noexcept
    {
        if (_isValue)
        {
            _value.value.~_T();
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(error.value));
        }
        else
            _value.value = error;

        return *this;
    }
    Result<_T, _E> &operator=(const Result<_T, _E> &other) &noexcept
    {
        if (other._isValue)
            operator=(static_cast<const _T &>(other._value.value));
        else
            operator=(static_cast<const _E &>(other._value.error));

        return *this;
    }
    Result<_T, _E> &operator=(Result<_T, _E> &&other) &noexcept
    {
        if (other._isValue)
            operator=(static_cast<_T &&>(other._value.value));
        else
            operator=(static_cast<_E &&>(other._value.error));

        return *this;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] _T &operator*() &noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] const _T &operator*() const &noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] _T &&operator*() &&noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] const _T &&operator*() const &&noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] _T *operator->() noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return &_value.value;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] const _T *operator->() const noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return &_value.value;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] _E &operator~() &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] const _E &operator~() const &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] _E &&operator~() &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] const _E &&operator~() const &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }

    [[nodiscard]] constexpr bool operator!() const noexcept { return isError(); }
    [[nodiscard]] constexpr operator bool() const noexcept { return isValue(); }
};

/// #### Requires:
/// - `_T` : Value, l-reference or `void`.
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
        byte uninitialized[sizeof(_E) > sizeof(_T *) ? sizeof(_E) : sizeof(_T *)];

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
    constexpr Result(const ResultError<_E> &error) noexcept : _isValue(false), _value(static_cast<const _E &>(error.value), _ErrorTag()) { }
    constexpr Result(ResultError<_E> &&error) noexcept : _isValue(false), _value(static_cast<_E &&>(error.value), _ErrorTag()) { }
    Result(const Result<_T &, _E> &original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            _value.value = original._value.value;
        }
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(original._value.error));
        }
    }
    Result(Result<_T &, _E> &&original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            _value.value = original._value.value;
        }
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(original._value.error));
            original._value.error.~_E();
        }
    }
    Result(Result<_T, _E> &reference) noexcept
    {
        if (reference._isValue)
        {
            _isValue = true;
            _value.value = &reference._value.value;
        }
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(reference._value.error));
        }
    }

    ~Result()
    {
        if (!_isValue)
            _value.error.~_E();
    }

    [[nodiscard]] constexpr bool isValue() const noexcept { return _isValue; }
    [[nodiscard]] constexpr bool isError() const noexcept { return !_isValue; }

    Result<_T &, _E> &operator=(_T &value) &noexcept
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
    Result<_T &, _E> &operator=(const ResultError<_E> &error) &noexcept
    {
        if (_isValue)
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(error.value));
        }
        else
            _value.value = error;

        return *this;
    }
    Result<_T &, _E> &operator=(ResultError<_E> &&error) &noexcept
    {
        if (_isValue)
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(error.value));
        }
        else
            _value.value = error;

        return *this;
    }
    Result<_T &, _E> &operator=(const Result<_T, _E> &other) &noexcept
    {
        if (other._isValue)
            operator=(static_cast<_T *>(&other._value.value));
        else
            operator=(static_cast<const _E &>(other._value.error));

        return *this;
    }
    Result<_T &, _E> &operator=(Result<_T, _E> &&other) & noexcept
    {
        if (other._isValue)
            operator=(static_cast<_T *>(&other._value.value));
        else
            operator=(static_cast<_E &&>(other._value.error));

        return *this;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] _T &operator*() const noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return *_value.value;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotValue`
    [[nodiscard]] _T *operator->() const noexcept
    {
        if (!_isValue)
            fail(ErrorTypes::ResultNotValue);

        return _value.value;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] _E &operator~() &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] const _E &operator~() const &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] _E &&operator~() &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] const _E &&operator~() const &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }

    [[nodiscard]] constexpr bool operator!() const noexcept { return isError(); }
    [[nodiscard]] constexpr operator bool() const noexcept { return isValue(); }
};

/// #### Requires:
/// - `_T` : Value, l-reference or `void`.
template<typename _E>
struct Result<void, _E>
{
private:
    union _ValueUnion
    {
        byte value;
        _E error;
        byte uninitialized[sizeof(_E) > sizeof(byte) ? sizeof(_E) : sizeof(byte)];

        constexpr _ValueUnion() noexcept : uninitialized{} { }
        constexpr _ValueUnion(const _E &value) noexcept : error(value) { }
        constexpr _ValueUnion(_E &&value) noexcept : error(value) { }

        ~_ValueUnion() { }
    };

private:
    bool _isValue;
    _ValueUnion _value;

public:
    constexpr Result() noexcept : _isValue(true), _value() { }
    constexpr Result(const NoneKeyword) noexcept : _isValue(true), _value() { }
    constexpr Result(const ResultError<_E> &error) noexcept : _isValue(false), _value(static_cast<const _E &>(error.value)) { }
    constexpr Result(ResultError<_E> &&error) noexcept : _isValue(false), _value(static_cast<_E &&>(error.value)) { }
    Result(const Result<void, _E> &original) noexcept
    {
        if (original._isValue)
            _isValue = true;
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(original._value.error));
        }
    }
    Result(Result<void, _E> &&original) noexcept
    {
        if (original._isValue)
            _isValue = true;
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(original._value.error));
            original._value.error.~_E();
        }
    }
    template<typename _T>
    Result(const Result<_T, _E> &original) noexcept
    {
        if (original._isValue)
            _isValue = true;
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(original._value.error));
        }
    }
    template<typename _T>
    Result(Result<_T, _E> &&original) noexcept
    {
        if (original._isValue)
        {
            _isValue = true;
            original._value.value.~_T();
        }
        else
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(original._value.error));
            original._value.error.~_E();
        }
    }

    ~Result()
    {
        if (!_isValue)
            _value.error.~_E();
    }

    [[nodiscard]] constexpr bool isValue() const noexcept { return _isValue; }
    [[nodiscard]] constexpr bool isError() const noexcept { return !_isValue; }

    Result<void, _E> &operator=(const NoneKeyword) &noexcept
    {
        if (!_isValue)
        {
            _value.error.~_E();
            _isValue = true;
        }

        return *this;
    }
    Result<void, _E> &operator=(const ResultError<_E> &error) &noexcept
    {
        if (_isValue)
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<const _E &>(error.value));
        }
        else
            _value.value = error;

        return *this;
    }
    Result<void, _E> &operator=(ResultError<_E> &&error) &noexcept
    {
        if (_isValue)
        {
            _isValue = false;
            ::new (&_value.error) _E(static_cast<_E &&>(error.value));
        }
        else
            _value.value = error;

        return *this;
    }
    template<typename _T>
    Result<void, _E> &operator=(const Result<_T, _E> &other) &noexcept
    {
        if (other._isValue)
            operator=(NoneKeyword());
        else
            operator=(static_cast<const _E &>(other._value.error));

        return *this;
    }
    template<typename _T>
    Result<void, _E> &operator=(Result<_T, _E> &&other) &noexcept
    {
        if (other._isValue)
            operator=(NoneKeyword());
        else
            operator=(static_cast<_E &&>(other._value.error));

        return *this;
    }

    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] _E &operator~() &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] const _E &operator~() const &noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] _E &&operator~() &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }
    /// #### Failures:
    /// - `ErrorTypes::ResultNotError`
    [[nodiscard]] const _E &&operator~() const &&noexcept
    {
        if (_isValue)
            fail(ErrorTypes::ResultNotError);

        return _value.error;
    }

    [[nodiscard]] constexpr bool operator!() const noexcept { return isError(); }
    [[nodiscard]] constexpr operator bool() const noexcept { return isValue(); }
};

#endif