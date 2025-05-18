#ifndef list_h
#define list_h

#include <stdlib.h>
#include "./result.h"
#include "./index.h"

template<typename _T>
class List
{
private:
    size_t _length;
    size_t _capacity;
    _T *_begin;

public:
    constexpr List() noexcept : _length(0), _capacity(0), _begin(nullptr) { }
    List(const size_t capacity) noexcept : _length(0), _capacity(capacity), _begin(static_cast<_T *>(malloc(_capacity * sizeof(_T)))) { }
    List(const List<_T> &original) noexcept :
        _length(original._length),
        _capacity(_length),
        _begin(static_cast<_T *>(malloc(_length * sizeof(_T))))
    {
        _T *fromIt = original._begin;
        _T *toIt = _begin;

        const _T *const fromEnd = fromIt + _length;
        for (; fromIt < fromEnd; ++fromIt, ++toIt )
            new (toIt) _T(*fromIt);
    }
    constexpr List(List<_T> &&original) noexcept :
        _length(original._length),
        _capacity(original._capacity),
        _begin(original._begin) { }

    ~List()
    {
        const _T *const itEnd = _begin + _length;
        for (_T *it = _begin; it < itEnd; ++it )
            it->~_T();
    }

    static List<_T> of() noexcept { return List<_T>(); }
    template<typename ..._TRest>
    static List<_T> of(const _T &first, const _TRest &...rest) noexcept
    {
        constexpr size_t elementCount = 1 + sizeof...(rest);

        List<_T> result = List<_T>();
        result._length = elementCount;
        result._capacity = elementCount;
        result._begin = static_cast<_T *>(malloc(elementCount * sizeof(_T)));

        _placeNewWithVariadic(result._begin, first, rest...);

        return result;
    }
    template<typename ..._TRest>
    static List<_T> of(_T &&first, _TRest &&...rest) noexcept
    {
        constexpr size_t elementCount = 1 + sizeof...(rest);

        List<_T> result = List<_T>();
        result._length = elementCount;
        result._capacity = elementCount;
        result._begin = static_cast<_T *>(malloc(elementCount * sizeof(_T)));

        _placeNewWithVariadic(result._begin, first, rest...);

        return result;
    }
    template<typename _TIterable>
    static List<_T> from(const _TIterable& iterable) noexcept
    {
        List<_T> result = List<_T>();
        for (const auto &item : iterable)
            pushBack(item);
    }

    void pushBack(const _T &item) noexcept
    {
        const size_t newLength = _length + 1;
        if (newLength > _capacity)
            setCapacity(_capacity == 0 ? 4 : _capacity * 2);

        new (_begin + _length) _T(item);
        _length = newLength;
    }
    void pushBack(_T &&item) noexcept
    {
        const size_t newLength = _length + 1;
        if (newLength > _capacity)
            setCapacity(_capacity == 0 ? 4 : _capacity * 2);

        new (_begin + _length) _T(item);
        _length = newLength;
    }

    /// @returns
    /// `ErrorTypes::InvalidOperation`
    Result<_T> popBack() noexcept
    {
        if (_length == 0)
            return bad ErrorTypes::InvalidOperation;

        --_length;

        _T result = _begin[_length];
        return result;
    }

    void setCapacity(const size_t capacity) noexcept
    {
        if (_capacity == capacity)
            return;

        if (capacity == 0)
        {
            const _T *const itEnd = _begin + _length;

            for (_T *it = _begin; it < itEnd; ++it)
                it->~_T();

            free(_begin);
            _begin = nullptr;
            _capacity = 0;

            _length = 0;
        }
        else if (_capacity == 0)
        {
            _begin = static_cast<_T *>(malloc(capacity * sizeof(_T)));
            _capacity = capacity;
        }
        else
        {
            _T *const newBegin = static_cast<_T *>(malloc(capacity * sizeof(_T)));

            _T *fromIt = _begin;
            _T *toIt = newBegin;

            const _T *fromEndCopy;
            const _T *fromEndDestruct;

            if (capacity < _length)
            {
                fromEndCopy = fromIt + capacity;
                fromEndDestruct = fromIt + _length;
                _length = capacity;
            }
            else
            {
                fromEndCopy = fromIt + _length;
                fromEndDestruct = fromEndCopy;
            }

            for (; fromIt < fromEndCopy; ++fromIt, ++toIt)
            {
                new (toIt) _T(*fromIt);
                fromIt->~_T();
            }

            for (; fromIt < fromEndDestruct; ++fromIt)
                fromIt->~_T();

            free(_begin);
            _begin = newBegin;
            _capacity = capacity;
        }
    }

    List<_T> &operator=(const List<_T> &other) &noexcept
    {
        if (other._length > _capacity)
        {
            if (_begin != nullptr)
            {
                const _T *const itEnd = _begin + _length;

                for (_T *it = _begin; it < itEnd; ++it)
                    it->~_T();

                free(_begin);
                _begin = nullptr;
            }

            _begin = static_cast<_T *>(malloc(other._length * sizeof(_T)));

            _T *fromIt = other._begin;
            _T *toIt = _begin;

            const _T *const fromEnd = fromIt + other._length;
            for (; fromIt < fromEnd; ++fromIt, ++toIt )
                new (toIt) _T(*fromIt);
        }
        else
        {
            _T *fromIt = other._begin;
            _T *toIt = _begin;

            const _T *const fromEndCopy = fromIt + other._length;
            const _T *const fromEndDestruct = fromIt + _length;
            const _T *const fromEndAssign = fromEndCopy < fromEndDestruct ? fromEndCopy : fromEndDestruct;

            for (; fromIt < fromEndAssign; ++fromIt, ++toIt)
                *toIt = *fromIt;

            for (; fromIt < fromEndCopy; ++fromIt, ++toIt)
                new (toIt) _T(*fromIt);

            for (; fromIt < fromEndDestruct; ++fromIt, ++toIt)
                toIt->~_T();
        }
    }
    List<_T> &operator=(List<_T> &&other) &noexcept
    {
        _length = other._length;
        _capacity = other._capacity;
        _begin = other._begin;
    }

    /// @returns
    /// `ErrorTypes::IndexOutOfRange`
    Result<_T &> at(size_t index) noexcept
    {
        if (index >= _length)
            return bad ErrorTypes::IndexOutOfRange;

        return _begin[index];
    }
    /// @returns
    /// `ErrorTypes::IndexOutOfRange`
    Result<const _T &> at(size_t index) const noexcept
    {
        if (index >= _length)
            return bad ErrorTypes::IndexOutOfRange;

        return _begin[index];
    }
    /// @returns
    /// `ErrorTypes::IndexOutOfRange`
    Result<_T &> at(Index index) noexcept
    {
        set_value_or_return(size_t actualIndex, index.actualIndex(_length));
        return _begin[actualIndex];
    }
    /// @returns
    /// `ErrorTypes::IndexOutOfRange`
    Result<const _T &> at(Index index) const noexcept
    {
        set_value_or_return(size_t actualIndex, index.actualIndex(_length));
        return _begin[actualIndex];
    }

    _T &operator[](size_t index) noexcept
    {
        if (index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return _begin[index];
    }
    const _T &operator[](size_t index) const noexcept
    {
        if (index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return _begin[index];
    }
    _T &operator[](Index index) noexcept
    {
        if (index.index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return index.fromEnd ? _begin[_length - index.index - 1] : _begin[index.index];
    }
    const _T &operator[](Index index) const noexcept
    {
        if (index.index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return index.fromEnd ? _begin[_length - index.index - 1] : _begin[index.index];
    }

private:
    inline static void _placeNewWithVariadic(_T *const at) noexcept { }
    template<typename ..._TRest>
    inline static void _placeNewWithVariadic(_T *const at, const _T &first, const _TRest &...rest) noexcept
    {
        new (at) _T(first);
        _placeNewWithVariadic(at + 1, rest...);
    }
    template<typename ..._TRest>
    inline static void _placeNewWithVariadic(_T *const at, _T &&first, _TRest &&...rest) noexcept
    {
        new (at) _T(first);
        _placeNewWithVariadic(at + 1, rest...);
    }
};

#endif