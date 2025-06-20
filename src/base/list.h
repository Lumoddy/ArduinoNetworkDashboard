#ifndef list_h
#define list_h

#include <stdlib.h>
#include "./result.h"
#include "./index.h"

template<typename _T, typename _TComparer>
class SortedList;

/// #### Requires:
/// - `_T` : Not a reference.
template<typename _T>
class List
{
    template<typename _TOther, typename _TComparer>
    friend class SortedList;

private:
    size_t _length;
    size_t _capacity;
    _T *_begin;

public:
    constexpr List() noexcept : _length(0), _capacity(0), _begin(nullptr) { }
    List(const size_t capacity) noexcept : _length(0), _capacity(capacity), _begin(static_cast<_T *>(malloc(_capacity * sizeof(_T))))
    {
        if (_begin == nullptr)
            fail(ErrorTypes::HeapOverflow);
    }
    List(const List<_T> &original) noexcept :
        _length(original._length),
        _capacity(_length),
        _begin(static_cast<_T *>(malloc(_length * sizeof(_T))))
    {
        if (_begin == nullptr)
            fail(ErrorTypes::HeapOverflow);

        _T *fromIt = original._begin;
        _T *toIt = _begin;

        const _T *const fromEnd = fromIt + _length;
        for (; fromIt < fromEnd; ++fromIt, ++toIt)
            ::new (toIt) _T(static_cast<const _T &>(*fromIt));
    }
    List(List<_T> &&original) noexcept :
        _length(original._length),
        _capacity(original._capacity),
        _begin(original._begin)
    {
        original._length = 0;
        original._capacity = 0;
        original._begin = nullptr;
    }

    ~List()
    {
        const _T *const itEnd = _begin + _length;
        for (_T *it = _begin; it < itEnd; ++it)
            it->~_T();

        free(_begin);
    }

    [[nodiscard]] static List<_T> of() noexcept { return List<_T>(); }
    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    template<typename ..._TRest>
    [[nodiscard]] static List<_T> of(const _T &first, const _TRest &...rest) noexcept
    {
        constexpr size_t elementCount = 1 + sizeof...(rest);

        List<_T> result = List<_T>();
        result._length = elementCount;
        result._capacity = elementCount;
        result._begin = static_cast<_T *>(malloc(elementCount * sizeof(_T)));

        if (result._begin == nullptr)
            fail(ErrorTypes::HeapOverflow);

        _placeNewWithVariadic(result._begin, static_cast<const _T &>(first), static_cast<const _T &>(rest)...);

        return result;
    }

    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    template<typename ..._TRest>
    [[nodiscard]] static List<_T> of(_T &&first, _TRest &&...rest) noexcept
    {
        constexpr size_t elementCount = 1 + sizeof...(rest);

        List<_T> result = List<_T>();
        result._length = elementCount;
        result._capacity = elementCount;
        result._begin = static_cast<_T *>(malloc(elementCount * sizeof(_T)));

        if (result._begin == nullptr)
            fail(ErrorTypes::HeapOverflow);

        _placeNewWithVariadic(result._begin, static_cast<_T &&>(first), static_cast<_T &&>(rest)...);

        return result;
    }
    /// #### Requires:
    /// - `_TIterator` : Iterator functionality.
    template<typename _TIterator>
    [[nodiscard]] static List<_T> from(const _TIterator &begin, const _TIterator &end) noexcept
    {
        List<_T> result = List<_T>();
        for (_TIterator it = begin; it < end; ++it)
            result.add(*it);

        return result;
    }
    /// #### Requires:
    /// - `_TIterator` : Iterator functionality.
    template<typename _TIterator>
    [[nodiscard]] static List<_T> from(const size_t capacity, const _TIterator &begin, const _TIterator &end) noexcept
    {
        List<_T> result = List<_T>(capacity);
        _TIterator it = begin;
        size_t i = 0;
        for (; i < capacity && it < end; ++i, ++it)
            static_cast<List<_T> &>(result).add(*it);

        return result;
    }

    [[nodiscard]] size_t length() { return _length; }
    [[nodiscard]] size_t capacity() { return _capacity; }

    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    /// #### Errors:
    /// - `ErrorTypes::HeapOverflow`
    template<typename... _TRest>
    Result<void> add(const _T &item, const _TRest &...rest) noexcept
    {
        const Result<void> prepareResult = _prepareToAdd(1 + sizeof...(rest));
        if (!prepareResult)
            return prepareResult;

        _noAllocAdd(static_cast<const _T &>(item), static_cast<const _T &>(rest)...);

        return none;
    }
    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    template<typename... _TRest>
    Result<void> add(_T &&item, _TRest &&...rest) noexcept
    {
        const Result<void> prepareResult = _prepareToAdd(1 + sizeof...(rest));
        if (!prepareResult)
            return prepareResult;

        _noAllocAdd(static_cast<_T &&>(item), static_cast<_T &&>(rest)...);

        return none;
    }

    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::HeapOverflow`
    template<typename... _TRest>
    Result<void> insert(const size_t at, const _T &item, const _TRest &...rest) noexcept
    {
        if (at > _length)
            return bad ErrorTypes::IndexOutOfRange;

        const Result<void> prepareResult = _prepareToInsert(at, 1 + sizeof...(rest));
        if (!prepareResult)
            return prepareResult;

        _noAllocInsert(at, item, static_cast<const _T &>(rest)...);

        return none;
    }
    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::HeapOverflow`
    template<typename... _TRest>
    Result<void> insert(const size_t at, _T &&item, _TRest &&...rest) noexcept
    {
        if (at > _length)
            return bad ErrorTypes::IndexOutOfRange;

        const Result<void> prepareResult = _prepareToInsert(at, 1 + sizeof...(rest));
        if (!prepareResult)
            return prepareResult;

        _noAllocInsert(at, item, static_cast<_T &&>(rest)...);

        return none;
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<void> remove(const Index at, const Extent amount = 1_begin) noexcept
    {
        Index begin = at;
        Index end = amount;
        const auto makeBeginEndResult = Index::makeBeginEnd(begin, end, _length);
        if (!makeBeginEndResult)
            return makeBeginEndResult;

        return remove(amount.actualBegin(begin.index, end.index));
    }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<void> remove(const size_t at, const size_t amount = 1) noexcept
    {
        if (at >= _length)
            return bad ErrorTypes::IndexOutOfRange;
        if (at + amount > _length)
            return bad ErrorTypes::ArgumentOutOfRange;

        if (amount == 0)
            return none;

        _T *fromIt = _begin + (at + amount);
        _T *toIt = _begin + at;

        const _T *const fromEndCopy = _begin + _length;
        const _T *const fromEndDestruct = _begin + (at + amount + amount);
        const _T *const fromEndAssign = fromEndCopy < fromEndDestruct ? fromEndCopy : fromEndDestruct;

        for (; fromIt < fromEndAssign; ++fromIt, ++toIt)
            *toIt = static_cast<_T &&>(*fromIt);

        for (; fromIt < fromEndCopy; ++fromIt, ++toIt)
            ::new (toIt) _T(static_cast<_T &&>(*fromIt));

        for (; fromIt < fromEndDestruct; ++fromIt, ++toIt)
            toIt->~_T();

        return none;
    }

    void clear() noexcept
    {
        const _T *const itEnd = _begin + _length;
        for (_T *it = _begin; it < itEnd; ++it)
            it->~_T();

        _length = 0;
    }

    /// #### Errors:
    /// - `ErrorTypes::HeapOverflow`
    Result<void> setCapacity(const size_t capacity) noexcept
    {
        if (_capacity == capacity)
            return none;

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
        else if (_begin == nullptr)
        {
            _begin = static_cast<_T *>(malloc(capacity * sizeof(_T)));

            if (_begin == nullptr)
                return bad ErrorTypes::HeapOverflow;

            _capacity = capacity;
        }
        else
        {
            _T *const newBegin = static_cast<_T *>(realloc(_begin, capacity * sizeof(_T)));

            if (newBegin == nullptr)
                return bad ErrorTypes::HeapOverflow;

            if (newBegin == _begin)
            {
                const _T *fromEndDestruct = _begin + _length;

                for (_T *it = _begin + capacity; it < fromEndDestruct; ++it)
                    it->~_T();
            }
            else
            {
                _T *fromIt = _begin;
                _T *toIt = newBegin;

                const _T *fromEndCopy;
                const _T *fromEndDestruct;

                if (capacity < _length)
                {
                    fromEndCopy = _begin + capacity;
                    fromEndDestruct = _begin + _length;
                    _length = capacity;
                }
                else
                {
                    fromEndCopy = _begin + _length;
                    fromEndDestruct = fromEndCopy;
                }

                for (; fromIt < fromEndCopy; ++fromIt, ++toIt)
                {
                    ::new (toIt) _T(*fromIt);
                    fromIt->~_T();
                }

                for (; fromIt < fromEndDestruct; ++fromIt)
                    fromIt->~_T();

                free(_begin);
                _begin = newBegin;
            }

            _capacity = capacity;
        }

        return none;
    }

    [[nodiscard]] _T *begin() { return _begin; }
    [[nodiscard]] const _T *begin() const { return _begin; }
    [[nodiscard]] const _T *end() const { return _begin + _length; }

    [[nodiscard]] _T *enumerate() { return _begin; }
    [[nodiscard]] const _T *enumerate() const { return _begin; }

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
                ::new (toIt) _T(static_cast<const _T &>(*fromIt));

            _capacity = other._length;
            _length = other._length;
        }
        else
        {
            _T *fromIt = other._begin;
            _T *toIt = _begin;

            const _T *const fromEndCopy = other._begin + other._length;
            const _T *const fromEndDestruct = other._begin + _length;
            const _T *const fromEndAssign = fromEndCopy < fromEndDestruct ? fromEndCopy : fromEndDestruct;

            for (; fromIt < fromEndAssign; ++fromIt, ++toIt)
                *toIt = static_cast<const _T &>(*fromIt);

            for (; fromIt < fromEndCopy; ++fromIt, ++toIt)
                ::new (toIt) _T(static_cast<const _T &>(*fromIt));

            for (; fromIt < fromEndDestruct; ++fromIt, ++toIt)
                toIt->~_T();

            _length = other._length;
        }
    }
    List<_T> &operator=(List<_T> &&other) &noexcept
    {
        if (_begin != nullptr)
            free(_begin);

        _length = other._length;
        _capacity = other._capacity;
        _begin = other._begin;

        other._length = 0;
        other._capacity = 0;
        other._begin = nullptr;
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<_T &> at(size_t index) noexcept
    {
        if (index >= _length)
            return bad ErrorTypes::IndexOutOfRange;

        return _begin[index];
    }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<const _T &> at(size_t index) const noexcept
    {
        if (index >= _length)
            return bad ErrorTypes::IndexOutOfRange;

        return _begin[index];
    }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<_T &> at(Index index) noexcept
    {
        Result<size_t> actualIndex = index.actualIndex(_length);

        if (!actualIndex)
            return ~actualIndex;

        return _begin[*actualIndex];
    }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<const _T &> at(Index index) const noexcept
    {
        Result<size_t> actualIndex = index.actualIndex(_length);

        if (!actualIndex)
            return ~actualIndex;

        return _begin[actualIndex];
    }

    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] _T &operator[](size_t index) noexcept
    {
        if (index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return _begin[index];
    }
    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] const _T &operator[](size_t index) const noexcept
    {
        if (index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return _begin[index];
    }
    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] _T &operator[](Index index) noexcept
    {
        if (index.index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return index.fromEnd ? _begin[_length - index.index - 1] : _begin[index.index];
    }
    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] const _T &operator[](Index index) const noexcept
    {
        if (index.index >= _length)
            fail(ErrorTypes::IndexOutOfRange);

        return index.fromEnd ? _begin[_length - index.index - 1] : _begin[index.index];
    }

private:
    inline Result<void> _prepareToAdd(const size_t amount) noexcept
    {
        if (amount == 0)
            return none;

        const size_t newLength = _length + amount;
        if (newLength > _capacity)
        {
            size_t newCapacity = _capacity == 0 ? 1 : _capacity;
            while (newCapacity < newLength)
                newCapacity *= 2;
            if (newCapacity < 4)
                newCapacity = 4;

            _T *const newBegin = static_cast<_T *>(realloc(_begin, newCapacity * sizeof(_T)));

            if (newBegin == nullptr)
                return bad ErrorTypes::HeapOverflow;

            if (newBegin != _begin)
            {
                _T *fromIt = _begin;
                _T *toIt = newBegin;

                const _T *const fromItEnd = fromIt + _length;

                for (; fromIt < fromItEnd; ++fromIt, ++toIt)
                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));

                free(_begin);
                _begin = newBegin;
            }

            _capacity = newCapacity;
        }

        return none;
    }

    constexpr inline void _noAllocAdd() noexcept { }
    template<typename... _TRest>
    inline void _noAllocAdd(const _T &first, const _TRest &...rest) noexcept
    {
        _placeNewWithVariadic(_begin + _length, static_cast<const _T &>(first), static_cast<const _TRest &>(rest)...);
        _length += 1 + sizeof...(rest);
    }
    template<typename... _TRest>
    inline void _noAllocAdd(_T &&first, _TRest &&...rest) noexcept
    {
        _placeNewWithVariadic(_begin + _length, static_cast<_T &&>(first), static_cast<_TRest &&>(rest)...);
        _length += 1 + sizeof...(rest);
    }

    inline Result<void> _prepareToInsert(const size_t at, const size_t amount) noexcept
    {
        if (amount == 0)
            return none;

        const size_t newLength = _length + amount;
        if (newLength > _capacity)
        {
            size_t newCapacity = _capacity == 0 ? 1 : _capacity;
            while (newCapacity < newLength)
                newCapacity *= 2;
            if (newCapacity < 4)
                newCapacity = 4;

            _T *const newBegin = static_cast<_T *>(realloc(_begin, newCapacity * sizeof(_T)));

            if (newBegin == nullptr)
                return bad ErrorTypes::HeapOverflow;

            if (newBegin != _begin)
            {
                _T *fromIt = _begin;
                _T *toIt = newBegin;

                const _T *const fromItStationaryEnd = _begin + at;

                for (; fromIt < fromItStationaryEnd; ++fromIt, ++toIt)
                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));

                fromIt = _begin + (_length - 1);
                toIt = newBegin + (newLength - 1);

                const _T *const fromItMovedStart = fromItStationaryEnd;

                for (; fromIt >= fromItMovedStart; --fromIt, --toIt)
                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));

                free(_begin);
                _begin = newBegin;
            }
            else
            {
                _T *fromIt = _begin + (_length - 1);
                _T *toIt = newBegin + (newLength - 1);

                const _T *const fromItMovedStart = _begin + at;

                for (; fromIt >= fromItMovedStart; --fromIt, --toIt)
                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));
            }

            _capacity = newCapacity;
        }

        return none;
    }

    constexpr inline void _noAllocInsert(const size_t) { }
    template<typename... _TRest>
    inline void _noAllocInsert(const size_t at, const _T &first, const _TRest &...rest)
    {
        _placeNewWithVariadic(_begin + at, static_cast<const _T &>(first), static_cast<const _TRest &>(rest)...);
        _length += 1 + sizeof...(rest);
    }
    template<typename... _TRest>
    inline void _noAllocInsert(const size_t at, _T &&first, _TRest &&...rest)
    {
        _placeNewWithVariadic(_begin + at, static_cast<_T &&>(first), static_cast<_TRest &&>(rest)...);
        _length += 1 + sizeof...(rest);
    }

    constexpr inline static void _placeNewWithVariadic(_T *const) noexcept { }
    template<typename... _TRest>
    inline static void _placeNewWithVariadic(_T *const at, const _T &first, const _TRest &...rest) noexcept
    {
        ::new (at) _T(static_cast<const _T &>(first));
        _placeNewWithVariadic(at + 1, static_cast<const _TRest &>(rest)...);
    }
    template<typename... _TRest>
    inline static void _placeNewWithVariadic(_T *const at, _T &&first, _TRest &&...rest) noexcept
    {
        ::new (at) _T(static_cast<_T &&>(first));
        _placeNewWithVariadic(at + 1, static_cast<_TRest &&>(rest)...);
    }
};

#endif