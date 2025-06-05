#ifndef index_h
#define index_h

#include <stddef.h>
#include "./result.h"

struct Index;

using Extent = Index;

struct Index
{
public:
    size_t index;
    bool fromEnd;

public:
    constexpr Index(const size_t index) noexcept : index(index), fromEnd(false) { }
    constexpr Index(const size_t index, const bool fromEnd) noexcept : index(index), fromEnd(fromEnd) { }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    Result<size_t> actualIndex(const size_t length) const noexcept
    {
        if (index >= length)
            return bad ErrorTypes::IndexOutOfRange;

        return fromEnd ? length - index - 1 : index;
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<size_t> actualBegin(const Index startIndex, const size_t length) const noexcept
    {
        const Result<size_t> actualStartIndex = startIndex.actualIndex(length);
        if (!actualStartIndex)
            return actualStartIndex;

        return actualBegin(*actualStartIndex, length);
    }
    /// #### Errors:
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<size_t> actualBegin(const size_t startIndex, const size_t length) const noexcept
    {
        if (!fromEnd)
            return startIndex;
        else
        {
            if (index > startIndex)
                return bad ErrorTypes::ArgumentOutOfRange;

            return startIndex - index;
        }
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<size_t> actualEnd(const Index startIndex, const size_t length) const noexcept
    {
        const Result<size_t> actualStartIndex = startIndex.actualIndex(length);
        if (!actualStartIndex)
            return actualStartIndex;

        return actualEnd(*actualStartIndex, length);
    }
    /// #### Errors:
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<size_t> actualEnd(const size_t startIndex, const size_t length) const noexcept
    {
        if (fromEnd)
            return startIndex + 1;
        else
        {
            if (index >= length)
                return bad ErrorTypes::ArgumentOutOfRange;

            return startIndex + index + 1;
        }
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<size_t> actualLength(const Index startIndex, const size_t length) const noexcept
    {
        const Result<size_t> actualStartIndex = startIndex.actualIndex(length);
        if (!actualStartIndex)
            return actualStartIndex;

        return actualLength(*actualStartIndex, length);
    }
    /// #### Errors:
    /// - `ErrorTypes::ArgumentOutOfRange`
    Result<size_t> actualLength(const size_t startIndex, const size_t length) const noexcept
    {
        if (fromEnd)
        {
            if (index >= startIndex)
                return bad ErrorTypes::ArgumentOutOfRange;
        }
        else
        {
            if (index + startIndex > length)
                return bad ErrorTypes::ArgumentOutOfRange;
        }

        return index;
    }

    /// #### Errors:
    /// - `ErrorTypes::IndexOutOfRange`
    /// - `ErrorTypes::ArgumentOutOfRange`
    static Result<void> makeBeginEnd(Index &atIntoBegin, Extent &amountIntoEnd, const size_t length)
    {
        if (atIntoBegin.index >= length)
            return bad ErrorTypes::IndexOutOfRange;

        size_t begin = atIntoBegin.fromEnd ? atIntoBegin.index : length - atIntoBegin.index - 1;
        size_t end;

        if (amountIntoEnd.fromEnd)
        {
            if (atIntoBegin.index > begin)
                return bad ErrorTypes::ArgumentOutOfRange;

            end = begin + 1;
            begin = end - amountIntoEnd.index;
        }
        else
        {
            if (begin + amountIntoEnd.index > length)
                return bad ErrorTypes::ArgumentOutOfRange;

            end = begin + amountIntoEnd.index;
        }

        atIntoBegin.fromEnd = false;
        atIntoBegin.index = begin;
        amountIntoEnd.fromEnd = false;
        amountIntoEnd.index = end;
        return none;
    }
};

constexpr Index operator""_begin(const unsigned long long i) { return Index(i, false); }
constexpr Index operator""_end(const unsigned long long i) { return Index(i, true); }

#endif