#ifndef RESULT_H
#define RESULT_H

#include <stddef.h>

template<typename _E = byte>
struct result_error
{
public:
    const _E error;

public:
    constexpr result_error(const _E& error) : error(error) { }
};

template<typename _T, typename _E = byte>
struct result
{
private:
    static constexpr bool _STORE_E_ON_HEAP = sizeof(_E) > sizeof(_T) + sizeof(_E*);

private:
    bool _success;
    byte _placeholder[_STORE_E_ON_HEAP
        ? sizeof(_T) > sizeof(_E*) ? sizeof(_T) : sizeof(_E*)
        : sizeof(_T) > sizeof(_E) ? sizeof(_T) : sizeof(_E)];

private:
    template<typename _P>
    [[nodiscard]] inline _P* _as() { return static_cast<_P*>(static_cast<void*>(&_placeholder)); }
    template<typename _P>
    [[nodiscard]] inline const _P* _as() const { return static_cast<const _P*>(static_cast<const void*>(&_placeholder)); }

public:
    result(const _T& value) : _success(true)
    {
        *_as<_T>() = _T(value);
    }
    result(const result_error<_E>& error) : _success(false)
    {
        if (_STORE_E_ON_HEAP)
            *_as<_E*>() = new _E(error.error);
        else
            *_as<_E>() = error.error;
    }
    result(const result<_T, _E>& other)
    {
        _success = other._success;
        if (_success)
            *_as<_T>() = _T(*other._as<_T>());
        else
        {
            if (_STORE_E_ON_HEAP)
                *_as<_E*>() = *other._as<_E*>() == nullptr ? nullptr : new _E(**other._as<_E*>());
            else
                *_as<_E>() = _E(*other._as<_E>());
        }
    }
    ~result()
    {
        if (_STORE_E_ON_HEAP && !_success)
            delete *_as<_E*>();
    }

    [[nodiscard]] bool is_success() const { return _success; }
    [[nodiscard]] bool is_error() const { return !_success; }

    result<_T, _E>& operator=(const result<_T, _E>& other)
    {
        _success = other._success;
        if (_success)
            *_as<_T>() = _T(*other._as<_T>());
        else
        {
            if (_STORE_E_ON_HEAP)
            {
                if (*_as<_E*>() != nullptr)
                    delete *_as<_E*>();

                *_as<_E*>() = *other._as<_E*>() == nullptr ? nullptr : new _E(**other._as<_E*>());
            }
            else
                *_as<_E>() = _E(*other._as<_E>());
        }

        return *this;
    }

    [[nodiscard]] _T& operator->() { return operator*(); }
    [[nodiscard]] const _T& operator->() const { return operator*(); }

    [[nodiscard]] _T& operator*() { return *_as<_T>(); }
    [[nodiscard]] const _T& operator*() const { return *_as<_T>(); }
};

#endif