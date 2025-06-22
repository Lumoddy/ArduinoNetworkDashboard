#ifndef sorted_list_h
#define sorted_list_h

#include <stdlib.h>
#include "./result.h"
#include "./index.h"

using Sign = int8_t;

template<typename _T>
constexpr static Sign compareByOperator(const _T &lhs, const _T &rhs) { return lhs == rhs ? 0 : lhs < rhs ? -1 : 1; }

template<typename _T>
constexpr static Sign compareValueByOperator(const _T lhs, const _T rhs) { return lhs == rhs ? 0 : lhs < rhs ? -1 : 1; }

template<typename _T>
static Sign compareByBinary(const _T &lhs, const _T &rhs)
{
    const uint8_t *lhsIt = static_cast<const uint8_t *>(static_cast<const void *>(&lhs));
    const uint8_t *rhsIt = static_cast<const uint8_t *>(static_cast<const void *>(&rhs));
    const uint8_t *const lhsEnd = lhsIt + sizeof(_T);

    for (; lhsIt < lhsEnd; ++lhsIt, ++rhsIt)
    {
        if (*lhsIt == *rhsIt)
            continue;

        return *lhsIt < *rhsIt ? -1 : 1;
    }

    return 0;
}

template<typename _T>
struct DefaultCompare { static Sign compare(const _T &lhs, const _T &rhs) { return compareByBinary(lhs, rhs); } };
template<>
struct DefaultCompare<uint8_t> { static Sign compare(const uint8_t lhs, const uint8_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<int8_t> { static Sign compare(const int8_t lhs, const int8_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<uint16_t> { static Sign compare(const uint16_t lhs, const uint16_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<int16_t> { static Sign compare(const int16_t lhs, const int16_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<uint32_t> { static Sign compare(const uint32_t lhs, const uint32_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<int32_t> { static Sign compare(const int32_t lhs, const int32_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<uint64_t> { static Sign compare(const uint64_t lhs, const uint64_t rhs) { return compareValueByOperator(lhs, rhs); } };
template<>
struct DefaultCompare<int64_t> { static Sign compare(const int64_t lhs, const int64_t rhs) { return compareValueByOperator(lhs, rhs); } };

/// #### Requires:
/// - `_T` : Not a reference.
/// - `_TComparer` : Implements `static Sign compare(_T, _T)`.
template<typename _T, typename _TComparer = DefaultCompare<_T>>
class SortedList : private List<_T>
{
public:
    constexpr SortedList() noexcept : List<_T>() { }
    SortedList(const size_t capacity) noexcept : List<_T>(capacity) { }
    SortedList(const SortedList<_T> &original) noexcept : List<_T>(static_cast<const SortedList<_T> &>(original)) { }
    SortedList(SortedList<_T> &&original) noexcept : List<_T>(static_cast<SortedList<_T> &&>(original)) { }

    ~SortedList()
    {
        List<_T>::~List<_T>();
    }

    [[nodiscard]] static SortedList<_T> of() noexcept { return SortedList<_T>(); }
    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    template<typename ..._TRest>
    [[nodiscard]] static SortedList<_T> of(const _T &first, const _TRest &...rest) noexcept
    {
        SortedList<_T> result = SortedList<_T>(1 + sizeof...(rest));
        result.add(static_cast<const _T &>(first), static_cast<const _T &>(rest)...);
        return result;
    }
    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    template<typename ..._TRest>
    [[nodiscard]] static SortedList<_T> of(_T &&first, _TRest &&...rest) noexcept
    {
        SortedList<_T> result = SortedList<_T>(1 + sizeof...(rest));
        result.add(static_cast<_T &&>(first), static_cast<_T &&>(rest)...);
        return result;
    }

    /// #### Requires:
    /// - `_TIterator` : Iterator functionality.
    template<typename _TIterator>
    [[nodiscard]] static SortedList<_T> from(const _TIterator &begin, const _TIterator &end) noexcept
    {
        SortedList<_T> result = SortedList<_T>();
        for (_TIterator it = begin; it < end; ++it)
            static_cast<List<_T>>(result).add(*it);

        _sortArray(static_cast<List<_T> &>(result)._begin, 0, capacity);

        return result;
    }
    /// #### Requires:
    /// - `_TIterator` : Iterator functionality.
    template<typename _TIterator>
    [[nodiscard]] static SortedList<_T> from(const size_t capacity, const _TIterator &begin, const _TIterator &end) noexcept
    {
        SortedList<_T> result = SortedList<_T>(capacity);
        _TIterator it = begin;
        size_t i = 0;
        for (; i < capacity && it < end; ++i, ++it)
            static_cast<List<_T> &>(result).add(*it);

        _sortArray(static_cast<List<_T> &>(result)._begin, 0, capacity);

        return result;
    }

    [[nodiscard]] size_t length() const { return List<_T>::length(); }
    [[nodiscard]] size_t capacity() const { return List<_T>::capacity(); }

    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    /// #### Errors:
    /// - `ErrorTypes::HeapOverflow`
    template<typename... _TRest>
    Result<void> add(const _T &item, const _TRest &...rest) noexcept
    {
        _T sortedParameters[1 + sizeof...(rest)] = { item, rest... };
        _sortArray(&sortedParameters, 0, 1 + sizeof...(rest));

        return _takeFromArrayAndAdd(sortedParameters, 1 + sizeof...(rest));
    }
    /// #### Requires:
    /// - `_TRest...` : Contains only `_T`.
    /// #### Errors:
    /// - `ErrorTypes::HeapOverflow`
    template<typename... _TRest>
    Result<void> add(_T &&item, _TRest &&...rest) noexcept
    {
        _T sortedParameters[1 + sizeof...(rest)] = { item, rest... };
        _sortArray(&sortedParameters, 0, 1 + sizeof...(rest));

        return _takeFromArrayAndAdd(sortedParameters, 1 + sizeof...(rest));
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<void> remove(const Index at, const Extent amount = 1_begin) noexcept { return List<_T>::remove(at, amount); }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<void> remove(const size_t at, const size_t amount = 1) noexcept { return List<_T>::remove(at, amount); }

    /// #### Errors:
    /// - `ErrorTypes::HeapOverflow`
    Result<void> setCapacity(const size_t capacity) noexcept { return List<_T>::setCapacity(capacity); }

    /// #### Errors:
    /// - `ErrorTypes::NotFound`
    Result<size_t> indexOf(const _T &value) const noexcept { return _binarySearch(List<_T>::_begin, 0, List<_T>::_length - 1, value); }

    size_t indexBefore(const _T &value) const noexcept { return _binarySearchBefore(List<_T>::_begin, 0, List<_T>::_length - 1, value); }

    size_t indexAfter(const _T &value) const noexcept { return _binarySearchBefore(List<_T>::_begin, 0, List<_T>::_length - 1, value); }

    [[nodiscard]] _T *begin() { return List<_T>::begin(); }
    [[nodiscard]] const _T *begin() const { return List<_T>::begin(); }
    [[nodiscard]] const _T *end() const { return List<_T>::end(); }

    SortedList<_T> &operator=(const SortedList<_T> &other) &noexcept { return List<_T>::operator=(static_cast<const SortedList<_T> &>(other)); }
    SortedList<_T> &operator=(SortedList<_T> &&other) &noexcept { return List<_T>::operator=(static_cast<SortedList<_T> &&>(other)); }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<_T &> at(size_t index) noexcept { return List<_T>::at(index); }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<const _T &> at(size_t index) const noexcept { return List<_T>::at(index); }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<_T &> at(Index index) noexcept { return List<_T>::at(index); }
    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<const _T &> at(Index index) const noexcept { return List<_T>::at(index); }

    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] _T &operator[](size_t index) noexcept { return List<_T>::operator[](index); }
    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] const _T &operator[](size_t index) const noexcept { return List<_T>::operator[](index); }
    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] _T &operator[](Index index) noexcept { return List<_T>::operator[](index); }
    /// #### Failures:
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] const _T &operator[](Index index) const noexcept { return List<_T>::operator[](index); }

protected:
    inline static void _sortArray(_T *const array, const size_t at, const size_t length)
    {
        if (length < 2)
            return;

        // Quick sort (https://opendsa-server.cs.vt.edu/embed/quicksortAV)

        const size_t pivotIndex = at + (length / 2);
        const size_t lastIndex = at + length - 1;

        _T &pivotValue = array[lastIndex];
        _swap(pivotValue, array[pivotIndex]);

        size_t leftBound = at;
        size_t rightBound = lastIndex - 1;

        while (true)
        {
            leftStep:
            {
                if (_TComparer::compare(array[leftBound], pivotValue) < 0)
                {
                    ++leftBound;
                    goto leftStep;
                }
            }

            rightStep:
            {
                if (leftBound <= rightBound)
                    goto finish;

                if (_TComparer::compare(array[rightBound], pivotValue) >= 0)
                {
                    --rightBound;
                    goto rightStep;
                }
            }

            _swap(array[leftBound], array[rightBound]);
        }

        finish:

        _swap(array[leftBound], pivotValue);

        _sortArray(array, at, leftBound - at);
        _sortArray(array, leftBound + 1, lastIndex - leftBound);
    }

    inline static void _swap(_T &a, _T &b)
    {
        uint8_t c[sizeof(_T)];
        ::new (static_cast<void *>(&c)) _T(static_cast<_T &&>(a));
        a = static_cast<_T &&>(b);
        b = static_cast<_T &&>(*static_cast<_T *>(static_cast<void *>(c)));
    }

    inline Result<void> _takeFromArrayAndAdd(_T *const sortedArray, const size_t arrayLength) noexcept
    {
        if (arrayLength == 0)
            return none;

        const size_t newLength = List<_T>::_length + arrayLength;
        if (newLength > List<_T>::_capacity)
        {
            size_t newCapacity = List<_T>::_capacity == 0 ? 1 : List<_T>::_capacity;
            while (newCapacity < newLength)
                newCapacity *= 2;
            if (newCapacity < 4)
                newCapacity = 4;

            _T *const newBegin = static_cast<_T *>(realloc(List<_T>::_begin, newCapacity * sizeof(_T)));

            if (newBegin == nullptr)
                return bad ErrorTypes::HeapOverflow;

            if (newBegin != List<_T>::_begin)
            {
                _T *arrayIt = sortedArray;
                const _T *const arrayEnd = sortedArray + arrayLength;

                _T *fromIt = List<_T>::_begin;
                _T *toIt = newBegin;

                const _T *const fromItEnd = fromIt + List<_T>::_length;

                for (; fromIt < fromItEnd; ++fromIt, ++toIt)
                {
                    for (; arrayIt < arrayEnd && _TComparer::compare(*arrayIt, *fromIt) < 0; ++arrayIt, ++toIt)
                        ::new (toIt) _T(static_cast<_T &&>(*arrayIt));

                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));
                }

                free(List<_T>::_begin);
                List<_T>::_begin = newBegin;
            }
            else
            {
                const size_t deltaCapacity = newCapacity - List<_T>::_capacity;

                _T *fromIt = List<_T>::_begin + (List<_T>::_length - 1);
                _T *toIt = List<_T>::_begin + (List<_T>::_length - 1 - deltaCapacity);

                for (; fromIt >= List<_T>::_begin; --fromIt, --toIt)
                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));

                _T *arrayIt = sortedArray;
                const _T *const arrayEnd = sortedArray + arrayLength;

                const _T *const fromItEnd = List<_T>::_begin + (List<_T>::_length - 1);

                fromIt = List<_T>::_begin + deltaCapacity;
                toIt = List<_T>::_begin;

                for (; fromIt < fromItEnd; ++fromIt, ++toIt)
                {
                    for (; arrayIt < arrayEnd && _TComparer::compare(*arrayIt, *fromIt) < 0; ++arrayIt, ++toIt)
                        ::new (toIt) _T(static_cast<_T &&>(*arrayIt));

                    ::new (toIt) _T(static_cast<_T &&>(*fromIt));
                }
            }

            List<_T>::_capacity = newCapacity;
        }

        return none;
    }

    inline static size_t _binarySearchBefore(const _T *const array, const size_t low, const size_t high, const _T &search)
    {
        if (low == high)
            return low;

        const size_t middle = low + ((high - low) / 2);
        if (_TComparer::compare(search, array[middle]) > 0)
            return _binarySearchBefore(array, middle, high, search);
        else
            return _binarySearchBefore(array, low, middle - 1, search);
    }

    inline static size_t _binarySearchAfter(const _T *const array, const size_t low, const size_t high, const _T &search)
    {
        if (low == high)
            return low + 1;

        const size_t middle = low + ((high - low) / 2);
        if (_TComparer::compare(search, array[middle]) < 0)
            return _binarySearchBefore(array, low, middle, search);
        else
            return _binarySearchBefore(array, middle + 1, high, search);
    }

    inline static Result<size_t> _binarySearch(const _T *const array, const size_t low, const size_t high, const _T &search)
    {
        if (low == high)
            return _TComparer::compare(search, array[low]) == 0 ? low : ErrorTypes::NotFound;

        const size_t middle = low + ((high - low) / 2);
        const Sign pivotComparison = _TComparer::compare(search, array[middle]);
        if (pivotComparison == 0)
            return middle;
        else if (pivotComparison > 0)
            return _binarySearch(array, middle + 1, high, search);
        else
            return _binarySearch(array, low, middle - 1, search);
    }
};

#endif